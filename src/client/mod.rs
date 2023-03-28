use crate::common::types::Package;

pub fn resolve_package_version(package: &String) -> Option<Package> {
    if package.contains("@") {
        let package_splits = package.split("@").collect::<Vec<&str>>();

        let package_name  = package_splits[0];
        let package_version = package_splits[1];

        return Some(Package {
            name: package_name.into(),
            version: package_version.into()
        });
    }

    None
}

pub fn resolve_package_versions(packages_string: &Vec<String>) -> Option<Vec<Package>> {
    let mut packages: Vec<Package> = vec![];

    for package_string in packages_string {
        let package = resolve_package_version(package_string)?;

        packages.push(package)
    }

    Some(packages)
}

pub fn get_parent_package() -> Option<Package> {
    Some(Package {
        name: "aptos-wallet-api".into(),
        version: "0.3.3".into()
    })
}

pub fn find_package_in_root(package: &Package) -> bool {
    let boolean = std::fs::read_dir(format!("~/.bnpm/{}/{}", package.name, package.version)).and_then(|x| Ok(x));

    if let Ok(_x) = boolean {
        return true
    } else {
        return false
    }
}

pub mod install;
pub mod download;
pub mod resolve;