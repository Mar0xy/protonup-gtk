use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityTool {
    pub name: String,
    pub description: String,
    pub launcher: Launcher,
    pub download_url: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Launcher {
    Steam,
    Lutris,
}

impl std::fmt::Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Launcher::Steam => write!(f, "Steam"),
            Launcher::Lutris => write!(f, "Lutris"),
        }
    }
}

// GitHub API response structures
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

pub struct ToolManager {
    tools: Vec<CompatibilityTool>,
    client: Client,
}

impl ToolManager {
    pub fn new() -> Self {
        Self { 
            tools: Vec::new(),
            client: Client::builder()
                .user_agent("ProtonUp-GTK/0.1.0")
                .build()
                .unwrap(),
        }
    }

    pub async fn fetch_available_tools(&mut self) -> Result<Vec<CompatibilityTool>> {
        let mut tools = Vec::new();

        // Fetch GE-Proton releases
        if let Ok(ge_proton) = self.fetch_ge_proton_latest().await {
            tools.push(ge_proton);
        }

        // Fetch Wine-GE releases
        if let Ok(wine_ge) = self.fetch_wine_ge_latest().await {
            tools.push(wine_ge);
        }

        // Fetch Luxtorpeda releases
        if let Ok(luxtorpeda) = self.fetch_luxtorpeda_latest().await {
            tools.push(luxtorpeda);
        }

        // Fetch Spritz-Wine releases
        if let Ok(spritz_wine) = self.fetch_spritz_wine_latest().await {
            tools.push(spritz_wine);
        }

        // Fetch dwproton releases
        if let Ok(dwproton) = self.fetch_dwproton_latest().await {
            tools.push(dwproton);
        }

        self.tools = tools.clone();
        Ok(tools)
    }

    async fn fetch_ge_proton_latest(&self) -> Result<CompatibilityTool> {
        let url = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";
        let release: GitHubRelease = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        // Find the tar.gz asset
        let asset = release.assets
            .iter()
            .find(|a| a.name.ends_with(".tar.gz"))
            .ok_or_else(|| anyhow::anyhow!("No .tar.gz asset found for GE-Proton"))?;

        Ok(CompatibilityTool {
            name: "GE-Proton".to_string(),
            description: "Proton compatibility tool with additional fixes".to_string(),
            launcher: Launcher::Steam,
            download_url: asset.browser_download_url.clone(),
            version: release.tag_name.clone(),
        })
    }

    async fn fetch_wine_ge_latest(&self) -> Result<CompatibilityTool> {
        let url = "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest";
        let release: GitHubRelease = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        // Find the tar.xz asset
        let asset = release.assets
            .iter()
            .find(|a| a.name.ends_with(".tar.xz"))
            .ok_or_else(|| anyhow::anyhow!("No .tar.xz asset found for Wine-GE"))?;

        Ok(CompatibilityTool {
            name: "Wine-GE".to_string(),
            description: "Wine with additional game fixes".to_string(),
            launcher: Launcher::Lutris,
            download_url: asset.browser_download_url.clone(),
            version: release.tag_name.clone(),
        })
    }

    async fn fetch_luxtorpeda_latest(&self) -> Result<CompatibilityTool> {
        let url = "https://api.github.com/repos/luxtorpeda-dev/luxtorpeda/releases/latest";
        let release: GitHubRelease = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        // Find the tar.xz asset
        let asset = release.assets
            .iter()
            .find(|a| a.name.ends_with(".tar.xz"))
            .ok_or_else(|| anyhow::anyhow!("No .tar.xz asset found for Luxtorpeda"))?;

        Ok(CompatibilityTool {
            name: "Luxtorpeda".to_string(),
            description: "Steam Play compatibility tool for native Linux games".to_string(),
            launcher: Launcher::Steam,
            download_url: asset.browser_download_url.clone(),
            version: release.tag_name.clone(),
        })
    }

    async fn fetch_spritz_wine_latest(&self) -> Result<CompatibilityTool> {
        let url = "https://api.github.com/repos/NelloKudo/Wine-Builds/releases/latest";
        let release: GitHubRelease = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        // Find a Spritz-Wine tar.xz asset (they have various builds)
        let asset = release.assets
            .iter()
            .find(|a| a.name.contains("Spritz-Wine") && a.name.ends_with(".tar.xz"))
            .ok_or_else(|| anyhow::anyhow!("No Spritz-Wine .tar.xz asset found"))?;

        Ok(CompatibilityTool {
            name: "Spritz-Wine".to_string(),
            description: "Wine builds optimized for gaming performance".to_string(),
            launcher: Launcher::Lutris,
            download_url: asset.browser_download_url.clone(),
            version: release.tag_name.clone(),
        })
    }

    async fn fetch_dwproton_latest(&self) -> Result<CompatibilityTool> {
        // Forgejo/Gitea API is similar to GitHub API
        let url = "https://dawn.wine/api/v1/repos/dawn-winery/dwproton/releases/latest";
        let release: GitHubRelease = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        // Find the tar.gz asset
        let asset = release.assets
            .iter()
            .find(|a| a.name.ends_with(".tar.gz"))
            .ok_or_else(|| anyhow::anyhow!("No .tar.gz asset found for dwproton"))?;

        Ok(CompatibilityTool {
            name: "dwproton".to_string(),
            description: "Dawn Wine Proton - Proton fork with improvements".to_string(),
            launcher: Launcher::Steam,
            download_url: asset.browser_download_url.clone(),
            version: release.tag_name.clone(),
        })
    }

    pub fn get_tools(&self) -> &[CompatibilityTool] {
        &self.tools
    }

    pub fn get_install_path(&self, launcher: &Launcher) -> Result<std::path::PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        
        let path = match launcher {
            Launcher::Steam => {
                home_dir.join(".steam/root/compatibilitytools.d")
            }
            Launcher::Lutris => {
                home_dir.join(".local/share/lutris/runners/wine")
            }
        };
        
        Ok(path)
    }
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_tools() {
        let mut manager = ToolManager::new();
        let result = manager.fetch_available_tools().await;
        
        // We expect this to succeed or gracefully handle errors
        match result {
            Ok(tools) => {
                // Should have at least one tool if API is available
                if !tools.is_empty() {
                    println!("Fetched {} tools", tools.len());
                    for tool in tools {
                        println!("- {} ({}): {}", tool.name, tool.version, tool.download_url);
                    }
                }
            }
            Err(e) => {
                println!("Error fetching tools (this may be expected if offline): {}", e);
            }
        }
    }

    #[test]
    fn test_get_install_path() {
        let manager = ToolManager::new();
        
        let steam_path = manager.get_install_path(&Launcher::Steam);
        assert!(steam_path.is_ok());
        
        let lutris_path = manager.get_install_path(&Launcher::Lutris);
        assert!(lutris_path.is_ok());
    }
}
