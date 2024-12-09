use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_exists = Path::new("settings.db").exists();
        let conn = Connection::open("settings.db")?;

        if !db_exists {
            Self::initialize_database(&conn)?;
        }

        Ok(Database { conn })
    }

    fn initialize_database(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                setting TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_installation_path(&self) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE setting = 'installation_path'")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn remove_installation_path(&self) -> Result<()> {
        self.delete_setting("installation_path")?;
        Ok(())
    }

    pub fn set_installation_path(&self, path: &str) -> Result<()> {
        self.set_setting("installation_path", path)?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE setting = ?1")?;
        let mut rows = stmt.query([key])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (setting, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }

    pub fn delete_setting(&self, key: &str) -> Result<()> {
        self.conn.execute("DELETE FROM settings WHERE setting = ?1", [key])?;
        Ok(())
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;
//
//     #[test]
//     fn test_database_creation() {
//         let _ = fs::remove_file("settings.db");
//         let db = Database::new().unwrap();
//         assert!(Path::new("settings.db").exists());
//
//         // Test installation path
//         assert!(db.get_installation_path().unwrap().is_none());
//         db.set_installation_path("/test/path").unwrap();
//         assert_eq!(db.get_installation_path().unwrap().unwrap(), "/test/path");
//
//         // Test settings
//         assert!(db.get_setting("theme").unwrap().is_none());
//         db.set_setting("theme", "dark").unwrap();
//         assert_eq!(db.get_setting("theme").unwrap().unwrap(), "dark");
//     }
// }
