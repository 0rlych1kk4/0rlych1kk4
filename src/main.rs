use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::{collections::BTreeMap, env, fs};

const START: &str = "<!-- MERGED-PRS:START -->";
const END: &str = "<!-- MERGED-PRS:END -->";
const MAX_PRS_PER_REPO: usize = 5;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<PullRequest>,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    number: u64,
    title: String,
    html_url: String,
    repository_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("GITHUB_USERNAME").unwrap_or_else(|_| "0rlych1kk4".to_string());

    let token =
        env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable is required")?;

    println!("Searching merged PRs for {username}...");

    let client = reqwest::Client::new();
    let query = format!("is:pr is:merged author:{username}");

    let mut all_prs = Vec::new();

    for page in 1.. {
        println!("Fetching page {page}...");

        let page_string = page.to_string();

        let response = client
            .get("https://api.github.com/search/issues")
            .query(&[
                ("q", query.as_str()),
                ("per_page", "100"),
                ("page", page_string.as_str()),
            ])
            .header(USER_AGENT, "github-profile-pr-updater")
            .header(ACCEPT, "application/vnd.github+json")
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?
            .error_for_status()?
            .json::<SearchResponse>()
            .await?;

        let count = response.items.len();

        if count == 0 {
            break;
        }

        all_prs.extend(response.items);

        if count < 100 {
            break;
        }
    }

    println!("GitHub returned {} merged PRs.", all_prs.len());

    let mut repositories: BTreeMap<String, Vec<PullRequest>> = BTreeMap::new();

    for pr in all_prs {
        let repo = pr
            .repository_url
            .trim_start_matches("https://api.github.com/repos/")
            .to_string();

        let owner = repo.split('/').next().unwrap_or("");

        // Hide PRs merged into repositories you own.
        if owner.eq_ignore_ascii_case(&username) {
            continue;
        }

        repositories.entry(repo).or_default().push(pr);
    }

    // Newest PRs first within each repository.
    for prs in repositories.values_mut() {
        prs.sort_by(|a, b| b.number.cmp(&a.number));
    }

    let total: usize = repositories.values().map(Vec::len).sum();

    let mut markdown = String::new();

    markdown.push_str(&format!(
        "**{} merged pull requests across {} external repositories.** \
Automatically generated from my upstream GitHub contributions.\n\n",
        total,
        repositories.len()
    ));

    for (repo, prs) in &repositories {
        let total_repo_prs = prs.len();

        markdown.push_str(&format!(
            "### [{}](https://github.com/{}) — {} merged PR{}\n\n",
            repo,
            repo,
            total_repo_prs,
            if total_repo_prs == 1 { "" } else { "s" }
        ));

        markdown.push_str("| PR | Contribution |\n");
        markdown.push_str("| --- | --- |\n");

        for pr in prs.iter().take(MAX_PRS_PER_REPO) {
            let title = pr.title.replace('|', "\\|");

            markdown.push_str(&format!(
                "| [#{}]({}) | {} |\n",
                pr.number, pr.html_url, title
            ));
        }

        markdown.push('\n');

        if total_repo_prs > MAX_PRS_PER_REPO {
            markdown.push_str(&format!(
                "[View all {} merged PRs in {}](https://github.com/{}/pulls?q=is%3Apr+is%3Amerged+author%3A{})\n\n",
                total_repo_prs,
                repo,
                repo,
                username
            ));
        }
    }

    update_readme(&markdown)?;

    println!(
        "README updated with {} merged PRs across {} repositories.",
        total,
        repositories.len()
    );

    Ok(())
}

fn update_readme(generated: &str) -> Result<()> {
    let readme = fs::read_to_string("README.md").context("Could not read README.md")?;

    let start_position = readme
        .find(START)
        .context("README is missing MERGED-PRS:START marker")?;

    let end_position = readme
        .find(END)
        .context("README is missing MERGED-PRS:END marker")?;

    if start_position >= end_position {
        anyhow::bail!("README PR markers are in the wrong order");
    }

    let before = &readme[..start_position + START.len()];

    let after = &readme[end_position..];

    let updated = format!("{before}\n{generated}{after}");

    fs::write("README.md", updated).context("Could not write README.md")?;

    Ok(())
}
