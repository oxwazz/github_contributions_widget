use crate::get_oss_contributions::PullRequest;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Utc};
use std::time::Duration;

const DEFAULT_HEIGHT_WRAPPER: f32 = 18.145;
const DEFAULT_POSITION_FIRST_ITEM: f32 = -28.313;
const DEFAULT_RANGE_POSITION_NEXT_ITEM: f32 = 18.113465;

fn uppercase_first_letter(word: &str) -> String {
    let mut c = word.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn parse_time_ago(timestamp: &str) -> String {
    let date1: DateTime<Utc> = match timestamp.parse() {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    let date2 = Utc::now();
    let seconds_diff = (date2 - date1).num_seconds();
    timeago::Formatter::new().convert(Duration::from_secs(seconds_diff as u64))
}

fn parse_number_compact(number: i32) -> String {
    let abs_number = number.abs();
    if abs_number < 1_000 {
        return number.to_string();
    }
    let suffixes = ["", "k", "M", "B", "T"];
    let log_number = (abs_number as f64).log10() / 3.0;
    let index = log_number.floor() as usize;
    if index >= suffixes.len() {
        return number.to_string();
    }
    let scaled_number = abs_number as f64 / (1_000_f64.powi(index as i32));
    let formatted = format!("{:.1}", scaled_number)
        .trim_end_matches(".0")
        .to_string();
    let sign = if number < 0 { "-" } else { "" };
    format!("{}{}{}", sign, formatted, suffixes[index])
}

fn get_formatted_date_now() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

async fn get_photo_base64_from_url(image_url: &str) -> String {
    let response = match reqwest::get(image_url).await {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    if !response.status().is_success() {
        return String::new();
    }
    let image_bytes = match response.bytes().await {
        Err(_) => return String::new(),
        Ok(v) => v,
    };
    general_purpose::STANDARD.encode(&image_bytes)
}

pub async fn generate_svg(username: &str, contributions: Vec<PullRequest>) -> String {
    let total_contributions = if contributions.len() >= 3 {
        3
    } else if !contributions.is_empty() {
        contributions.len()
    } else {
        return String::new();
    };

    let mut svg = format!(
        r###"
<svg width="139.025mm" height="{}mm" viewBox="0 0 139.025 {}" xml:space="preserve" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns="http://www.w3.org/2000/svg">
  <title>{} Open-Source Contributions</title>
  <defs>
    <linearGradient id="d">
      <stop style="stop-color:#ffffff;stop-opacity:1" offset="0"/>
      <stop style="stop-color:#ffffff;stop-opacity:1" offset=".744"/>
      <stop style="stop-color:#000000;stop-opacity:1" offset=".9"/>
    </linearGradient>
    <linearGradient id="b">
      <stop style="stop-color:#ffffff;stop-opacity:1" offset=".001"/>
      <stop style="stop-color:#ffffff;stop-opacity:1" offset=".788"/>
      <stop style="stop-color:#000000;stop-opacity:1" offset=".902"/>
    </linearGradient>
    <linearGradient xlink:href="#b" id="a" gradientUnits="userSpaceOnUse" x1="35.056" y1="49.495" x2="168.074" y2="49.495"/>
    <linearGradient xlink:href="#d" id="c" gradientUnits="userSpaceOnUse" x1="53.999" y1="71.145" x2="149.974" y2="71.145"/>
    <mask maskUnits="userSpaceOnUse" id="i">
      <path style="opacity:1;fill:url(#a);fill-opacity:1;stroke:none;stroke-width:0.206837;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" d="M35.056 43.985h133.018v11.021H35.056z"/>
    </mask>
    <mask maskUnits="userSpaceOnUse" id="j">
      <path style="opacity:1;fill:url(#c);fill-opacity:1;stroke:none;stroke-width:0.192219;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" d="M53.999 63.954h95.975v14.383H53.999z"/>
    </mask>
    <mask maskUnits="userSpaceOnUse" id="g">
      <path style="opacity:1;fill:url(#c);fill-opacity:1;stroke:none;stroke-width:0.192219;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" d="M53.999 63.954h95.975v14.383H53.999z"/>
    </mask>
    <mask maskUnits="userSpaceOnUse" id="e">
      <path style="opacity:1;fill:url(#c);fill-opacity:1;stroke:none;stroke-width:0.192219;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" d="M53.999 63.954h95.975v14.383H53.999z"/>
    </mask>
    <clipPath clipPathUnits="userSpaceOnUse" id="k">
      <circle style="display:inline;fill:#ffffff;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.0197735;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" cx="33.488" cy="34.627" r="1.649"/>
    </clipPath>
    <clipPath clipPathUnits="userSpaceOnUse" id="h">
      <circle style="display:inline;fill:#ffffff;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.0197735;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" cx="33.488" cy="34.627" r="1.649"/>
    </clipPath>
    <clipPath clipPathUnits="userSpaceOnUse" id="f">
      <circle style="display:inline;fill:#ffffff;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.0197735;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" cx="33.488" cy="34.627" r="1.649"/>
    </clipPath>
  </defs>
  <g transform="matrix(1.00001 0 0 1.00157 -30.683 -35.56)">
    <path style="fill:#f6f8fa;stroke:#d1d9e0;stroke-width:0.206042;stroke-linecap:square;stroke-dasharray:none" d="M33.495 35.677h133.399c1.5 0 2.709 1.208 2.709 2.709v15.201s-101.107-.108-138.817 0V38.36c0-1.5 1.208-2.682 2.709-2.682z"/>
    <g transform="translate(-1.38 -3.367)" mask="url(#i)">
      <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.58249px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.325744;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="37.721" y="48.992" transform="scale(1.00078 .99922)"><tspan style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.58249px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#1f2328;fill-opacity:1;stroke:none;stroke-width:0.325743" x="37.721" y="48.992">{} Open-Source Contributions</tspan></text>
      <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:2.82px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;opacity:0.8;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.325744;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="36.418" y="49.6" transform="matrix(1.00078 0 0 .99922 1.38 3.367)"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:2.82px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;letter-spacing:0px;text-anchor:start;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.325743" x="36.418" y="49.6">- Updated at {} -</tspan></text>
    </g>
  </g>
"###,
        (total_contributions as f32 + 1.0) * DEFAULT_HEIGHT_WRAPPER,
        (total_contributions as f32 + 1.0) * DEFAULT_HEIGHT_WRAPPER,
        uppercase_first_letter(username),
        uppercase_first_letter(username),
        get_formatted_date_now()
    );

    for count in 0..(total_contributions - 1) {
        let contribution = contributions
            .get(count)
            .expect("[OXWAZZ-ERR] Request failed: error get data contribution");
        let g = format!(
            r##"
  <g transform="translate(-14.086 {})">
    <g transform="matrix(1 0 0 1.0002 0 -.006)">
      <path style="fill:#f6f8fa;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.206464;stroke-linecap:square;stroke-linejoin:miter;stroke-opacity:1;paint-order:normal" d="M14.189 28.42h138.818v17.935H14.189z"/>
      <path style="fill:#f6f8fa;fill-opacity:1;stroke:none;stroke-width:0.310733;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" d="M14.292 28.317h138.612v.22H14.292z"/>
    </g>
    <g transform="matrix(1.00001 0 0 1.00155 -25.695 -34.045)">
      <g transform="matrix(.99999 0 0 .99845 24.999 34.017)">
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:end;writing-mode:lr-tb;direction:ltr;text-anchor:end;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="147.984" y="38.793"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:end;text-anchor:end;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="147.984" y="38.793">{}</tspan></text>
        <path d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5zM5 3.25a.75.75 0 1 0 0 .005z" style="display:{};fill:#8250df" transform="matrix(.42534 0 0 .42534 20.159 33.985)"/>
        <path d="M3.25 1A2.25 2.25 0 0 1 4 5.372v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.251 2.251 0 0 1 3.25 1Zm9.5 5.5a.75.75 0 0 1 .75.75v3.378a2.251 2.251 0 1 1-1.5 0V7.25a.75.75 0 0 1 .75-.75Zm-2.03-5.273a.75.75 0 0 1 1.06 0l.97.97.97-.97a.748.748 0 0 1 1.265.332.75.75 0 0 1-.205.729l-.97.97.97.97a.751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018l-.97-.97-.97.97a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l.97-.97-.97-.97a.75.75 0 0 1 0-1.06zM2.5 3.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0zM3.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm9.5 0a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z" style="display:{};fill:#d1242f" transform="matrix(.42534 0 0 .42534 20.37 33.985)"/>
        <path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354zM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0z" style="display:{};fill:#1a7f37" transform="matrix(.42534 0 0 .42534 20.266 34.069)"/>
      </g>
      <g mask="url(#e)">
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.93889px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="31.435" y="41.043" transform="matrix(.99999 0 0 .99845 24.999 34.017)"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.58611px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="31.435" y="41.043">{}</tspan></text>
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="36.35" y="35.608" transform="matrix(.99999 0 0 .99845 24.999 34.017)"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="36.35" y="35.608">{} ‧ <tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:2.82222px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal">{}</tspan></tspan></text>
        <g transform="matrix(.99999 0 0 .99845 24.999 34.029)">
          <circle style="display:inline;fill:#ffffff;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.02;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" cx="33.488" cy="34.627" r="1.668"/>
          <image width="3.36" height="3.36" preserveAspectRatio="none" xlink:href="data:image/png;base64,{}" x="31.805" y="32.953" clip-path="url(#f)" transform="matrix(1.0062 0 0 1.0062 -.208 -.215)"/>
        </g>
      </g>
    </g>
  </g>
            "##,
            DEFAULT_POSITION_FIRST_ITEM + ((count as f32 + 1.0) * DEFAULT_RANGE_POSITION_NEXT_ITEM), // tes
            parse_time_ago(&contribution.createdAt),
            if contribution.state == "MERGED" {
                "inline"
            } else {
                "none"
            }, // status merge
            if contribution.state == "CLOSED" {
                "inline"
            } else {
                "none"
            }, // status closed
            if contribution.state == "OPEN" {
                "inline"
            } else {
                "none"
            }, // status open
            contribution.title,                    // PR Title
            contribution.repository.nameWithOwner, // owner and project owner
            parse_number_compact(contribution.repository.stargazerCount), // project stargazers
            get_photo_base64_from_url(contribution.repository.owner.avatarUrl.as_str())
                .await
        );
        svg = format!("{svg}{g}");
    }

    if total_contributions >= 3 {
        let count = 2; // index item 3
        let contribution = contributions
            .get(count)
            .expect("[OXWAZZ-ERR] Request failed: error get data contribution");
        let g = format!(
            r##"
  <g transform="matrix(1.00001 0 0 1.00155 -39.781 {})">
    <g transform="matrix(1 0 0 1.00001 0 0)">
      <path style="fill:#f6f8fa;stroke:#d1d9e0;stroke-width:0.206042;stroke-linecap:square;stroke-dasharray:none" d="M175.992 80.279h-133.4a2.703 2.703 0 0 1-2.708-2.709V62.368s101.107.109 138.817 0v15.229c0 1.5-1.208 2.682-2.71 2.682z"/>
      <path style="fill:#f6f8fa;fill-opacity:1;stroke:none;stroke-width:0.40073;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" d="M39.987 62.265h138.612v.366H39.987z"/>
    </g>
    <g>
      <g transform="matrix(.99999 0 0 .99845 24.999 34.017)">
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:end;writing-mode:lr-tb;direction:ltr;text-anchor:end;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="147.984" y="38.793"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:end;text-anchor:end;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="147.984" y="38.793">{}</tspan></text>
        <path d="M5.45 5.154A4.25 4.25 0 0 0 9.25 7.5h1.378a2.251 2.251 0 1 1 0 1.5H9.25A5.734 5.734 0 0 1 5 7.123v3.505a2.25 2.25 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.95-.218ZM4.25 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5zm8.5-4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5zM5 3.25a.75.75 0 1 0 0 .005z" style="display:{};fill:#8250df" transform="matrix(.42534 0 0 .42534 20.159 33.985)"/>
        <path d="M3.25 1A2.25 2.25 0 0 1 4 5.372v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.251 2.251 0 0 1 3.25 1Zm9.5 5.5a.75.75 0 0 1 .75.75v3.378a2.251 2.251 0 1 1-1.5 0V7.25a.75.75 0 0 1 .75-.75Zm-2.03-5.273a.75.75 0 0 1 1.06 0l.97.97.97-.97a.748.748 0 0 1 1.265.332.75.75 0 0 1-.205.729l-.97.97.97.97a.751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018l-.97-.97-.97.97a.749.749 0 0 1-1.275-.326.749.749 0 0 1 .215-.734l.97-.97-.97-.97a.75.75 0 0 1 0-1.06zM2.5 3.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0zM3.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm9.5 0a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z" style="display:{};fill:#d1242f" transform="matrix(.42534 0 0 .42534 20.37 33.985)"/>
        <path d="M1.5 3.25a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-1.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25Zm5.677-.177L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-1.5 0V5a1 1 0 0 0-1-1h-1v1.646a.25.25 0 0 1-.427.177L7.177 3.427a.25.25 0 0 1 0-.354zM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm0 9.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm8.25.75a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0z" style="display:{};fill:#1a7f37" transform="matrix(.42534 0 0 .42534 20.266 34.069)"/>
      </g>
      <g mask="url(#j)">
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.93889px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="31.435" y="41.043" transform="matrix(.99999 0 0 .99845 24.999 34.017)"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:4.58611px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="31.435" y="41.043">{}</tspan></text>
        <text xml:space="preserve" style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326001;stroke-linecap:butt;stroke-linejoin:bevel;stroke-opacity:1;paint-order:normal" x="36.35" y="35.608" transform="matrix(.99999 0 0 .99845 24.999 34.017)"><tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:3.52778px;font-family:'Noto Sans';-inkscape-font-specification:'Noto Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#59636e;fill-opacity:1;stroke:none;stroke-width:0.326" x="36.35" y="35.608">{} ‧ <tspan style="font-style:normal;font-variant:normal;font-weight:normal;font-stretch:normal;font-size:2.82222px;font-family:Sans;-inkscape-font-specification:'Sans, Normal';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal">{}</tspan></tspan></text>
        <g transform="matrix(.99999 0 0 .99845 24.999 34.029)">
          <circle style="display:inline;fill:#ffffff;fill-opacity:1;stroke:#d1d9e0;stroke-width:0.02;stroke-linecap:square;stroke-dasharray:none;stroke-opacity:1" cx="33.488" cy="34.627" r="1.668"/>
          <image width="3.36" height="3.36" preserveAspectRatio="none" xlink:href="data:image/png;base64,{}" x="31.805" y="32.953" clip-path="url(#k)" transform="matrix(1.0062 0 0 1.0062 -.208 -.215)"/>
        </g>
      </g>
    </g>
  </g>
            "##,
            -62.26639 + ((count as f32 + 1.0) * DEFAULT_RANGE_POSITION_NEXT_ITEM), // tes
            parse_time_ago(&contribution.createdAt),
            if contribution.state == "MERGED" {
                "inline"
            } else {
                "none"
            }, // status merge
            if contribution.state == "CLOSED" {
                "inline"
            } else {
                "none"
            }, // status closed
            if contribution.state == "OPEN" {
                "inline"
            } else {
                "none"
            }, // status open
            contribution.title,                    // PR Title
            contribution.repository.nameWithOwner, // owner and project owner
            parse_number_compact(contribution.repository.stargazerCount), // project stargazers
            get_photo_base64_from_url(contribution.repository.owner.avatarUrl.as_str())
                .await
                .to_string()
        );
        svg = format!("{svg}{g}");
    }

    format!("{svg}</svg>")
}
