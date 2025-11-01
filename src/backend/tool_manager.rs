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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct ToolWithVersions {
    pub name: String,
    pub description: String,
    pub default_launcher: Launcher,  // Changed from 'launcher' to 'default_launcher' for clarity
    pub versions: Vec<ToolVersion>,
}

#[derive(Debug, Clone)]
pub struct ToolVersion {
    pub version: String,
    pub download_url: String,
}

pub struct ToolManager {
    tools: Vec<CompatibilityTool>,
    tools_with_versions: Vec<ToolWithVersions>,
    client: Client,
    custom_steam_path: Option<std::path::PathBuf>,
    custom_lutris_path: Option<std::path::PathBuf>,
}

impl ToolManager {
    pub fn new() -> Self {
        Self { 
            tools: Vec::new(),
            tools_with_versions: Vec::new(),
            client: Client::builder()
                .user_agent("ProtonUp-GTK/0.2.0")
                .build()
                .expect("Failed to create HTTP client"),
            custom_steam_path: None,
            custom_lutris_path: None,
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

        // Fetch Spritz-Wine releases
        if let Ok(spritz_wine) = self.fetch_spritz_wine_latest().await {
            tools.push(spritz_wine);
        }

        // Fetch dwproton releases
        if let Ok(dwproton) = self.fetch_dwproton_latest().await {
            tools.push(dwproton);
        }

        self.tools = tools;
        Ok(self.tools.clone())
    }

    pub async fn fetch_tools_with_versions(&mut self) -> Result<Vec<ToolWithVersions>> {
        let mut tools = Vec::new();

        // Fetch GE-Proton releases (last 4)
        if let Ok(ge_proton_versions) = self.fetch_ge_proton_versions(4).await {
            tools.push(ge_proton_versions);
        }

        // Fetch Wine-GE releases (last 4)
        if let Ok(wine_ge_versions) = self.fetch_wine_ge_versions(4).await {
            tools.push(wine_ge_versions);
        }

        // Fetch Spritz-Wine releases (last 4)
        if let Ok(spritz_wine_versions) = self.fetch_spritz_wine_versions(4).await {
            tools.push(spritz_wine_versions);
        }

        // Fetch dwproton releases (last 4)
        if let Ok(dwproton_versions) = self.fetch_dwproton_versions(4).await {
            tools.push(dwproton_versions);
        }

        self.tools_with_versions = tools;
        Ok(self.tools_with_versions.clone())
    }

    async fn fetch_ge_proton_versions(&self, count: usize) -> Result<ToolWithVersions> {
        let url = format!(
            "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases?per_page={}",
            count
        );
        let releases: Vec<GitHubRelease> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let mut versions = Vec::new();
        for release in releases {
            if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.gz")) {
                versions.push(ToolVersion {
                    version: release.tag_name.clone(),
                    download_url: asset.browser_download_url.clone(),
                });
            }
        }

        Ok(ToolWithVersions {
            name: "GE-Proton".to_string(),
            description: "Proton compatibility tool with additional fixes".to_string(),
            default_launcher: Launcher::Steam,
            versions,
        })
    }

    async fn fetch_wine_ge_versions(&self, count: usize) -> Result<ToolWithVersions> {
        let url = format!(
            "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases?per_page={}",
            count
        );
        let releases: Vec<GitHubRelease> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let mut versions = Vec::new();
        for release in releases {
            if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.xz")) {
                versions.push(ToolVersion {
                    version: release.tag_name.clone(),
                    download_url: asset.browser_download_url.clone(),
                });
            }
        }

        Ok(ToolWithVersions {
            name: "Wine-GE".to_string(),
            description: "Wine with additional game fixes".to_string(),
            default_launcher: Launcher::Lutris,
            versions,
        })
    }

    async fn fetch_spritz_wine_versions(&self, count: usize) -> Result<ToolWithVersions> {
        let url = format!(
            "https://api.github.com/repos/NelloKudo/Wine-Builds/releases?per_page={}",
            count
        );
        let releases: Vec<GitHubRelease> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let mut versions = Vec::new();
        for release in releases {
            if let Some(asset) = release.assets.iter()
                .find(|a| a.name.to_lowercase().contains("spritz") && a.name.ends_with(".tar.xz"))
            {
                versions.push(ToolVersion {
                    version: release.tag_name.clone(),
                    download_url: asset.browser_download_url.clone(),
                });
            }
        }

        Ok(ToolWithVersions {
            name: "Spritz-Wine".to_string(),
            description: "Wine builds optimized for gaming performance".to_string(),
            default_launcher: Launcher::Lutris,
            versions,
        })
    }

    async fn fetch_dwproton_versions(&self, count: usize) -> Result<ToolWithVersions> {
        let url = format!(
            "https://dawn.wine/api/v1/repos/dawn-winery/dwproton/releases?per_page={}",
            count
        );
        let releases: Vec<GitHubRelease> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let mut versions = Vec::new();
        for release in releases {
            if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.xz")) {
                versions.push(ToolVersion {
                    version: release.tag_name.clone(),
                    download_url: asset.browser_download_url.clone(),
                });
            }
        }

        Ok(ToolWithVersions {
            name: "dwproton".to_string(),
            description: "Dawn Wine Proton - Proton fork with improvements".to_string(),
            default_launcher: Launcher::Steam,
            versions,
        })
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
            .find(|a| a.name.to_lowercase().contains("spritz") && a.name.ends_with(".tar.xz"))
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

        // Find the tar.xz asset
        let asset = release.assets
            .iter()
            .find(|a| a.name.ends_with(".tar.xz"))
            .ok_or_else(|| anyhow::anyhow!("No .tar.xz asset found for dwproton"))?;

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
        // Check if there's a custom path set
        match launcher {
            Launcher::Steam => {
                if let Some(ref custom_path) = self.custom_steam_path {
                    return Ok(custom_path.clone());
                }
            }
            Launcher::Lutris => {
                if let Some(ref custom_path) = self.custom_lutris_path {
                    return Ok(custom_path.clone());
                }
            }
        }
        
        // Use default paths
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

    pub fn set_steam_path(&mut self, path: Option<std::path::PathBuf>) {
        self.custom_steam_path = path;
    }

    pub fn set_lutris_path(&mut self, path: Option<std::path::PathBuf>) {
        self.custom_lutris_path = path;
    }

    pub fn is_tool_installed(&self, tool_name: &str, launcher: &Launcher) -> bool {
        if let Ok(install_path) = self.get_install_path(launcher) {
            // Check if a directory with the tool name exists
            // GE-Proton versions are typically named like "GE-Proton9-7"
            // We need to check if any directory contains the tool name
            if let Ok(entries) = std::fs::read_dir(&install_path) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(dir_name) = entry.file_name().to_str() {
                                // Check if directory name contains the tool version
                                if dir_name == tool_name || dir_name.contains(tool_name) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
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
