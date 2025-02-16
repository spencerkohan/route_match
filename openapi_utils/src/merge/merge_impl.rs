use std::path::PathBuf;

use openapiv3::{OpenAPI, PathItem, RefOr};

use crate::MergeArgs;

use super::SourceMap;

pub fn merge_impl(args: &MergeArgs, map: SourceMap) -> OpenAPI {
    let mut result = OpenAPI::default();
    merge_source_map(&mut result, &map);
    let sources = &map.sources;

    for (source, overrides) in sources {
        let spec = get_spec(&args, source, overrides);
        result = result.merge(spec).unwrap();
    }

    result
}

pub fn get_spec(args: &MergeArgs, source: &PathBuf, overrides: &Option<PathItem>) -> OpenAPI {
    let mut spec = get_source_spec(args, &source);

    if let Some(overrides) = overrides {
        for (_, item) in spec.paths.iter_mut() {
            if let RefOr::Item(item) = item {
                merge_path_item(item, &overrides);
            }
        }
    }

    spec
}

pub fn get_source_spec(args: &MergeArgs, source: &PathBuf) -> OpenAPI {
    let path = args.relative_path(&source);
    let source_content = std::fs::read_to_string(&path).unwrap();
    let spec: OpenAPI = match source.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::from_str(&source_content).unwrap(),
        Some("yml") => serde_yaml::from_str(&source_content).unwrap(),
        Some("yaml") => serde_yaml::from_str(&source_content).unwrap(),
        _ => {
            if let Ok(spec) = serde_json::from_str(&source_content) {
                spec
            } else {
                serde_yaml::from_str(&source_content).unwrap()
            }
        }
    };
    spec
}

pub fn merge_source_map(spec: &mut OpenAPI, map: &SourceMap) {
    if let Some(version) = &map.openapi {
        spec.openapi = version.clone();
    }
    if let Some(info) = &map.info {
        spec.info = info.clone();
    }
    if !map.servers.is_empty() {
        spec.servers = map.servers.clone();
    }
    if !map.security.is_empty() {
        spec.security = map.security.clone();
    }
    if !map.tags.is_empty() {
        spec.tags = map.tags.clone();
    }
    if let Some(external_docs) = &map.external_docs {
        spec.external_docs = Some(external_docs.clone());
    }
    if !map.extensions.is_empty() {
        spec.extensions = map.extensions.clone();
    }
    spec.paths = map.paths.clone();
}

pub fn merge_path_item(item: &mut PathItem, overrides: &PathItem) {
    if let Some(summary) = &overrides.summary {
        item.summary = Some(summary.clone());
    }
    if let Some(description) = &overrides.description {
        item.description = Some(description.clone());
    }
    if let Some(get) = &overrides.get {
        item.get = Some(get.clone());
    }
    if let Some(put) = &overrides.put {
        item.put = Some(put.clone());
    }
    if let Some(post) = &overrides.post {
        item.post = Some(post.clone());
    }
    if let Some(delete) = &overrides.delete {
        item.delete = Some(delete.clone());
    }
    if let Some(options) = &overrides.options {
        item.options = Some(options.clone());
    }
    if let Some(head) = &overrides.head {
        item.head = Some(head.clone());
    }
    if let Some(patch) = &overrides.patch {
        item.patch = Some(patch.clone());
    }
    if let Some(trace) = &overrides.trace {
        item.trace = Some(trace.clone());
    }
    if !overrides.servers.is_empty() {
        item.servers = overrides.servers.clone();
    }
    if !overrides.parameters.is_empty() {
        item.parameters = overrides.parameters.clone();
    }
    if !overrides.extensions.is_empty() {
        item.extensions = overrides.extensions.clone();
    }
}
