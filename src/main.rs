use reqwest::Error;
use http::HeaderMap;


async fn post_request() -> Result<(), Error> {
    let url = "https://you-search-zeta.vercel.app/ping";

    let json_data = r#"
    {
        "id": "hello world"
    }"#;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = client.post(url)
                            .headers(headers)
                            .body(json_data.to_owned())
                            .send()
                            .await?;

    log::info!("Status Code: {}", response.status());
    let response_body = response.text().await?;
    log::info!("Response body: {}", response_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("starting ...");
    post_request().await?;
    Ok(())
}
