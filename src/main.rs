use error_chain::error_chain;
use rsdlm::downloader::download_file;
use rsdlm::utils::validate_url;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

/* todo ->
    add error handling
    add auto file format detection
    add progress bar
    add simple tui
    expand to multi chunk parallel downloading,
        multi file downloading, multi threaded downloading
*/

#[tokio::main]
async fn main() -> Result<()> {
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let file_path = "./data/downloaded_file.png";
    if !validate_url(target) {
        // add proper error handling later
        eprintln!("Invalid URL");
        return Ok(());
    }

    match download_file(&target, &file_path).await {
        Ok(_) => println!("File downloaded successfully"),
        Err(e) => eprintln!("Error downloading file: {}", e),
    }

    Ok(())
}
