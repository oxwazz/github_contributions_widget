use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::From;
use worker::*;

const USER_AGENT: &str = r"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.3";
const TEST_WIDGET: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg" width="139.02461mm" height="18.144964mm" viewBox="0 0 139.02461 18.144964" version="1.1" id="svg1" xml:space="preserve"><defs id="defs1"/><g id="layer1" transform="translate(-14.085964,-28.313465)"><g id="g9" transform="translate(10.102788,14.592916)"><g id="g8-6" transform="translate(-13.96171,-24.018567)"><rect style="fill:#f6f8fa;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.206464;stroke-linecap:square;stroke-linejoin:miter;stroke-opacity:1;paint-order:normal" id="rect1-4" width="138.81816" height="17.935101" x="18.048119" y="37.845749"/><g id="g7-9" transform="translate(-2.7277885,-1.0855835)"><text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="38.11879" y="46.118828" id="text1-5"><tspan id="tspan1-0" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="38.11879" y="46.118828">owner/project-name</tspan></text><text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.93889px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="38.021423" y="51.554306" id="text1-7-4"><tspan id="tspan1-5-8" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.58611px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="38.021423" y="51.554306">[FEAT] add some feature</tspan></text></g><g style="fill:#d1242f" id="g2-1-7" transform="matrix(0.43477182,0,0,0.43452841,24.166005,43.337582)"><path d="M 3.25,1 A 2.25,2.25 0 0 1 4,5.372 v 5.256 a 2.251,2.251 0 1 1 -1.5,0 V 5.372 A 2.251,2.251 0 0 1 3.25,1 Z m 9.5,5.5 a 0.75,0.75 0 0 1 0.75,0.75 v 3.378 a 2.251,2.251 0 1 1 -1.5,0 V 7.25 A 0.75,0.75 0 0 1 12.75,6.5 Z M 10.72,1.227 a 0.75,0.75 0 0 1 1.06,0 l 0.97,0.97 0.97,-0.97 a 0.748,0.748 0 0 1 1.265,0.332 0.75,0.75 0 0 1 -0.205,0.729 l -0.97,0.97 0.97,0.97 A 0.751,0.751 0 0 1 14.762,5.27 0.751,0.751 0 0 1 13.72,5.288 l -0.97,-0.97 -0.97,0.97 A 0.749,0.749 0 0 1 10.505,4.962 0.749,0.749 0 0 1 10.72,4.228 l 0.97,-0.97 -0.97,-0.97 a 0.75,0.75 0 0 1 0,-1.06 z M 2.5,3.25 a 0.75,0.75 0 1 0 1.5,0 0.75,0.75 0 0 0 -1.5,0 z M 3.25,12 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z m 9.5,0 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z" id="path1-1-5-1"/></g></g><rect style="fill:none;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" id="rect8" width="11.48887" height="4.5308218" x="131.37071" y="15.534246"/><rect style="fill:#f6f8fa;fill-opacity:1;stroke:none;stroke-width:0.310733;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" id="rect9" width="138.61183" height="0.2200667" x="4.1894417" y="13.720549"/></g></g></svg>"##;
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
    console_error_panic_hook::set_once();
    let router = Router::new();
    let github_token = &env.var("GITHUB_TOKEN").expect("sdf");
    let github_token = &github_token.to_string();

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
            let mut response = Response::from_bytes(TEST_WIDGET.as_bytes().to_vec())?;
            // let mut response = Response::from_bytes(ICON_STAR.as_bytes().to_vec())?;
            response
                .headers_mut()
                .set("Content-Type", "image/svg+xml")?;
            Ok(response)
        })
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
                .bearer_auth(github_token)
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

                        let total_item = if res.data.user.pullRequests.nodes.len() >= 3 {
                            3
                        } else {
                            res.data.user.pullRequests.nodes.len()
                        };
                        console_log!("{}", res.data.user.pullRequests.nodes.len());
                        let mut Tes = String::from(format!(r###"<svg width="139.02461mm" height="{}mm" viewBox="0 0 139.02461 {}" version="1.1" id="svg1" xml:space="preserve" xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg">
<!--  18.144964-->
  <!--  18.113465-->

"###, (total_item as f32 * 18.144964), (total_item as f32 * 18.144964)));

                        for count in 0..total_item {
                            let g = format!(r##"
<g id="layer1" transform="translate(-14.085964,{})">
    <rect style="fill:#f6f8fa;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.206464;stroke-linecap:square;stroke-linejoin:miter;stroke-opacity:1;paint-order:normal" id="rect1-4" width="138.81816" height="17.935101" x="14.189197" y="28.420097" />
    <rect style="fill:#f6f8fa;fill-opacity:1;stroke:none;stroke-width:0.310733;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" id="rect9" width="138.61183" height="0.2200667" x="14.29223" y="28.313465" />
    <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.93889px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="31.434713" y="41.043072" id="text1-7-4">
      <tspan id="tspan1-5-8" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.58611px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="31.434713" y="41.043072">{}</tspan>
    </text>
    <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="31.53208" y="35.607594" id="text1-5">
      <tspan id="tspan1-0" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="31.53208" y="35.607594">{}</tspan>
    </text>
    <g style="display:{};fill:#8250df" id="g2-9" transform="matrix(0.42534395,0,0,0.42534395,20.158968,33.984894)">
      <path d="M 5.45,5.154 A 4.25,4.25 0 0 0 9.25,7.5 h 1.378 a 2.251,2.251 0 1 1 0,1.5 H 9.25 A 5.734,5.734 0 0 1 5,7.123 v 3.505 a 2.25,2.25 0 1 1 -1.5,0 V 5.372 A 2.25,2.25 0 1 1 5.45,5.154 Z M 4.25,13.5 a 0.75,0.75 0 1 0 0,-1.5 0.75,0.75 0 0 0 0,1.5 z M 12.75,9 a 0.75,0.75 0 1 0 0,-1.5 0.75,0.75 0 0 0 0,1.5 z M 5,3.25 a 0.75,0.75 0 1 0 0,0.005 z" id="path1-84" />
    </g>
    <g style="display:{};fill:#d1242f" id="g3-8" transform="matrix(0.42534395,0,0,0.42534395,20.370175,33.985395)">
      <path d="M 3.25,1 A 2.25,2.25 0 0 1 4,5.372 v 5.256 a 2.251,2.251 0 1 1 -1.5,0 V 5.372 A 2.251,2.251 0 0 1 3.25,1 Z m 9.5,5.5 a 0.75,0.75 0 0 1 0.75,0.75 v 3.378 a 2.251,2.251 0 1 1 -1.5,0 V 7.25 A 0.75,0.75 0 0 1 12.75,6.5 Z M 10.72,1.227 a 0.75,0.75 0 0 1 1.06,0 l 0.97,0.97 0.97,-0.97 a 0.748,0.748 0 0 1 1.265,0.332 0.75,0.75 0 0 1 -0.205,0.729 l -0.97,0.97 0.97,0.97 A 0.751,0.751 0 0 1 14.762,5.27 0.751,0.751 0 0 1 13.72,5.288 l -0.97,-0.97 -0.97,0.97 A 0.749,0.749 0 0 1 10.505,4.962 0.749,0.749 0 0 1 10.72,4.228 l 0.97,-0.97 -0.97,-0.97 a 0.75,0.75 0 0 1 0,-1.06 z M 2.5,3.25 a 0.75,0.75 0 1 0 1.5,0 0.75,0.75 0 0 0 -1.5,0 z M 3.25,12 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z m 9.5,0 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z" id="path1-1-1" />
    </g>
    <g style="display:{};fill:#59636e" id="g4-0" transform="matrix(0.42534395,0,0,0.42534395,20.372146,33.984603)">
      <path d="M 3.25,1 A 2.25,2.25 0 0 1 4,5.372 v 5.256 a 2.251,2.251 0 1 1 -1.5,0 V 5.372 A 2.251,2.251 0 0 1 3.25,1 Z m 9.5,14 a 2.25,2.25 0 1 1 0,-4.5 2.25,2.25 0 0 1 0,4.5 z M 2.5,3.25 a 0.75,0.75 0 1 0 1.5,0 0.75,0.75 0 0 0 -1.5,0 z M 3.25,12 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z m 9.5,0 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z M 14,7.5 a 1.25,1.25 0 1 1 -2.5,0 1.25,1.25 0 0 1 2.5,0 z m 0,-4.25 a 1.25,1.25 0 1 1 -2.5,0 1.25,1.25 0 0 1 2.5,0 z" id="path1-8-3" />
    </g>
    <g style="display:{};fill:#1a7f37" id="g5-0" transform="matrix(0.42534395,0,0,0.42534395,20.265598,34.068916)">
      <path d="m 1.5,3.25 a 2.25,2.25 0 1 1 3,2.122 v 5.256 a 2.251,2.251 0 1 1 -1.5,0 V 5.372 A 2.25,2.25 0 0 1 1.5,3.25 Z M 7.177,3.073 9.573,0.677 A 0.25,0.25 0 0 1 10,0.854 V 2.5 h 1 A 2.5,2.5 0 0 1 13.5,5 v 5.628 a 2.251,2.251 0 1 1 -1.5,0 V 5 A 1,1 0 0 0 11,4 H 10 V 5.646 A 0.25,0.25 0 0 1 9.573,5.823 L 7.177,3.427 a 0.25,0.25 0 0 1 0,-0.354 z M 3.75,2.5 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z m 0,9.5 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z M 12,12.75 a 0.75,0.75 0 1 0 1.5,0 0.75,0.75 0 0 0 -1.5,0 z" id="path1-3-4" />
    </g>
  </g>
            "##,
                                            -28.313465 + (count as f64 * 18.113465), // tes
                                            res.data.user.pullRequests.nodes.get(count).expect("harusnya dapat").title, // PR Title
                                            res.data.user.pullRequests.nodes.get(count).expect("harusnya dapat").repository.nameWithOwner, // owner and project owner
                                            if res.data.user.pullRequests.nodes.get(count).expect("harusnya dapat").state == "MERGED" { "inline" } else { "none" }, // status merge
                                            if res.data.user.pullRequests.nodes.get(count).expect("harusnya dapat").state == "CLOSED" { "inline" } else { "none" }, // status closed
                                            "none", // status draft
                                            if res.data.user.pullRequests.nodes.get(count).expect("harusnya dapat").state == "OPEN" { "inline" } else { "none" }, // status open
                            );
                            Tes = format!("{}{}", Tes, g);
                        }


                        Tes = format!("{}{}", Tes, "</svg>");

                        let mut response = Response::from_bytes(Tes.as_bytes().to_vec())?;
                        // let mut response = Response::from_bytes(ICON_STAR.as_bytes().to_vec())?;
                        response
                            .headers_mut()
                            .set("Content-Type", "image/svg+xml")?;
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
