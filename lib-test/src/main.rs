use dkg_client_rust::client::DkgClient;
use reqwest::Client;

extern crate dkg_client_rust;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let dkg_client = DkgClient {
        http_client: Client::new(),
        endpoint: String::from("http://0.0.0.0:8900/"),
    };

    let node_infos = dkg_client.node_info().await?;

    println!("Node info struct: {}", node_infos);
    println!("Node version: {}", node_infos["version"]);

    let publishing_reponse = dkg_client.publish("../dataset.json").await?;

    println!(
        "Status for our dataset is: {}",
        publishing_reponse["status"].as_str().unwrap()
    );

    Ok(())
}
