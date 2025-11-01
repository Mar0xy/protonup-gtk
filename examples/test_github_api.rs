use serde::Deserialize;
use reqwest::Client;
use anyhow::Result;

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing GitHub API integration for ProtonUp-GTK...\n");

    let client = Client::builder()
        .user_agent("ProtonUp-GTK/0.1.0")
        .build()?;

    // Test GE-Proton
    println!("Fetching GE-Proton latest release...");
    let url = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(release) = response.json::<GitHubRelease>().await {
                    println!("✓ GE-Proton: {} - {}", release.tag_name, release.name);
                    if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.gz")) {
                        println!("  Download: {}", asset.browser_download_url);
                    }
                } else {
                    println!("✗ Failed to parse GE-Proton release data");
                }
            } else {
                println!("✗ HTTP error: {}", response.status());
            }
        }
        Err(e) => println!("✗ Request failed: {}", e),
    }

    println!();

    // Test Wine-GE
    println!("Fetching Wine-GE latest release...");
    let url = "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(release) = response.json::<GitHubRelease>().await {
                    println!("✓ Wine-GE: {} - {}", release.tag_name, release.name);
                    if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.xz")) {
                        println!("  Download: {}", asset.browser_download_url);
                    }
                } else {
                    println!("✗ Failed to parse Wine-GE release data");
                }
            } else {
                println!("✗ HTTP error: {}", response.status());
            }
        }
        Err(e) => println!("✗ Request failed: {}", e),
    }

    println!();

    // Test Luxtorpeda
    println!("Fetching Luxtorpeda latest release...");
    let url = "https://api.github.com/repos/luxtorpeda-dev/luxtorpeda/releases/latest";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(release) = response.json::<GitHubRelease>().await {
                    println!("✓ Luxtorpeda: {} - {}", release.tag_name, release.name);
                    if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.xz")) {
                        println!("  Download: {}", asset.browser_download_url);
                    }
                } else {
                    println!("✗ Failed to parse Luxtorpeda release data");
                }
            } else {
                println!("✗ HTTP error: {}", response.status());
            }
        }
        Err(e) => println!("✗ Request failed: {}", e),
    }

    println!();

    // Test Spritz-Wine
    println!("Fetching Spritz-Wine latest release...");
    let url = "https://api.github.com/repos/NelloKudo/Wine-Builds/releases/latest";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(release) = response.json::<GitHubRelease>().await {
                    println!("✓ Spritz-Wine: {} - {}", release.tag_name, release.name);
                    if let Some(asset) = release.assets.iter().find(|a| a.name.contains("Spritz-Wine") && a.name.ends_with(".tar.xz")) {
                        println!("  Download: {}", asset.browser_download_url);
                    } else {
                        println!("  (No Spritz-Wine .tar.xz asset found)");
                    }
                } else {
                    println!("✗ Failed to parse Spritz-Wine release data");
                }
            } else {
                println!("✗ HTTP error: {}", response.status());
            }
        }
        Err(e) => println!("✗ Request failed: {}", e),
    }

    println!();

    // Test dwproton
    println!("Fetching dwproton latest release...");
    let url = "https://dawn.wine/api/v1/repos/dawn-winery/dwproton/releases/latest";
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(release) = response.json::<GitHubRelease>().await {
                    println!("✓ dwproton: {} - {}", release.tag_name, release.name);
                    if let Some(asset) = release.assets.iter().find(|a| a.name.ends_with(".tar.gz")) {
                        println!("  Download: {}", asset.browser_download_url);
                    }
                } else {
                    println!("✗ Failed to parse dwproton release data");
                }
            } else {
                println!("✗ HTTP error: {}", response.status());
            }
        }
        Err(e) => println!("✗ Request failed: {}", e),
    }

    println!("\nGitHub API integration test complete!");
    Ok(())
}
