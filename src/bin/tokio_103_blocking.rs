#[tokio::main]
async fn main() {
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("Inside spawn blocking");
    });

    blocking_task.await.unwrap();
}
