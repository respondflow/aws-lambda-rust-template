// Build script for AWS Rust stack

extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};

use std::env;
use std::fs;
use std::path::Path;

// This is set in Makefile to the contents of template.yaml
const TEMPLATE_YAML: &'static str = env!("TEMPLATE_YAML");

// The name of the AWS resource (Lambda) being compiled
const AWS_RESOURCE_NAME: &'static str = env!("AWS_RESOURCE_NAME");

// Filename of Rust main.rs template, to be transformed
const MAIN_RS: &'static str = "main.rs.template";

// Filename for the transformed main.rs template, should match main.rs
const TRANSFORMED_MAIN_RS: &'static str = "main.include.rs";

/// Execute pre-compilation steps for a Rust Lambda
fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&manifest_dir).join("src").join(MAIN_RS);
    let original_src = fs::read_to_string(src_path).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(TRANSFORMED_MAIN_RS);

    let handler_name = get_handler_name();
    let rust_handler_to_replace = "/*{{RF_LAMBDA_HANDLER}}*/ handlers::default_handler";

    fs::write(
        &dest_path,
        original_src.replace(rust_handler_to_replace, &handler_name)
    ).unwrap();

    // Re-run build script if environment variables change
    println!("cargo:rerun-if-env-changed=AWS_RESOURCE_NAME");
    println!("cargo:rerun-if-env-changed=TEMPLATE_YAML");
}

/// Extract the name of the Rust handler from template.yaml
fn get_handler_name() -> String {
    let yaml_docs = YamlLoader::load_from_str(TEMPLATE_YAML).unwrap();
    let template = &yaml_docs[0].as_hash().unwrap();
    let resources = template.get(&Yaml::from_str("Resources")).unwrap().as_hash().unwrap();
    let current_resource = resources.get(&Yaml::from_str(AWS_RESOURCE_NAME)).unwrap().as_hash().unwrap();
    let properties = current_resource.get(&Yaml::from_str("Properties")).unwrap().as_hash().unwrap();
    let handler_name = properties.get(&Yaml::from_str("Handler")).unwrap().as_str().unwrap();

    String::from(handler_name)
}
