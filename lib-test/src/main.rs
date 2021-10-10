use dkg_client_rust::client::DkgClient;
use reqwest::Client;

extern crate dkg_client_rust;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    //dkg_client_rust::client::DkgClient::init(String::from("http://0.0.0.0:8900/api/latest/info"));

    let DKGClient = DkgClient {
        http_client: Client::new(),
        endpoint: String::from("http://0.0.0.0:8900/"),
    };

    let node_infos = DKGClient.node_info().await?;
    println!("{}", node_infos);
    println!("Node version: {}", *node_infos.get("version").unwrap());

    //let network_query = DKGClient.network_query(String::from("query")).await?;

    let import_query = DKGClient
        .publish("~/Documents/example_dataset.json")
        .await?;
    Ok(())
}
