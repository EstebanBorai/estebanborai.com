mod utils;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use url::Url;
use worker::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(mut req: Request, env: Env) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    log_request(&req);

    let api_url = env
        .var("API_URL")
        .expect("Missing \"API_URL\" environment variable.")
        .to_string();
    let api_url = Url::parse(&api_url).map_err(|err| worker::Error::from(err.to_string()))?;
    let client = reqwest::Client::new();
    let method = reqwest::Method::from_str(&req.method().to_string())
        .map_err(|err| worker::Error::from(err.to_string()))?;
    let api_url = utils::make_target_url(
        api_url,
        req.url()
            .map_err(|err| worker::Error::from(err.to_string()))?,
    );
    let req_body = req
        .bytes()
        .await
        .map_err(|err| worker::Error::from(err.to_string()))?;
    let mut outgoing_headers = HeaderMap::new();

    for key in req.headers().keys() {
        let header_name =
            HeaderName::from_str(&key).map_err(|err| worker::Error::from(err.to_string()))?;
        let header_value = req
            .headers()
            .get(&key)
            .map_err(|err| worker::Error::from(err.to_string()))?
            .unwrap();
        let header_value = HeaderValue::from_str(&header_value).unwrap();

        outgoing_headers.append(header_name, header_value);
    }

    outgoing_headers.append(
        HeaderName::from_str("X-Auth-Token").map_err(|err| worker::Error::from(err.to_string()))?,
        HeaderValue::from_str(
            &env.var("X_AUTH_TOKEN")
                .expect("Missing \"X_AUTH_TOKEN\" environment variable.")
                .to_string(),
        )
        .unwrap(),
    );

    let response = client
        .request(method, api_url)
        .headers(outgoing_headers)
        .body(reqwest::Body::from(req_body))
        .send()
        .await
        .map_err(|err| worker::Error::from(err.to_string()))?;
    let headers = response.headers().clone();
    let mut worker_response = worker::Response::from_bytes(
        response
            .bytes()
            .await
            .map_err(|err| worker::Error::from(err.to_string()))?
            .to_vec(),
    )
    .unwrap();
    let worker_response_headers = worker_response.headers_mut();

    for (name, value) in headers.iter() {
        worker_response_headers
            .set(
                name.to_string().as_str(),
                value
                    .to_str()
                    .map_err(|err| worker::Error::from(err.to_string()))?,
            )
            .map_err(|err| worker::Error::from(err.to_string()))?;
    }

    Ok(worker_response)
}
