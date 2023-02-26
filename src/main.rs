use lcu::client;

fn main() {
    let c = client::Client::new();
    println!("{c:?}");
}
