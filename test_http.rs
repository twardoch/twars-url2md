use reqwest;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(10))
        .build()?;
    
    println!("Fetching URL...");
    let response = client
        .get("https://helpx.adobe.com/pl/indesign/using/using-fonts.html")
        .send()
        .await?;
    
    println!("Status: {}", response.status());
    println!("Content-Type: {:?}", response.headers().get("content-type"));
    
    let text = response.text().await?;
    println!("Response length: {} bytes", text.len());
    
    Ok(())
}