use std::{thread::sleep, time::Duration};

pub fn fixutres() {
    // start test server
    std::thread::spawn(start_server);

    // wait a bit to ensure the server is up and running
    sleep(Duration::from_millis(100));
}

fn start_server() {
    quest_tests::start_test_server();
}
