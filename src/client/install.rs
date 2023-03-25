use serde_json::Value;
use crate::{common::types::Package, tree::Node};

pub fn get_installed_packages() -> Option<Vec<Package>> {
    let file_contents = std::fs::read_to_string("/home/cosmix/.bnpm/installed.json").unwrap();
    let file_content_to_struct: Value = serde_json::from_str(file_contents.as_str()).unwrap();

    let installed_packages = &file_content_to_struct["packages"];

    match installed_packages {
        Value::Array(packages) => {
            let p = packages.into_iter().map(|x| {
                let g: Package = serde_json::from_value(x.to_owned()).unwrap();

                return g;
            }).collect::<Vec<Package>>();

            return Some(p);
        },
        _ => None
    }
}

pub fn need_to_install(packages: &Vec<Node>) -> Option<(Vec<Package>, Vec<Package>)> {
    let installed_packages = get_installed_packages().unwrap();
    
    let mut need_to_be_installed: Vec<Package> = vec![];
    let mut need_not_be_installed: Vec<Package> = vec![];

    for package in packages {
        if installed_packages.contains(&package.package) {
            need_not_be_installed.push(package.package.clone());
        } else {
            need_to_be_installed.push(package.package.clone());
        }
    }

    Some((need_to_be_installed, need_not_be_installed))
}