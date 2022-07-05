use rlt_client::open_tunnel;

#[tokio::main]
async fn main() {
    println!("Run localtunnel CLI!");

    let result = open_tunnel("http://proxy.ad4m.dev", Some("demo")).await;

    println!("result: {:?}", result);
}
