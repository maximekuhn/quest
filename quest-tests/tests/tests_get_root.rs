use quest_core::config::Config;

mod common;

#[test]
fn test_get_root_should_return_hello_world() {
    common::fixutres();

    let config = Config {
        http_file_path: "./requests/get_root.http".into(),
        env_var_file: None,
    };
    let output = quest_core::run(config).unwrap();

    let result = output.results.first().unwrap();
    assert!(result
        .response
        .as_ref()
        .unwrap()
        .body
        .as_ref()
        .unwrap()
        .contains("hello world"));
}
