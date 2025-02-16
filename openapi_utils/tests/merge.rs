use std::path::Path;

use openapi_utils::{Encoding, MergeArgs, TemplateVar, merge::exec};
use openapiv3::OpenAPI;

#[test]
pub fn test_merge() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_dir_path = Path::new(manifest_dir);
    eprintln!("manifest_dir_path: {:?}", manifest_dir_path);

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
    };

    let result = exec(args).unwrap();

    println!("result:\n{}", result);

    let _: OpenAPI = serde_yaml::from_str(&result).unwrap();
}
