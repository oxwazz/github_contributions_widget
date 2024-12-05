use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

const GITHUB_TOKEN: &str = r"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";
const USER_AGENT: &str = r"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.3";
const ICON_PR_MERGED: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#8250df" class="octicon octicon-git-merge color-fg-done" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM5 3.25a.75.75 0 1 0 0 .005V3.25Z"></path></svg>"##;
const ICON_PR_CLOSED: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#d1242f" class="octicon octicon-git-pull-request-closed color-fg-closed" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M3.25 1A2.25 2.25 0 0 1 4 5.372v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.251 2.251 0 0 1 3.25 1Zm9.5 5.5a.75.75 0 0 1 .75.75v3.378a2.251 2.251 0 1 1-1.5 0V7.25a.75.75 0 0 1 .75-.75Zm-2.03-5.273a.75.75 0 0 1 1.06 0l.97.97.97-.97a.748.748 0 0 1 1.265.332.75.75 0 0 1-.205.729l-.97.97.97.97a.751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018l-.97-.97-.97.97a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l.97-.97-.97-.97a.75.75 0 0 1 0-1.06ZM2.5 3.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0ZM3.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm9.5 0a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Z"></path></svg>"##;
const ICON_PR_DRAFT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#59636e" class="octicon octicon-git-pull-request-draft color-fg-muted" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M3.25 1A2.25 2.25 0 0 1 4 5.372v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.251 2.251 0 0 1 3.25 1Zm9.5 14a2.25 2.25 0 1 1 0-4.5 2.25 2.25 0 0 1 0 4.5ZM2.5 3.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0ZM3.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm9.5 0a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5ZM14 7.5a1.25 1.25 0 1 1-2.5 0 1.25 1.25 0 0 1 2.5 0Zm0-4.25a1.25 1.25 0 1 1-2.5 0 1.25 1.25 0 0 1 2.5 0Z"></path></svg>"##;
const ICON_PR_OPEN: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#1a7f37" class="octicon octicon-git-pull-request color-fg-open" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354ZM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5Zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Z"></path></svg>"##;
const ICON_STAR: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="#eac54f" aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-star-fill Button-visual"><path d="M8 .25a.75.75 0 0 1 .673.418l1.882 3.815 4.21.612a.75.75 0 0 1 .416 1.279l-3.046 2.97.719 4.192a.751.751 0 0 1-1.088.791L8 12.347l-3.766 1.98a.75.75 0 0 1-1.088-.79l.72-4.194L.818 6.374a.75.75 0 0 1 .416-1.28l4.21-.611L7.327.668A.75.75 0 0 1 8 .25Z"></path></svg>"##;
const QUERY: &str = r"
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

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    data: UserData,
}

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    user: User,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    avatarUrl: String,
    login: String,
    name: String,
    pullRequests: PullRequests,
}

#[derive(Deserialize, Serialize, Debug)]
struct PullRequests {
    nodes: Vec<PullRequest>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PullRequest {
    createdAt: String,
    repository: Repository,
    state: String,
    title: String,
    url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Repository {
    nameWithOwner: String,
    owner: Owner,
    stargazerCount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct Owner {
    avatarUrl: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/", |_, _| async move {
            Response::from_html("<p>Hey, I'm Oxwazz! 😄👋 Great to see you!</p>")
        })
        .get_async("/favicon.ico", |_, _| async move {
            let favicon = include_bytes!("../assets/favicon.ico");
            let mut response = Response::from_bytes(favicon.to_vec())?;
            response.headers_mut().set("Content-Type", "image/x-icon")?;
            response.headers_mut().set("Cache-Control", "max-age=300")?; // 5 minutes
            Ok(response)
        })
        .get_async("/svg", |_, _| async move {
            let mut response = Response::from_bytes(ICON_STAR.as_bytes().to_vec())?;
            response
                .headers_mut()
                .set("Content-Type", "image/svg+xml")?;
            Ok(response)
        })
        // get data github
        .get_async("/:username", |req: Request, _| async move {
            // Extract username from the URL
            let url = req.url().expect("Failed to get URL");
            let tes = url.path();
            let tes3 = url
                .query_pairs()
                .find(|(key, _)| key == "username")
                .map(|(_, value)| value.to_string())
                .unwrap_or_default();
            // Split the path and get the username
            let path_parts: Vec<&str> =
                url.path_segments().map(|c| c.collect()).unwrap_or_default();

            // Get username (first segment after the base path)
            let username = path_parts.first().unwrap_or(&"default_user");

            console_log!("Requested username: {:?}", tes);
            console_log!("Requested username: {:?}", tes3);
            console_log!("Requested username: {:?}", path_parts);
            console_log!("Requested username: {}", username);

            let client = Client::new();
            // Construct the JSON body with the query
            let body = json!({
                "query": QUERY,
                "variables": {
                    "username": username
                }
            });

            // Send the POST request to the GraphQL endpoint
            let response = client
                .post("https://api.github.com/graphql")
                .bearer_auth(GITHUB_TOKEN)
                .header("Content-Type", "application/json")
                .header("User-Agent", USER_AGENT)
                .json(&body)
                .send()
                .await;

            // Handle the response
            match response {
                Ok(res) => {
                    // Check if the response was successful
                    if res.status().is_success() {
                        let res: Data = res.json().await.expect("df");

                        console_log!("Request succeeded: {:#?}", res);
                        let mut response = Response::from_json(&res.data.user)?;
                        response
                            .headers_mut()
                            .set("Cache-Control", "public, max-age=300")?; // 5 minutes
                        Ok(response)
                    } else {
                        // Capture the error code
                        let status = res.status();
                        console_log!("Error: Status code: {}", status);

                        // Capture the error message from the response body
                        let error_body = res.text().await.unwrap_or_default();
                        console_log!("Error message: {:#?}", error_body);
                        Response::from_html("&resp")
                    }
                }
                Err(err) => {
                    console_log!("Request failed: {}", err);
                    Response::from_html("&resp")
                }
            }
        })
        .run(req, env)
        .await
}
