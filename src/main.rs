use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for i in 1..1000 {
        let builder = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(60 * 3))
            .pool_max_idle_per_host(1);
        let client = builder.build().unwrap();
        handles.push(tokio::task::spawn(async move {
            let _ = client.get(format!("https://{}.xyz", i)).send().await;
            println!("{}", i);
        }));
    }
    for h in handles {
        let _ = h.await;
    }
    println!("done. measure memory usage now and press enter to stop.");
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
}
