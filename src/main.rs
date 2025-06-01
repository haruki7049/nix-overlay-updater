use clap::Parser;
use serde::{Deserialize, Serialize};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set URL & etc
    let args: CLIArgs = CLIArgs::parse();

    // Sending requests
    let mut page = octocrab::instance()
        .repos(args.owner, args.repo)
        .releases()
        .list()
        .send()
        .await?;

    for release in page.take_items() {
        for asset in release.assets {
            println!("{}", asset.browser_download_url);
        }
    }

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
struct JsonOutput(Vec<Release>);

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    version: String,
    url: Url,
    arch: Arch,
    sha256: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
enum Arch {
    x86_64_linux,
    aarch64_linux,
    x86_64_darwin,
    aarch64_darwin,
}
