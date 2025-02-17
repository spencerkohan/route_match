use openapiv3::{IndexMap, OpenAPI, Paths, v2};
use serde_json;

pub fn to_v2(spec: OpenAPI) -> v2::OpenAPI {
    let mut result = v2::OpenAPI::default();

    result.swagger = "2.0".to_string();
    result.info = serde_json::from_str(&serde_json::to_string(&spec.info).unwrap()).unwrap();
    result.paths = serde_json::from_str(&serde_json::to_string(&spec.paths).unwrap()).unwrap();
    result.security =
        serde_json::from_str(&serde_json::to_string(&spec.security).unwrap()).unwrap();
    result.extensions = spec.extensions.clone();

    result
}
