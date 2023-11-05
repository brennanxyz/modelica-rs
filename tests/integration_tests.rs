// Tests for practical use and for test-driven developemnt

// use modelica_rs::{ModelicaPackage};
use reqwest::Client;

#[cfg(test)]

#[test]
fn free_test() {
    assert_eq!(2 + 2, 4);
}

#[tokio::test]
async fn repo_connection() {
    let client = Client::new();
    match client.get("https://github.com/modelica/ModelicaStandardLibrary").send().await {
        Ok(response) => {
            assert_eq!(response.status(), 200);
        },
        Err(_) => {
            assert!(false);
        }
    }
}



