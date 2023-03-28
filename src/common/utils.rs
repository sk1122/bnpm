use std::path::Path;

use super::types::{ProjectRoot, Package};

pub fn get_project_root_path() -> Option<ProjectRoot> {
    Some(ProjectRoot { name: "aptos-wallet-api".to_string(), path: "/home/cosmix/Desktop/code/bnpm/node_modules".into(), package: Package { name: "aptos-wallet-api".into(), version: "0,3,3".into() } })
}