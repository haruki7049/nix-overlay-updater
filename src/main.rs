use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    let url: String = format!("https://api.github.com/repos/{}/{}/releases", args.owner, args.repo);
    let client = reqwest::Client::new();
    let response = client.get(url);
    println!("{response:#?}");
    Ok(())
}

#[derive(Parser)]
struct CLIArgs {
    owner: String,
    repo: String,
}
