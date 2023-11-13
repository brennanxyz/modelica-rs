// Tests for practical use and for test-driven developemnt

// use modelica_rs::{ModelicaPackage};

use async_recursion::async_recursion;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use tempdir::TempDir;

use modelica_rs::{ModelicaPackage, ModelicaClass};

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

#[tokio::test]
async fn test_modelica_package() {
    dotenv().ok();
    let bearer = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let response = match client
        .get("https://raw.githubusercontent.com/modelica/ModelicaStandardLibrary/master/Modelica/Blocks/Continuous.mo")
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

    // println!("CONTINUOUS | {}", response.text().await.unwrap());
    let cont_text = response.text().await.unwrap();
    let mp = ModelicaPackage::extract(cont_text);
    println!("{:?}", mp);
}

#[tokio::test]
async fn test_modelica_double_package() {
    // println!("CONTINUOUS | {}", response.text().await.unwrap());
    let cont_text = "within Modelica.Blocks; \
    package ExampleOnePackage \"Library of continuous control blocks with internal states\" \
    import Modelica.Blocks.Interfaces; \
      extends Modelica.Icons.Package; \
      block Integrator \"Output the integral of the input signal with optional reset\" \
        import Modelica.Blocks.Types.Init; \
        parameter Real k=1 \"Integrator gain\"; \
        parameter Boolean use_reset = false \"= true, if reset port enabled\" \
          annotation(Evaluate=true, HideResult=true, choices(checkBox=true)); \
        parameter Boolean use_set = false \"= true, if set port enabled and used as reinitialization value when reset\" \
          annotation(Dialog(enable=use_reset), Evaluate=true, HideResult=true, choices(checkBox=true)); \
        /* InitialState is the default, because it was the default in Modelica 2.2 \
         and therefore this setting is backward compatible \
      */ \
        parameter Init initType=Init.InitialState \
          \"Type of initialization (1: no init, 2: steady state, 3,4: initial output)\" annotation(Evaluate=true, \
            Dialog(group=\"Initialization\")); \
        parameter Real y_start=0 \"Initial or guess value of output (= state)\" \
          annotation (Dialog(group=\"Initialization\")); \
        extends Interfaces.SISO(y(start=y_start)); \
        Modelica.Blocks.Interfaces.BooleanInput reset if use_reset \"Optional connector of reset signal\" annotation(Placement( \
          transformation( \
            extent={{-20,-20},{20,20}}, \
            rotation=90, \
            origin={60,-120}))); \
        Modelica.Blocks.Interfaces.RealInput set if use_reset and use_set \"Optional connector of set signal\" annotation(Placement( \
          transformation( \
            extent={{-20,-20},{20,20}}, \
            rotation=270, \
            origin={60,120}))); \
      protected \
        Modelica.Blocks.Interfaces.BooleanOutput local_reset annotation(HideResult=true); \
        Modelica.Blocks.Interfaces.RealOutput local_set annotation(HideResult=true); \
      initial equation \
        if initType == Init.SteadyState then \
           der(y) = 0; \
        elseif initType == Init.InitialState or \
               initType == Init.InitialOutput then \
          y = y_start; \
        end if; \
      equation \
        if use_reset then \
          connect(reset, local_reset); \
          if use_set then \
            connect(set, local_set); \
          else \
            local_set = y_start; \
          end if; \
          when local_reset then \
            reinit(y, local_set); \
          end when; \
        else \
          local_reset = false; \
          local_set = 0; \
        end if; \
        der(y) = k*u; \
        annotation ( \
          Documentation(info=\"<html> \
    <p> \
    This blocks computes output <strong>y</strong> as \
    <em>integral</em> of the input <strong>u</strong> multiplied with \
    the gain <em>k</em>: \
    </p> \
    <blockquote><pre> \
        k \
    y = - u \
        s \
    </pre></blockquote> \
    <p> \
    It might be difficult to initialize the integrator in steady state. \
    This is discussed in the description of package \
    </p> \
    <p> \
    If the <em>reset</em> port is enabled, then the output <strong>y</strong> is reset to <em>set</em> \
    or to <em>y_start</em> (if the <em>set</em> port is not enabled), whenever the <em>reset</em> \
    port has a rising edge. \
    </p> \
    </html>\"), Icon(coordinateSystem( \
                preserveAspectRatio=true, \
                extent={{-100.0,-100.0},{100.0,100.0}}), \
              graphics={ \
                Line( \
                  points={{-80.0,78.0},{-80.0,-90.0}}, \
                  color={192,192,192}), \
                Polygon( \
                  lineColor={192,192,192}, \
                  fillColor={192,192,192}, \
                  fillPattern=FillPattern.Solid, \
                  points={{-80.0,90.0},{-88.0,68.0},{-72.0,68.0},{-80.0,90.0}}), \
                Line( \
                  points={{-90.0,-80.0},{82.0,-80.0}}, \
                  color={192,192,192}), \
                Polygon( \
                  lineColor={192,192,192}, \
                  fillColor={192,192,192}, \
                  fillPattern=FillPattern.Solid, \
                  points={{90.0,-80.0},{68.0,-72.0},{68.0,-88.0},{90.0,-80.0}}), \
                Text( \
                  textColor={192,192,192}, \
                  extent={{0.0,-70.0},{60.0,-10.0}}, \
                  textString=\"I\"), \
                Text( \
                  extent={{-150.0,-150.0},{150.0,-110.0}}, \
                  textString=\"k=%k\"), \
                Line( \
                  points=DynamicSelect({{-80.0,-80.0},{80.0,80.0}}, if use_reset then {{-80.0,-80.0},{60.0,60.0},{60.0,-80.0},{80.0,-60.0}} else {{-80.0,-80.0},{80.0,80.0}}), \
                  color={0,0,127}), \
                Line( \
                  visible=use_reset, \
                  points={{60,-100},{60,-80}}, \
                  color={255,0,255}, \
                  pattern=LinePattern.Dot), \
                Text( \
                  visible=use_reset, \
                  extent={{-28,-62},{94,-86}}, \
                  textString=\"reset\")})); \
      end Integrator; \
    end ExampleOnePackage; \
    
    package ExampleTwoPackage \"Library of continuous control blocks with internal states\" \
    import Modelica.Blocks.Interfaces; \
      extends Modelica.Icons.Package; \
      block Integrator \"Output the integral of the input signal with optional reset\" \
        import Modelica.Blocks.Types.Init; \
        parameter Real k=1 \"Integrator gain\"; \
        parameter Boolean use_reset = false \"= true, if reset port enabled\" \
          annotation(Evaluate=true, HideResult=true, choices(checkBox=true)); \
        parameter Boolean use_set = false \"= true, if set port enabled and used as reinitialization value when reset\" \
          annotation(Dialog(enable=use_reset), Evaluate=true, HideResult=true, choices(checkBox=true)); \
        /* InitialState is the default, because it was the default in Modelica 2.2 \
         and therefore this setting is backward compatible \
      */ \
        parameter Init initType=Init.InitialState \
          \"Type of initialization (1: no init, 2: steady state, 3,4: initial output)\" annotation(Evaluate=true, \
            Dialog(group=\"Initialization\")); \
        parameter Real y_start=0 \"Initial or guess value of output (= state)\" \
          annotation (Dialog(group=\"Initialization\")); \
        extends Interfaces.SISO(y(start=y_start)); \
        Modelica.Blocks.Interfaces.BooleanInput reset if use_reset \"Optional connector of reset signal\" annotation(Placement( \
          transformation( \
            extent={{-20,-20},{20,20}}, \
            rotation=90, \
            origin={60,-120}))); \
        Modelica.Blocks.Interfaces.RealInput set if use_reset and use_set \"Optional connector of set signal\" annotation(Placement( \
          transformation( \
            extent={{-20,-20},{20,20}}, \
            rotation=270, \
            origin={60,120}))); \
      protected \
        Modelica.Blocks.Interfaces.BooleanOutput local_reset annotation(HideResult=true); \
        Modelica.Blocks.Interfaces.RealOutput local_set annotation(HideResult=true); \
      initial equation \
        if initType == Init.SteadyState then \
           der(y) = 0; \
        elseif initType == Init.InitialState or \
               initType == Init.InitialOutput then \
          y = y_start; \
        end if; \
      equation \
        if use_reset then \
          connect(reset, local_reset); \
          if use_set then \
            connect(set, local_set); \
          else \
            local_set = y_start; \
          end if; \
          when local_reset then \
            reinit(y, local_set); \
          end when; \
        else \
          local_reset = false; \
          local_set = 0; \
        end if; \
        der(y) = k*u; \
        annotation ( \
          Documentation(info=\"<html> \
    <p> \
    This blocks computes output <strong>y</strong> as \
    <em>integral</em> of the input <strong>u</strong> multiplied with \
    the gain <em>k</em>: \
    </p> \
    <blockquote><pre> \
        k \
    y = - u \
        s \
    </pre></blockquote> \
    <p> \
    It might be difficult to initialize the integrator in steady state. \
    This is discussed in the description of package \
    </p> \
    <p> \
    If the <em>reset</em> port is enabled, then the output <strong>y</strong> is reset to <em>set</em> \
    or to <em>y_start</em> (if the <em>set</em> port is not enabled), whenever the <em>reset</em> \
    port has a rising edge. \
    </p> \
    </html>\"), Icon(coordinateSystem( \
                preserveAspectRatio=true, \
                extent={{-100.0,-100.0},{100.0,100.0}}), \
              graphics={ \
                Line( \
                  points={{-80.0,78.0},{-80.0,-90.0}}, \
                  color={192,192,192}), \
                Polygon( \
                  lineColor={192,192,192}, \
                  fillColor={192,192,192}, \
                  fillPattern=FillPattern.Solid, \
                  points={{-80.0,90.0},{-88.0,68.0},{-72.0,68.0},{-80.0,90.0}}), \
                Line( \
                  points={{-90.0,-80.0},{82.0,-80.0}}, \
                  color={192,192,192}), \
                Polygon( \
                  lineColor={192,192,192}, \
                  fillColor={192,192,192}, \
                  fillPattern=FillPattern.Solid, \
                  points={{90.0,-80.0},{68.0,-72.0},{68.0,-88.0},{90.0,-80.0}}), \
                Text( \
                  textColor={192,192,192}, \
                  extent={{0.0,-70.0},{60.0,-10.0}}, \
                  textString=\"I\"), \
                Text( \
                  extent={{-150.0,-150.0},{150.0,-110.0}}, \
                  textString=\"k=%k\"), \
                Line( \
                  points=DynamicSelect({{-80.0,-80.0},{80.0,80.0}}, if use_reset then {{-80.0,-80.0},{60.0,60.0},{60.0,-80.0},{80.0,-60.0}} else {{-80.0,-80.0},{80.0,80.0}}), \
                  color={0,0,127}), \
                Line( \
                  visible=use_reset, \
                  points={{60,-100},{60,-80}}, \
                  color={255,0,255}, \
                  pattern=LinePattern.Dot), \
                Text( \
                  visible=use_reset, \
                  extent={{-28,-62},{94,-86}}, \
                  textString=\"reset\")})); \
      end Integrator; \
    end ExampleTwoPackage;".to_string();
    let mp = ModelicaPackage::extract(cont_text);
    println!("{:?}", mp);
}



