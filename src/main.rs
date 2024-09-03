use error_chain::error_chain;
use rsdlm::url::validate_url;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    if !validate_url(target) {
        // add proper error handling later
        println!("Invalid URL");
        return Ok(());
    }
    let response = reqwest::get(target).await?;

    let custom_path = PathBuf::from("./data");
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    let dest = custom_path.join(fname);
    println!("Path: {:?}", dest);
    let mut dest_file = File::create(dest)?;

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest_file)?;
    Ok(())
}
