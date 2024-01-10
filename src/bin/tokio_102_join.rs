use tokio::time::Duration;

async fn sleep_then_print(timer: u32) {
    println!("Start timer {}", timer);
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Stop timer {}", timer);
}

#[tokio::main]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(sleep_then_print(1)),
        tokio::spawn(sleep_then_print(2)),
        tokio::spawn(sleep_then_print(3)),
    );
}
