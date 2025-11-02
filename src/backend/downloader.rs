use anyhow::Result;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_file_with_progress<F>(
        &self,
        url: &str,
        destination: &Path,
        mut progress_callback: F,
    ) -> Result<()>
    where
        F: FnMut(f64),
    {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to download file: HTTP {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;

        let mut file = tokio::fs::File::create(destination).await?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            
            if total_size > 0 {
                let progress = (downloaded as f64 / total_size as f64) * 100.0;
                progress_callback(progress);
            }
        }

        file.flush().await?;
        progress_callback(100.0);
        Ok(())
    }

    pub async fn extract_archive_to_specific_dir(&self, archive_path: &Path, extract_to: &Path, target_dir_name: &str) -> Result<()> {
        // Determine archive type by extension
        let extension = archive_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        // First extract to a temporary location
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let temp_extract_dir = std::env::temp_dir().join(format!("protonup-extract-{}", timestamp));
        tokio::fs::create_dir_all(&temp_extract_dir).await?;

        // Extract the archive
        match extension {
            "gz" | "tgz" => self.extract_tar_gz(archive_path, &temp_extract_dir).await?,
            "xz" => self.extract_tar_xz(archive_path, &temp_extract_dir).await?,
            _ => return Err(anyhow::anyhow!("Unsupported archive format: {}", extension)),
        }

        // Find the extracted content (usually a single directory)
        let mut entries = tokio::fs::read_dir(&temp_extract_dir).await?;
        let mut extracted_dir = None;
        
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                extracted_dir = Some(entry.path());
                break;
            }
        }

        let source_dir = extracted_dir.ok_or_else(|| anyhow::anyhow!("No directory found in extracted archive"))?;
        
        // Create the target directory with the specified name
        tokio::fs::create_dir_all(extract_to).await?;
        let target_path = extract_to.join(target_dir_name);
        
        // If target already exists, remove it first
        if target_path.exists() {
            tokio::fs::remove_dir_all(&target_path).await?;
        }
        
        // Try to rename first (fast if same filesystem), fall back to copy if it fails
        match tokio::fs::rename(&source_dir, &target_path).await {
            Ok(_) => {},
            Err(_) => {
                // Rename failed (likely different filesystem), so copy instead
                Self::copy_dir_recursive(&source_dir, &target_path).await?;
                // Remove the source after successful copy
                let _ = tokio::fs::remove_dir_all(&source_dir).await;
            }
        }
        
        // Clean up the temporary extraction directory
        let _ = tokio::fs::remove_dir_all(&temp_extract_dir).await;
        
        Ok(())
    }

    async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
        tokio::fs::create_dir_all(dst).await?;
        
        let mut entries = tokio::fs::read_dir(src).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dst.join(&file_name);
            
            let metadata = tokio::fs::symlink_metadata(&entry_path).await?;
            
            if metadata.is_symlink() {
                // Handle symlinks by reading and recreating them
                let link_target = tokio::fs::read_link(&entry_path).await?;
                #[cfg(unix)]
                tokio::fs::symlink(&link_target, &dest_path).await?;
                #[cfg(windows)]
                {
                    // On Windows, we need to check if the symlink points to a directory or file
                    if tokio::fs::metadata(&entry_path).await.map(|m| m.is_dir()).unwrap_or(false) {
                        tokio::fs::symlink_dir(&link_target, &dest_path).await?;
                    } else {
                        tokio::fs::symlink_file(&link_target, &dest_path).await?;
                    }
                }
            } else if metadata.is_dir() {
                // Recursively copy subdirectories
                Box::pin(Self::copy_dir_recursive(&entry_path, &dest_path)).await?;
            } else if metadata.is_file() {
                // Copy regular files
                tokio::fs::copy(&entry_path, &dest_path).await?;
            }
            // Skip other file types (devices, sockets, etc.)
        }
        
        Ok(())
    }

    async fn extract_tar_gz(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        let file = std::fs::File::open(archive_path)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        
        tokio::fs::create_dir_all(extract_to).await?;
        archive.unpack(extract_to)?;
        
        Ok(())
    }

    async fn extract_tar_xz(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        let file = std::fs::File::open(archive_path)?;
        let decoder = xz2::read::XzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        
        tokio::fs::create_dir_all(extract_to).await?;
        archive.unpack(extract_to)?;
        
        Ok(())
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
