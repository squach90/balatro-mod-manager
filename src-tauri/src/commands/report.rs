use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use std::fs;
use sysinfo::System;

const REPORT_URL: &str = "https://balatro-mod-manager-reports.dasguney.com/report";

#[derive(Serialize)]
struct ReportPayload<'a> {
    title: &'a str,
    description: &'a str,
    mm_version: &'a str,
    os: String,
    arch: String,
    cpu: String,
    gpu: String,
    ram: String,
    log_b64: String,
    log_filename: String,
}

/// Submit an issue report to the Jimbo bot server.
#[tauri::command]
pub async fn submit_report(
    title: String,
    description: String,
    mm_version: String,
) -> Result<(), String> {
    // Collect system info
    let (cpu, ram_str) = get_cpu_and_ram();
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let gpu = detect_gpu().unwrap_or_else(|| "Unknown".to_string());

    // Get latest log
    let (log_filename, log_text) =
        latest_log().unwrap_or_else(|| ("bmm.log".to_string(), String::from("No logs found.")));
    let log_b64 = STANDARD.encode(log_text.as_bytes());

    let payload = ReportPayload {
        title: &title,
        description: &description,
        mm_version: &mm_version,
        os,
        arch,
        cpu,
        gpu,
        ram: ram_str,
        log_b64,
        log_filename,
    };

    // Send
    let client = reqwest::Client::new();
    let resp = client
        .post(REPORT_URL)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Report request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!(
            "Report server error: HTTP {}{}",
            status,
            if text.is_empty() {
                String::new()
            } else {
                format!(" - {}", text)
            }
        ));
    }
    Ok(())
}

/// Returns the latest log's filename and text, if any.
#[tauri::command]
pub async fn get_latest_log() -> Result<(String, String), String> {
    latest_log().ok_or_else(|| "No logs found.".to_string())
}

fn latest_log() -> Option<(String, String)> {
    let dir = dirs::config_dir()?.join("Balatro").join("logs");
    let entries = fs::read_dir(&dir).ok()?;
    let mut files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.extension().is_some_and(|ext| ext == "log")
                && p.file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with("bmm_"))
        })
        .collect();

    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
    let path = files.last()?.path();
    let filename = path.file_name()?.to_string_lossy().to_string();
    let text = fs::read_to_string(&path).ok()?;
    Some((filename, text))
}

fn get_cpu_and_ram() -> (String, String) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu = detect_cpu();

    // Try to produce a sane GB value despite sysinfo changes over versions
    let tm = sys.total_memory();
    let gib_from_bytes = (tm as f64) / (1024.0 * 1024.0 * 1024.0);
    let gib_from_kib = (tm as f64) / (1024.0 * 1024.0);

    let gb = if (1.0..=1024.0).contains(&gib_from_bytes) {
        gib_from_bytes
    } else if (1.0..=1024.0).contains(&gib_from_kib) {
        gib_from_kib
    } else {
        // Fallback: prefer bytes path
        gib_from_bytes
    };
    let ram_str = format!("{}GB", (gb.round() as u64).max(1));
    (cpu, ram_str)
}

// Best-effort GPU detection; returns None if not supported
#[cfg(target_os = "macos")]
fn detect_gpu() -> Option<String> {
    use std::process::Command;
    let out = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .arg("-detailLevel")
        .arg("mini")
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout);
    for line in s.lines() {
        let l = line.trim();
        if l.starts_with("Chipset Model:") || l.starts_with("Chipset:") {
            return Some(
                l.replace("Chipset Model:", "")
                    .replace("Chipset:", "")
                    .trim()
                    .to_string(),
            );
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn detect_gpu() -> Option<String> {
    // Using wmic is deprecated on newer Windows; keep it best-effort
    use std::process::Command;
    let out = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "name"])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout);
    let mut lines = s.lines().skip(1).filter(|l| !l.trim().is_empty());
    lines.next().map(|l| l.trim().to_string())
}

#[cfg(target_os = "linux")]
fn detect_gpu() -> Option<String> {
    use std::process::Command;
    let out = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -Ei 'vga|3d|display' | head -n1")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    Some(s.trim().to_string())
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn detect_gpu() -> Option<String> {
    None
}

// Platform-specific CPU detection (best effort)
fn detect_cpu() -> String {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("wmic").args(["cpu", "get", "name"]).output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout);
                if let Some(name) = s.lines().skip(1).find(|l| !l.trim().is_empty()) {
                    return name.trim().to_string();
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg("lscpu | grep 'Model name' | cut -d: -f2")
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
        if let Ok(out) = Command::new("sh")
            .arg("-c")
            .arg("grep -m1 'model name' /proc/cpuinfo | cut -d: -f2")
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    "Unknown".to_string()
}
