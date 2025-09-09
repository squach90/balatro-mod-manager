import { invoke } from "@tauri-apps/api/core";

// Minimal typed map for Tauri commands used in the start flow.
export type InvokeMap = {
  find_steam_balatro: { args: undefined; result: string[] };
  check_custom_balatro: { args: { path: string }; result: boolean };
  check_existing_installation: { args: undefined; result: string | null };
};

type Args<C extends keyof InvokeMap> = InvokeMap[C]["args"];
type Result<C extends keyof InvokeMap> = InvokeMap[C]["result"];

export function invokeTyped<C extends keyof InvokeMap>(
  cmd: C,
  args?: Args<C>,
): Promise<Result<C>> {
  return invoke<Result<C>>(
    cmd,
    args as unknown as Record<string, unknown> | undefined,
  );
}

export function invokeWithTimeout<C extends keyof InvokeMap>(
  cmd: C,
  args?: Args<C>,
  ms = 5000,
): Promise<Result<C>> {
  return new Promise<Result<C>>((resolve, reject) => {
    const t = setTimeout(
      () => reject(new Error(`invoke-timeout:${String(cmd)}`)),
      ms,
    );
    invoke<Result<C>>(
      cmd,
      args as unknown as Record<string, unknown> | undefined,
    )
      .then((v) => {
        clearTimeout(t);
        resolve(v);
      })
      .catch((e) => {
        clearTimeout(t);
        reject(e);
      });
  });
}
