pub mod merge_impl;

use std::{collections::BTreeMap, path::PathBuf};

use merge_impl::merge_impl;
use openapiv3::{
    ExternalDocumentation, IndexMap, Info, PathItem, Paths, SecurityRequirement, Server, Tag, v2,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Encoding, MergeArgs, utils::to_v2};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMap {
    pub openapi: Option<String>,
    pub info: Option<Info>,
    #[serde(default)]
    pub servers: Vec<Server>,
    #[serde(default)]
    pub security: Vec<SecurityRequirement>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(default)]
    pub extensions: IndexMap<String, Value>,
    #[serde(default)]
    pub paths: Paths,
    pub sources: BTreeMap<PathBuf, Option<PathItem>>,
}

pub fn exec(args: MergeArgs) -> Option<String> {
    let map = SourceMap::from(&args);
    let spec = merge_impl(&args, map);

    if args.use_version_2 {
        let spec = to_v2(spec);
        let spec_content = match &args.output_format {
            Some(Encoding::Yaml) => serde_yaml::to_string(&spec).unwrap(),
            Some(Encoding::Yml) => serde_json::to_string(&spec).unwrap(),
            _ => serde_json::to_string_pretty(&spec).unwrap(),
        };

        if let Some(path) = &args.output {
            std::fs::write(path, spec_content).unwrap();
            return None;
        }

        return Some(spec_content);
    }

    let spec_content = match &args.output_format {
        Some(Encoding::Yaml) => serde_yaml::to_string(&spec).unwrap(),
        Some(Encoding::Yml) => serde_json::to_string(&spec).unwrap(),
        _ => serde_json::to_string_pretty(&spec).unwrap(),
    };

    if let Some(path) = &args.output {
        std::fs::write(path, spec_content).unwrap();
        return None;
    }

    Some(spec_content)
}

impl SourceMap {
    pub fn from(args: &MergeArgs) -> Self {
        #[allow(unused_assignments)]
        let mut encoding: Option<Encoding> = None;
        let mut source = if let Some(path) = &args.file {
            encoding = match path.extension().and_then(|ext| ext.to_str()) {
                Some("json") => Some(Encoding::Json),
                Some("yaml") => Some(Encoding::Yaml),
                Some("yml") => Some(Encoding::Yaml),
                _ => None,
            };
            eprintln!("path: {:?}", path);
            std::fs::read_to_string(path).unwrap()
        } else if let Some(source) = &args.json {
            encoding = Some(Encoding::Json);
            source.clone()
        } else if let Some(source) = &args.yaml {
            encoding = Some(Encoding::Yaml);
            source.clone()
        } else {
            panic!("no source provided.\nfile, json or yaml argument must be provided");
        };

        if let Some(encode) = args.encoding {
            encoding = Some(encode);
        }

        if args.verbose {
            eprintln!("Source retrieved:\n{}", source);
        }

        for var in &args.template_vars {
            source = source.replace(&format!("${{{}}}", &var.key), &var.value);
        }

        if args.verbose {
            eprintln!("Tamplating complete:\n{}", source);
        }

        let map = match encoding {
            Some(Encoding::Json) => serde_json::from_str(&source).unwrap(),
            Some(Encoding::Yaml) => serde_yaml::from_str(&source).unwrap(),
            Some(Encoding::Yml) => serde_yaml::from_str(&source).unwrap(),
            _ => {
                if let Ok(map) = serde_json::from_str(&source) {
                    map
                } else {
                    serde_yaml::from_str(&source).unwrap()
                }
            }
        };

        map
    }
}
