![](https://i.imgur.com/XkISdML.png)

# Rust DKG Client

**Rust library for interaction with the OriginTrail Decentralized Knowledge Graph**

**Note**: This library is currently in alpha, so you can expect issues to arise. We'd appreciate that if you do run into trouble, you [open up an issue on this repository](https://github.com/OriginTrail/dkg-client/issues) and let us know. 

The official OriginTrail documentation can be found [here](https://docs.origintrail.io/en/latest/).

## Intro - What is a Decentralized Knowledge Graph (DKG)


There are many avaialable definitions of a knowlege graph, therefore we will present a simplified one focused on usability, rather than completeness. The purpose of this introduction is not to be a comprehensive guide for knowledge graphs, however it aims to get you started with the basics.

A **knowledge graph (KG)** is a network of entities — physical & digital objects, events or concepts — illustrating the relationship between them (aka a semantic network). KGs are used by major companies such as [Amazon](http://lunadong.com/talks/PG.pdf), [Google](https://en.wikipedia.org/wiki/Google_Knowledge_Graph), [Uber](https://www.youtube.com/watch?v=r3yMSl5NB_Q), [IBM](https://www.ibm.com/cloud/learn/knowledge-graph) etc for various applications: search, data integration, knowledge reasoning, recommendation engines, analytics, machine learning and AI etc.

Key characteristics of knowledge graphs:
* focus on data connections as "first class citizens" (linked data) 
* designed to ingest data from multiple sources, usually in different formats
* flexible data model, easily extendable

Common knowledge graphs however are deployed within the domain of one organization and are designed to capture knowledge from various sources both from within and outside of the organization.

We define **decentralized knowledge graph (DKG)** as a global shared knowledge graph that is designed to benefit organizations and individuals by providing a common infrastructure for data exchange. The DKG:

* Enables Dapps with search, integration, analytics, AI and ML capabilities for any data source: blockchains, IPFS, enterprise systems, web services, personal devices 
* Removes central authorities (decentralized infrastructure)
* Enables permissionless PUBLISH and QUERY (public network)
* Decentralized identity & Verifiable Credentials based access control (references private data)

## The OriginTrail DKG Architecture 

The OriginTrail Decentralized Network implements the DKG according the the OriginTrail protocol.

It is:

* **a permissionless network** - anyone can run OriginTrail nodes
* **a multi-chain data exchange network** - connects to several blockchains (currently Ethereum and xDai with more integrations upcoming such as with Polkadot)
* **designed for off-chain data exchange using standardized data models** (GS1 & W3C standards and recommendations)
* **public open source software**
* **infrastructure for knowledge marketplaces & tenders** - more info [here](https://www.youtube.com/watch?v=4uCxYGRh5fk)

More information is available on the OriginTrail [website](https://origintrail.io), [official documentation](https://docs.origintrail.io) and [blog](https://medium.com/origintrail).


![](https://i.imgur.com/yTNtZE1.png)

## Getting Started

### Start local DKG network

To start local DKG network on your machine you should follow instructions on this [link](https://docs.origintrail.io/developers/setting-up-development-environment)

### Start the Client

Build:
```
cd lib-test && cargo build
```

Run tests:
```
./target/debug/dkg-client-rust-test  
```

Initializing Rust DKG client:

```rust
let dkg_client = DkgClient {
        http_client: Client::new(),
        endpoint: String::from("http://0.0.0.0:8900/"),
    };
```

To get node info, publish data, perform network query or trail just 
paste some of the code snippets below in the lib-test/src/main.rs:

Node info:

```rust
let node_infos = dkg_client.node_info().await?;

println!("Node info struct: {}", node_infos);
println!("Node version: {}", node_infos["version"]);
```

Publishing data - we are publishing dataset.json that we have in the repo:

```rust
let publishing_reponse = dkg_client.publish("../dataset.json").await?;

println!(
    "Status for our dataset is: {}",
    publishing_reponse["status"].as_str().unwrap()
);
```

Network query:

```rust
let query = r#"
    {
        "query" : [{
            "path": "id",
            "value": "test1",
            "opcode": "EQ"
        }]
    }
"#;

let network_query = dkg_client.network_query(String::from(query)).await?;

println!(
    "Data set id is: {}",
    network_query[0]["datasets"][0]["data_set_id"]
);
```

Trail:
```rust
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
```

## Learn More

- More about [OriginTrail](https://origintrail.io/)
- More about [decentralized knowledge graph](https://origintrail.io/technology)
- Video explainer about [the world’s first Decentralized Knowledge Graph](https://www.youtube.com/watch?v=AsCUigu39Hw&ab_channel=OriginTrail)