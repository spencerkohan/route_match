use std::path::Path;

use openapi_tools::{Encoding, MergeArgs, TemplateVar, merge::exec};
use openapiv3::{OpenAPI, v2};

#[test]
pub fn test_merge() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir_path = Path::new(manifest_dir);
    println!("manifest_dir_path: {:?}", manifest_dir_path);

    let working_dir = Path::new(manifest_dir).join("tests");
    let file_path = Path::new(manifest_dir).join("tests/openapi.template.yaml");

    let args = MergeArgs {
        file: Some(file_path.into()),
        json: None,
        yaml: None,
        encoding: None,
        template_vars: vec![TemplateVar {
            key: "addr".to_string(),
            value: "https://url-of.func.com".to_string(),
        }],
        output: None,
        output_format: Some(Encoding::Yaml),
        working_directory: Some(working_dir.into()),
        verbose: false,
        use_version_2: false,
    };

    let result = exec(args).unwrap();

    println!("result:\n{}", result);

    let _: OpenAPI = serde_yaml::from_str(&result).unwrap();
}

#[test]
pub fn test_merge_to_v2() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir_path = Path::new(manifest_dir);
    println!("manifest_dir_path: {:?}", manifest_dir_path);

    let working_dir = Path::new(manifest_dir).join("tests");
    let file_path = Path::new(manifest_dir).join("tests/openapi.template.yaml");

    let args = MergeArgs {
        file: Some(file_path.into()),
        json: None,
        yaml: None,
        encoding: None,
        template_vars: vec![TemplateVar {
            key: "addr".to_string(),
            value: "https://url-of.func.com".to_string(),
        }],
        output: None,
        output_format: Some(Encoding::Yaml),
        working_directory: Some(working_dir.into()),
        verbose: false,
        use_version_2: true,
    };

    let result = exec(args).unwrap();

    println!("result:\n{}", result);

    let _: v2::OpenAPI = serde_yaml::from_str(&result).unwrap();
}
