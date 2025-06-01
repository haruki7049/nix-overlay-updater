use clap::Parser;
use reqwest::Client;
use reqwest::header;
use serde::{Serialize, Deserialize};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let url: String = format!(
        "https://api.github.com/repos/{}/{}/releases",
        args.owner, args.repo
    );

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()?;

    let body = client.get(url).send().await?.json::<Vec<Release>>().await?;
    println!("{:?}", body);

    //let body = client.get(url).send().await?.text().await?;
    //println!("{}", body);

    Ok(())
}

#[derive(Parser)]
struct CLIArgs {
    owner: String,
    repo: String,

    #[arg(short, long, default_value_t = true)]
    save_json: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    name: String,
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}
