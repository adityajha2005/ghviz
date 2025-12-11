use serde::Deserialize;
use clap::Parser;
use colored::*;

#[derive(Debug, Deserialize)]
struct CommitInfo {
    sha: String,
    html_url: String,
    commit: InnerCommit,
}

#[derive(Debug, Deserialize)]
struct InnerCommit {
    message: String,
    author: AuthorInfo,
}

#[derive(Debug, Deserialize)]
struct AuthorInfo {
    name: String,
    date: chrono::DateTime<chrono::Utc>,
}

#[derive(Parser)]
struct Args {
    repo: String,
    #[arg(short, long, default_value_t = 10)]
    limit: usize
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match fetch_commits(&args.repo).await {
        Ok(commits) => print_commits(&commits, args.limit),
        Err(e) => eprintln!("Error fetching commits: {}", e),
    }
}

async fn fetch_commits(repo: &str) -> Result<Vec<CommitInfo>, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{repo}/commits");
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "ghviz")
        .send()
        .await?
        .json::<Vec<CommitInfo>>()
        .await?;

    Ok(resp)
}

fn print_commits(commits: &[CommitInfo], limit: usize) {
    let standard = figlet_rs::FIGfont::standard().unwrap();
let figure = standard.convert("GHVIZ").unwrap();
println!("{}", figure.to_string().bright_blue().bold());

     println!("\n{}", "Commit History".bold().underline());
    println!("{}", "---------------------------------------------------------".yellow());

    for c in commits.iter().take(limit) {
        let sha = &c.sha[..7]; // short sha
        let author = c.commit.author.name.blue();
        let message = c.commit.message.yellow();
        let date = c.commit.author.date.format("%Y-%m-%d").to_string().green();

        println!("{:<8} | {:<20} | {:<40} | {}\n   {}", 
    sha.red(), author, message, date,
    c.html_url.underline().cyan()
);

    }

    println!("{}", "---------------------------------------------------------".yellow());
}
