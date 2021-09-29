extern crate dkg_client_rust;
use dkg_client_rust::client;

fn main() {
    let resp = dkg_client_rust::client::resolve("https://mainnet.ot-node.com/", "id").unwrap();
    println!("{:#?}", resp);

    let resp = dkg_client_rust::client::discover("https://mainnet.ot-node.com/", "topics").unwrap();
    println!("{:#?}", resp);

    let resp = dkg_client_rust::client::query("https://mainnet.ot-node.com/", "topics", "query").unwrap();
    println!("{:#?}", resp);
}
