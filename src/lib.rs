mod generate_svg;
mod generate_svg_empty_state;
mod generate_svg_error_state;
mod get_oss_contributions;
mod utils;

use crate::generate_svg::generate_svg;
use crate::generate_svg_empty_state::generate_svg_empty_state;
use crate::generate_svg_error_state::generate_svg_error_state;
use crate::get_oss_contributions::{get_oss_contributions, Data};
use worker::{console_log, Context, Env, Request, Response, Result, Router};
use worker_macros::event;

fn return_error_state(username: &str) -> Result<Response> {
    let svg = generate_svg_error_state(username);
    let mut response = Response::from_bytes(svg.as_bytes().to_vec())?;
    response
        .headers_mut()
        .set("Content-Type", "image/svg+xml")?;
    Ok(response)
}

fn return_empty_state(username: &str) -> Result<Response> {
    let svg = generate_svg_empty_state(username);
    let mut response = Response::from_bytes(svg.as_bytes().to_vec())?;
    response
        .headers_mut()
        .set("Content-Type", "image/svg+xml")?;
    Ok(response)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // init worker error panic
    console_error_panic_hook::set_once();

    // init worker router
    let router = Router::new();

    // get github token from env
    let github_token = match env.var("GITHUB_TOKEN") {
        Err(err) => {
            // error early return
            println!(
                "[OXWAZZ-ERR] Request failed: error get data GITHUB_TOKEN: {}",
                err
            );
            return return_error_state("<undefined>");
        }
        Ok(v) => v,
    };
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
        .get_async("/test-svg", |_, _| async move {
            let svg = include_bytes!("../assets/snapshot/v0.1.3.svg");
            let mut response = Response::from_bytes(svg.to_vec())?;
            response
                .headers_mut()
                .set("Content-Type", "image/svg+xml")?;
            Ok(response)
        })
        .get_async("/:username", |req: Request, _| async move {
            let url = match req.url() {
                Err(err) => {
                    // error early return
                    console_log!("[OXWAZZ-ERR] Request failed: {}", err);
                    return return_error_state("<undefined>");
                }
                Ok(v) => v,
            };

            // Get path :username
            let username: Vec<&str> = url.path_segments().map(|c| c.collect()).unwrap_or_default();
            let username = username.first().unwrap_or(&"");

            // Get query ?states=
            let query_states = url
                .query_pairs()
                .find(|(key, _)| key == "states")
                .map(|(_, value)| value.to_string());

            console_log!("[OXWAZZ-LOG] Requested path: {}", username);
            console_log!("[OXWAZZ-LOG] Requested query: {:?}", query_states);

            match get_oss_contributions(username, query_states.as_deref(), github_token).await {
                Err(err) => {
                    // error early return
                    console_log!("[OXWAZZ-ERR] Request failed: {}", err);
                    return_error_state(username)
                }
                Ok(res) => {
                    // Check if the response was successful
                    if res.status().is_success() {
                        let res: Data = match res.json().await {
                            Err(err) => {
                                // error early return
                                console_log!(
                                    "[OXWAZZ-ERR] Request failed: error get data json: {}",
                                    err
                                );
                                return return_error_state(username);
                            }
                            Ok(v) => v,
                        };

                        // early return if user doesn't have contributions
                        if res.data.user.pullRequests.nodes.len().le(&0) {
                            return return_empty_state(username);
                        }

                        let svg = generate_svg(username, res.data.user.pullRequests.nodes).await;
                        // early return generate_svg has error inside / return empty string
                        if svg.is_empty() {
                            return return_empty_state(username);
                        }
                        let mut response = Response::from_bytes(svg.as_bytes().to_vec())?;
                        response
                            .headers_mut()
                            .set("Content-Type", "image/svg+xml")?;
                        Ok(response)
                    } else {
                        // get error detail
                        let status = res.status();
                        let error_body = res.text().await.unwrap_or_default();

                        // error early return
                        console_log!("[OXWAZZ-ERR] Request failed: {} {:?}", status, error_body);
                        return_error_state(username)
                    }
                }
            }
        })
        .run(req, env)
        .await
}
