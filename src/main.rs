use std::{fs::File, io::copy};
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("File to downlaod: {}", fname);
        let dir = tmp_dir.path().join(fname);
        println!("File path: {:?}", dir);
        File::create(dir)?
    };

    let content = response.text().await?;
    let _ = copy(&mut content.as_bytes(), &mut dest);

    Ok(())
}
