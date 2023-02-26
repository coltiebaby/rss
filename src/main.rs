use lcu::client::RiotClient;
// use std::{path, fs};
// use std::sync::mpsc;

#[tokio::main]
async fn main() {
    let rc = RiotClient::new();

    let url = rc.get_url_builder().join("/swagger/v3/openapi.json").unwrap();
    let res = rc.client.get(url).send().await.expect("failed to make request");
    let body = res.text().await.expect("text");
    println!("{}", body);


    // let u = format!("https://localhost:{port}/swagger/v1/api-docs", port=client.port);
    // println!("{}", u);
    // let resp = c.get(u).basic_auth("riot", Some(client.token))
    //     .send()
    //     .await
    //     .expect("send");;

    // eprintln!("Response: {:?} {}", resp.version(), resp.status());
    // eprintln!("Headers: {:#?}\n", resp.headers());

    // let body = resp.text().await.expect("text");

    // println!("{}", body);

}
