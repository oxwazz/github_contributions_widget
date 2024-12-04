const QUERY: &str = r"query ($username: String!) {
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
  }";

#[tokio::main]
async fn main() {
    let token = String::from("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");

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("asdf");
    let response: serde_json::Value = octocrab
        .graphql(&serde_json::json!({
            "query": QUERY,
            "variables": {
                "username": "fransdhinta"
            }
        }))
        .await
        .expect("REASON");
    dbg!(response);
}
