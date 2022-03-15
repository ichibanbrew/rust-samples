use tokio::{sync::mpsc, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx_task_1_to_2, mut rx_0) = mpsc::channel(2);
    let (tx_task_2_to_3, mut rx_1) = mpsc::channel(2);

    let task_1 = tokio::spawn(async move {
        for i in 0..3 {
            println!("\nTask 1, i = {}", i);
            if tx_task_1_to_2.send(i).await.is_err() {
                panic!("Task 1: Could not send to channel");
            };

            time::sleep(time::Duration::from_secs(2)).await;
        }
        println!("(Task 1 exiting.)")
    });
    let task_2 = tokio::spawn(async move {
        while let Some(msg) = rx_0.recv().await {
            println!("Task 2: got message {}, sending to Task 3", msg);
            if tx_task_2_to_3.send(msg * 10).await.is_err() {
                panic!("Task 2: Could not send to channel");
            };
        }
        println!("(Task 2 exiting.)")
    });
    let task_3 = tokio::spawn(async move {
        while let Some(msg) = rx_1.recv().await {
            println!("Task 3: got message {}", msg);
        }
        println!("(Task 3 exiting.)")
    });

    println!("(Now awaiting for all things to complete before exiting. \nIf don't await we just exit and all tasks are killed without getting a chance to complete.)");
    task_1.await.unwrap();
    println!("(awaited Task 1)");
    task_2.await.unwrap();
    println!("(awaited Task 2)");
    task_3.await.unwrap();
    println!("(awaited Task 3)");

    println!("Done. Exiting...");
    Ok(())
}
