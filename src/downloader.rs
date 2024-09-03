use std::{error::Error, fs::File, io::Write};

use reqwest::Client;

pub async fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    let mut file = File::create(file_path)?;

    let buffer = response.bytes().await?;

    let _ = file.write_all(&buffer);

    println!("Downloaded file to: {}", file_path);
    Ok(())
}
