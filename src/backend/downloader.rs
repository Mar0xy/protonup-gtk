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

    pub async fn download_file(&self, url: &str, destination: &Path) -> Result<()> {
        self.download_file_with_progress(url, destination, |_progress| {}).await
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

    pub async fn extract_archive(&self, archive_path: &Path, extract_to: &Path) -> Result<()> {
        // Determine archive type by extension
        let extension = archive_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension {
            "gz" | "tgz" => self.extract_tar_gz(archive_path, extract_to).await,
            "xz" => self.extract_tar_xz(archive_path, extract_to).await,
            _ => Err(anyhow::anyhow!("Unsupported archive format: {}", extension)),
        }
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
