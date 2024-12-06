use reqwest::header::USER_AGENT;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub data: UserData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub avatarUrl: String,
    pub login: String,
    pub name: String,
    pub pullRequests: PullRequests,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PullRequests {
    pub nodes: Vec<PullRequest>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PullRequest {
    pub createdAt: String,
    pub repository: Repository,
    pub state: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    pub nameWithOwner: String,
    pub owner: Owner,
    pub stargazerCount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub avatarUrl: String,
}

pub async fn get_oss_contributions(
    username: &str,
    github_token: &str,
) -> reqwest::Result<Response> {
    let client = Client::new();

    let query = r"
      query ($username: String!) {
        user(login: $username) {
          name
          login
          avatarUrl
          pullRequests(first: 100, orderBy: { field: CREATED_AT, direction: DESC }) {
            nodes {
              title
              url
              state
              createdAt
              repository {
                nameWithOwner
                stargazerCount
                owner {
                  avatarUrl
                }
              }
            }
          }
        }
      }
    ";
    
    let body = json!({
        "query": query,
        "variables": {
            "username": username
        }
    });

    client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .header("Content-Type", "application/json")
        .header("User-Agent", USER_AGENT)
        .json(&body)
        .send()
        .await
}
