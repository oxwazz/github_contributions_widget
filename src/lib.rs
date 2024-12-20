mod generate_svg;
mod get_oss_contributions;

use crate::generate_svg::generate_svg;
use crate::get_oss_contributions::{get_oss_contributions, Data};
use chrono::{TimeZone, Utc};
use std::time::Duration;
use worker::{console_log, Context, Env, Request, Response, Result, Router};
use worker_macros::event;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();
    let github_token = &env
        .var("GITHUB_TOKEN")
        .expect("[OXWAZZ-ERR] Request failed: error get data GITHUB_TOKEN");
    let github_token = &github_token.to_string();

    router
        .get_async("/", |_, _| async move {
            let date1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
            let date2 = Utc::now();

            let seconds_diff = (date2 - date1).num_seconds();

            let tes = timeago::Formatter::new().convert(Duration::from_secs(seconds_diff as u64));
            // let tes = timeago::Formatter::new().convert(Duration::from_secs(60));
            console_log!("{}", tes);

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
            let svg = include_bytes!("../assets/snapshot/v0.1.2.svg");
            let mut response = Response::from_bytes(svg.to_vec())?;
            response
                .headers_mut()
                .set("Content-Type", "image/svg+xml")?;
            Ok(response)
        })
        .get_async("/:username", |req: Request, _| async move {
            let url = match req.url() {
                Err(err) => {
                    console_log!("[OXWAZZ-ERR] Request failed: {}", err);
                    return Response::from_html(err.to_string());
                }
                Ok(v) => v,
            };

            // Get path :username
            let username: Vec<&str> = url.path_segments().map(|c| c.collect()).unwrap_or_default();
            let username = username.first().unwrap_or(&"");

            // Get query ?username=
            let query_username = url
                .query_pairs()
                .find(|(key, _)| key == "username")
                .map(|(_, value)| value.to_string())
                .unwrap_or_default();

            // Get query ?states=
            let query_states = url
                .query_pairs()
                .find(|(key, _)| key == "states")
                .map(|(_, value)| value.to_string());

            console_log!("[OXWAZZ-LOG] Requested path: {}", &username);
            console_log!("[OXWAZZ-LOG] Requested query: {}", &query_username);
            console_log!("[OXWAZZ-LOG] Requested query: {:?}", &query_states);

            match get_oss_contributions(username, query_states.as_deref(), github_token).await {
                Err(err) => {
                    console_log!("[OXWAZZ-ERR] Request failed: {}", err);
                    Response::from_html(err.to_string())
                }
                Ok(res) => {
                    // Check if the response was successful
                    if res.status().is_success() {
                        let res: Data = res
                            .json()
                            .await
                            .expect("[OXWAZZ-ERR] Request failed: error get data json");

                        let svg = generate_svg(username, res.data.user.pullRequests.nodes);
                        let mut response = Response::from_bytes(svg.await.as_bytes().to_vec())?;
                        response
                            .headers_mut()
                            .set("Content-Type", "image/svg+xml")?;
                        Ok(response)
                    } else {
                        // Capture the error code
                        let status = res.status();
                        // Capture the error message from the response body
                        let error_body = res.text().await.unwrap_or_default();
                        console_log!("[OXWAZZ-ERR] Request failed: {} {:?}", status, error_body);
                        Response::from_html(&error_body)
                    }
                }
            }
        })
        .run(req, env)
        .await
}
