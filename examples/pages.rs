use std::time::Duration;

use c8e_rs::C8e;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment variables or use defaults
    let domain =
        std::env::var("CONFLUENCE_DOMAIN").unwrap_or_else(|_| "example.atlassian.net".to_string());
    let email =
        std::env::var("CONFLUENCE_EMAIL").unwrap_or_else(|_| "your-email@example.com".to_string());
    let token = std::env::var("CONFLUENCE_TOKEN").unwrap_or_else(|_| "your-api-token".to_string());

    println!("🔗 Connecting to Confluence: {}", domain);
    println!("📧 Using email: {}", email);

    // Create the Confluence client
    let client = C8e::new(&domain)
        .auth(email, token)
        .timeout(Duration::from_secs(30))
        .build()?;

    // pages
    let page = client.pages();
    let content = page
        .get_by_id("164192", Some("body-format=atlas_doc_format"))
        .await;
    match content {
        Ok(page) => {
            println!("{:?}", page);
        }
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}
