use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use super::Launcher;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("com.github.Mar0xy.ProtonUpGTK");
        
        std::fs::create_dir_all(&config_dir)?;
        let db_path = config_dir.join("settings.db");
        
        let conn = Connection::open(db_path)?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS installed_runners (
                version TEXT NOT NULL,
                launcher TEXT NOT NULL,
                PRIMARY KEY (version, launcher)
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    // Settings methods
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE key = ?")?;
        let mut rows = stmt.query(params![key])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
    
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }
    
    pub fn delete_setting(&self, key: &str) -> Result<()> {
        self.conn.execute("DELETE FROM settings WHERE key = ?", params![key])?;
        Ok(())
    }
    
    // Path methods
    pub fn get_steam_path(&self) -> Result<Option<PathBuf>> {
        if let Some(path_str) = self.get_setting("steam_path")? {
            Ok(Some(PathBuf::from(path_str)))
        } else {
            Ok(None)
        }
    }
    
    pub fn set_steam_path(&self, path: Option<&PathBuf>) -> Result<()> {
        if let Some(p) = path {
            if let Some(path_str) = p.to_str() {
                self.set_setting("steam_path", path_str)?;
            }
        } else {
            self.delete_setting("steam_path")?;
        }
        Ok(())
    }
    
    pub fn get_lutris_path(&self) -> Result<Option<PathBuf>> {
        if let Some(path_str) = self.get_setting("lutris_path")? {
            Ok(Some(PathBuf::from(path_str)))
        } else {
            Ok(None)
        }
    }
    
    pub fn set_lutris_path(&self, path: Option<&PathBuf>) -> Result<()> {
        if let Some(p) = path {
            if let Some(path_str) = p.to_str() {
                self.set_setting("lutris_path", path_str)?;
            }
        } else {
            self.delete_setting("lutris_path")?;
        }
        Ok(())
    }
    
    // Installed runners methods
    pub fn add_installed_runner(&self, version: &str, launcher: &Launcher) -> Result<()> {
        let launcher_str = match launcher {
            Launcher::Steam => "Steam",
            Launcher::Lutris => "Lutris",
        };
        
        self.conn.execute(
            "INSERT OR REPLACE INTO installed_runners (version, launcher) VALUES (?, ?)",
            params![version, launcher_str],
        )?;
        Ok(())
    }
    
    pub fn remove_installed_runner(&self, version: &str, launcher: &Launcher) -> Result<()> {
        let launcher_str = match launcher {
            Launcher::Steam => "Steam",
            Launcher::Lutris => "Lutris",
        };
        
        self.conn.execute(
            "DELETE FROM installed_runners WHERE version = ? AND launcher = ?",
            params![version, launcher_str],
        )?;
        Ok(())
    }
    
    pub fn is_runner_installed(&self, version: &str, launcher: &Launcher) -> Result<bool> {
        let launcher_str = match launcher {
            Launcher::Steam => "Steam",
            Launcher::Lutris => "Lutris",
        };
        
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM installed_runners WHERE version = ? AND launcher = ?"
        )?;
        let count: i64 = stmt.query_row(params![version, launcher_str], |row| row.get(0))?;
        
        Ok(count > 0)
    }
    
    pub fn get_installed_runners(&self) -> Result<Vec<(String, Launcher)>> {
        let mut stmt = self.conn.prepare("SELECT version, launcher FROM installed_runners")?;
        let rows = stmt.query_map([], |row| {
            let version: String = row.get(0)?;
            let launcher_str: String = row.get(1)?;
            let launcher = if launcher_str == "Steam" {
                Launcher::Steam
            } else {
                Launcher::Lutris
            };
            Ok((version, launcher))
        })?;
        
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
