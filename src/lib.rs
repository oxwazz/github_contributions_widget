mod generate_svg;
mod get_oss_contributions;

use crate::generate_svg::generate_svg;
use crate::get_oss_contributions::{get_oss_contributions, Data};
use chrono::{TimeZone, Utc};
use std::time::Duration;
use worker::{
    console_log, Context, Env, Request, Response,
    Result, Router,
};
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
            console_log!("{}",tes);

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
            let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg" width="139.02461mm" height="18.144964mm" viewBox="0 0 139.02461 18.144964" version="1.1" id="svg1" xml:space="preserve"><defs id="defs1"/><g id="layer1" transform="translate(-14.085964,-28.313465)"><g id="g9" transform="translate(10.102788,14.592916)"><g id="g8-6" transform="translate(-13.96171,-24.018567)"><rect style="fill:#f6f8fa;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.206464;stroke-linecap:square;stroke-linejoin:miter;stroke-opacity:1;paint-order:normal" id="rect1-4" width="138.81816" height="17.935101" x="18.048119" y="37.845749"/><g id="g7-9" transform="translate(-2.7277885,-1.0855835)"><text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="38.11879" y="46.118828" id="text1-5"><tspan id="tspan1-0" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="38.11879" y="46.118828">owner/project-name</tspan></text><text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.93889px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="38.021423" y="51.554306" id="text1-7-4"><tspan id="tspan1-5-8" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.58611px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.326" x="38.021423" y="51.554306">[FEAT] add some feature</tspan></text></g><g style="fill:#d1242f" id="g2-1-7" transform="matrix(0.43477182,0,0,0.43452841,24.166005,43.337582)"><path d="M 3.25,1 A 2.25,2.25 0 0 1 4,5.372 v 5.256 a 2.251,2.251 0 1 1 -1.5,0 V 5.372 A 2.251,2.251 0 0 1 3.25,1 Z m 9.5,5.5 a 0.75,0.75 0 0 1 0.75,0.75 v 3.378 a 2.251,2.251 0 1 1 -1.5,0 V 7.25 A 0.75,0.75 0 0 1 12.75,6.5 Z M 10.72,1.227 a 0.75,0.75 0 0 1 1.06,0 l 0.97,0.97 0.97,-0.97 a 0.748,0.748 0 0 1 1.265,0.332 0.75,0.75 0 0 1 -0.205,0.729 l -0.97,0.97 0.97,0.97 A 0.751,0.751 0 0 1 14.762,5.27 0.751,0.751 0 0 1 13.72,5.288 l -0.97,-0.97 -0.97,0.97 A 0.749,0.749 0 0 1 10.505,4.962 0.749,0.749 0 0 1 10.72,4.228 l 0.97,-0.97 -0.97,-0.97 a 0.75,0.75 0 0 1 0,-1.06 z M 2.5,3.25 a 0.75,0.75 0 1 0 1.5,0 0.75,0.75 0 0 0 -1.5,0 z M 3.25,12 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z m 9.5,0 a 0.75,0.75 0 1 0 0,1.5 0.75,0.75 0 0 0 0,-1.5 z" id="path1-1-5-1"/></g></g><rect style="fill:none;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" id="rect8" width="11.48887" height="4.5308218" x="131.37071" y="15.534246"/><rect style="fill:#f6f8fa;fill-opacity:1;stroke:none;stroke-width:0.310733;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" id="rect9" width="138.61183" height="0.2200667" x="4.1894417" y="13.720549"/></g></g></svg>"##;
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
                    return Response::from_html(err.to_string())
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

            console_log!("[OXWAZZ-LOG] Requested path: {}", &username);
            console_log!("[OXWAZZ-LOG] Requested query: {}", &query_username);

            match get_oss_contributions(username, github_token).await {
                Err(err) => {
                    console_log!("[OXWAZZ-ERR] Request failed: {}", err);
                    Response::from_html(err.to_string())
                }
                Ok(res) => {
                    // Check if the response was successful
                    if res.status().is_success() {
                        let res: Data = res.json().await.expect("[OXWAZZ-ERR] Request failed: error get data json");

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
