// Tests for practical use and for test-driven developemnt

// use modelica_rs::{ModelicaPackage};

use async_recursion::async_recursion;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use tempdir::TempDir;

#[derive(Debug)]
struct ModelicaDirectory {
    name: String,
    path: String,
    directories: Vec<ModelicaDirectory>,
    files: Vec<ModelicaFile>,
}

#[derive(Debug)]
struct ModelicaFile {
    name: String,
    download_url: String,
}

#[derive(Deserialize, Debug)]
struct ModelicaRepoEntry {
    name: String,
    path: String,
    download_url: Option<String>,
    r#type: String,
}

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

#[test]
fn env_var_access() {
    dotenv().ok();
    match env::var("GITHUB_TOKEN") {
        Ok(_) => {
            assert!(true);
        },
        Err(_) => {
            assert!(false);
        }
    };
}

#[tokio::test]
async fn raw_repo_connection() {
    dotenv().ok();
    let bearer = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    match client
        .get("https://api.github.com/repos/modelica/ModelicaStandardLibrary/contents/Modelica")
        .header("User-Agent", "My Rust Program 1.0")
        .header("Authorization", format!("Bearer {}", bearer))
        .send().await {
        Ok(response) => {
            assert_eq!(response.status(), 200);
        },
        Err(_) => {
            assert!(false);
        }
    }
}

#[test]
fn tempdir_make() {
    let tdir = match TempDir::new("modelica-rs") {
        Ok(tdir) => {
            tdir
        },
        Err(_) => {
            assert!(false);
            return;
        }
    };

    match tdir.close() {
        Ok(_) => {
            assert!(true)
        },
        Err(_) => {
            assert!(false)
        }
    }
}

#[tokio::test]
async fn repo_download() {
    let tdir = match TempDir::new("modelica-rs") {
        Ok(tdir) => {
            tdir
        },
        Err(_) => {
            assert!(false);
            return;
        }
    };

    let url = "https://api.github.com/repos/modelica/ModelicaStandardLibrary/contents";

    let mut m_d = ModelicaDirectory {
        name: "Blocks".to_string(),
        path: "Modelica/Blocks".to_string(),
        directories: Vec::new(),
        files: Vec::new(),
    };

    let m_d = recur_through_repo(url, m_d).await;

    println!("{:?}", m_d);
    
}

#[async_recursion]
async fn recur_through_repo(base: &str, mut mod_dir: ModelicaDirectory) -> ModelicaDirectory {
    dotenv().ok();
    let bearer = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    println!("Recursing through {}", mod_dir.path);
    let client = Client::new();
    let response = match client
        .get(format!("{}/{}", base, mod_dir.path))
        .header("User-Agent", "My Rust Program 1.0")
        .header("Authorization", format!("Bearer {}", bearer))
        .send().await {
        Ok(response) => {
            response
        },
        Err(_) => {
            assert!(false);
            panic!();
        }
    };

    let entries = match response.json::<Vec<ModelicaRepoEntry>>().await {
        Ok(entries) => {
            entries
        },
        Err(_) => {
            vec![]
        }
    };

    for entry in entries {
        if entry.r#type == "dir" {
            let n_m_d = ModelicaDirectory {
                name: entry.name.clone(),
                path: entry.path,
                directories: Vec::new(),
                files: Vec::new(),
            };
            if entry.name == "Resources" {
                continue;
            }
            mod_dir.directories.push(recur_through_repo(base, n_m_d).await);
        } else if entry.r#type == "file" {
            mod_dir.files.push(ModelicaFile {
                name: entry.path,
                download_url: entry.download_url.expect("No download url"),
            });
        }
    }
    return mod_dir
}



