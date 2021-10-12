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

    let tquery = r#"
        {
            "identifier_types": ["id"],
            "identifier_values": ["test1"],
            "depth": 10
        }
    "#;

    let trail_query = dkg_client.trail(String::from(tquery)).await?;

    println!("Trail result:\n{}", trail_query);
    println!("Id of object is: {}", trail_query[0]["otObject"]["@id"]);

    Ok(())
}
