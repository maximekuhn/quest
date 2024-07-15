use std::time::Duration;

use assert_cmd::Command;
use serial_test::serial;

mod common;

// FIXME: for some reason, the server doesn't seem to be up and running
// and the test fails with the command timeout (interrupted)

//#[tokio::test]
#[serial]
async fn test_get_root_should_return_hello_world_for_all_requests() {
    let shutdown_ch = common::fixtures().await;

    let mut cmd = Command::cargo_bin("quest").unwrap();
    cmd.arg("--file").arg("./requests/get_root.http");
    cmd.timeout(Duration::from_secs(1));

    // run the command and wait for the output
    let cmd_assert = cmd.assert();

    // gracefully shutdown the server
    shutdown_ch.send(true).await.unwrap();

    cmd_assert.success();

    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    assert!(output.contains("Hello world"));
}
