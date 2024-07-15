use std::time::Duration;

pub async fn fixtures() -> tokio::sync::mpsc::Sender<bool> {
    // start the test HTTP server in a background task
    let tx = quest_tests::start_test_server().await.unwrap();

    // sleep a bit to ensure the test server is up and running
    // note: for some reason, sleeping with tokio blocks all tests..
    std::thread::sleep(Duration::from_millis(100));

    tx
}
