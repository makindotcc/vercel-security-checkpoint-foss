mod solver;

use crate::solver::{solve_challenge, solve_challenge_faster};
use reqwest::{
    StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect,
};
use std::{process, time::Instant};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36";

#[tokio::main]
async fn main() {
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://v0-hello-world-site-zeta.vercel.app".to_string());

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .user_agent(USER_AGENT)
        .default_headers(default_headers())
        .build()
        .unwrap();

    eprintln!("[1] Initial get {}", target);
    let initial_request = client.get(&target).send().await.unwrap();
    if initial_request.status() != StatusCode::TOO_MANY_REQUESTS
        && initial_request.status() != StatusCode::FORBIDDEN
    {
        eprintln!(
            "[*] No challenge — page returned {} directly",
            initial_request.status()
        );
        println!(
            "{}",
            initial_request
                .text()
                .await
                .as_deref()
                .unwrap_or("<err reading response>")
        );
        return;
    }
    let Some(token) = initial_request
        .headers()
        .get("x-vercel-challenge-token")
        .and_then(|v| v.to_str().ok())
    else {
        eprintln!(
            "[!] No challenge token in response headers: {:#?}",
            initial_request.headers()
        );
        process::exit(1);
    };

    eprintln!("    token: {token}");

    eprintln!("[2] Solving challenge...");

    {
        let started_solving_at = Instant::now();
        let solution = solve_challenge(&token, 42);
        eprintln!(
            "    found solution in {:?} the old fashioned way: {solution}",
            started_solving_at.elapsed()
        );
    }

    let started_solving_at = Instant::now();
    let solution = solve_challenge_faster(&token);
    eprintln!(
        "    found solution in {:?}: {solution}",
        started_solving_at.elapsed()
    );

    let challenge_url = format!("{target}/.well-known/vercel/security/request-challenge");
    eprintln!("[3] POST {challenge_url}");

    let solution_response = client
        .post(&challenge_url)
        .headers(build_challenge_headers(&SolutionData {
            token,
            solution: &solution,
            origin: &target,
        }))
        .send()
        .await
        .unwrap();
    eprintln!("    {}", solution_response.status());

    let cookie = solution_response
        .headers()
        .get("set-cookie")
        .map(|v| v.to_str().unwrap().to_string());
    if cookie.is_none() {
        eprintln!("[!] No cookie in response — solution rejected?");
        std::process::exit(1);
    }
    let cookie_val = cookie.unwrap().split(';').next().unwrap().to_string();
    eprintln!("    cookie: {cookie_val}");

    eprintln!("[4] GET {} (with cookie)", target);
    let mut final_headers = HeaderMap::new();
    final_headers.insert("cookie", HeaderValue::from_str(&cookie_val).unwrap());
    final_headers.insert("cache-control", HeaderValue::from_static("max-age=0"));
    final_headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    final_headers.insert(
        "referer",
        HeaderValue::from_str(&format!("{target}/")).unwrap(),
    );

    let final_response = client
        .get(&target)
        .headers(final_headers)
        .send()
        .await
        .unwrap();
    eprintln!("    {:?}", final_response.status());
    let body = final_response.text().await.unwrap();
    eprintln!("    body length: {}", body.len());
    // println!("{body}");
}

struct SolutionData<'a> {
    token: &'a str,
    solution: &'a str,
    origin: &'a str,
}

fn build_challenge_headers(data: &SolutionData) -> HeaderMap {
    let mut post_headers = HeaderMap::new();
    for (k, v) in [
        ("accept", "*/*"),
        ("accept-language", "en-US,en;q=0.9"),
        ("accept-encoding", "identity"),
        ("sec-fetch-site", "same-origin"),
        ("sec-fetch-mode", "cors"),
        ("sec-fetch-dest", "empty"),
        (
            "sec-ch-ua",
            "\"Not-A.Brand\";v=\"24\", \"Chromium\";v=\"146\"",
        ),
        ("sec-ch-ua-mobile", "?0"),
        ("sec-ch-ua-platform", "\"macOS\""),
        ("priority", "u=1, i"),
        ("x-vercel-challenge-token", &data.token),
        ("x-vercel-challenge-solution", &data.solution),
        ("x-vercel-challenge-version", "2"),
        ("origin", &data.origin),
        (
            "referer",
            &format!(
                "{}/.well-known/vercel/security/static/challenge.v2.min.js",
                data.origin
            ),
        ),
    ] {
        post_headers.insert(
            HeaderName::from_bytes(k.as_bytes()).unwrap(),
            HeaderValue::from_str(v).unwrap(),
        );
    }
    post_headers
}

fn default_headers() -> HeaderMap {
    let pairs = [
        (
            "accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        ),
        ("accept-language", "en-US,en;q=0.9"),
        ("accept-encoding", "identity"),
        (
            "sec-ch-ua",
            "\"Not-A.Brand\";v=\"24\", \"Chromium\";v=\"146\"",
        ),
        ("sec-ch-ua-mobile", "?0"),
        ("sec-ch-ua-platform", "\"macOS\""),
        ("sec-fetch-site", "none"),
        ("sec-fetch-mode", "navigate"),
        ("sec-fetch-user", "?1"),
        ("sec-fetch-dest", "document"),
        ("upgrade-insecure-requests", "1"),
        ("priority", "u=0, i"),
    ];
    let mut headers = HeaderMap::new();
    for (k, v) in pairs {
        headers.insert(
            HeaderName::from_bytes(k.as_bytes()).unwrap(),
            HeaderValue::from_str(v).unwrap(),
        );
    }
    headers
}
