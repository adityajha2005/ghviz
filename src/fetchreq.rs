// pub async fn get<T: IntoUrl>(url: T) -> Result<Response>

async fn fetch_commits(repo:&str)->Result<Vec<CommitInfo>,reqwest::Error>{
    let url = format!("https://api.github.com/repos/{repo}/commits");

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent":"ghviz")
        .send()
        .await?
        .json::<Vec<CommitInfo>>()
        .await?;
    Ok(resp)
}
