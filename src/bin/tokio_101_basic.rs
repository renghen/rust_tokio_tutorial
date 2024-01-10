use std::time::Duration;

#[tokio::main]
async fn main() {
    print!("hello ");
    std::thread::sleep(Duration::from_secs(5));
    println!("world after 5 seconds");
}
