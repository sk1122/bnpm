use std::io::BufReader;
use flate2::bufread::GzDecoder;
use tar::Archive;
use serde_json::Value;

use crate::common::types::Package;

use super::find_package_in_root;

pub fn get_dependency_of_package(package: &Package) -> Option<Vec<Package>> {
    let url = format!("https://registry.npmjs.org/{}", package.name);
    
    let res = reqwest::blocking::get(url).unwrap();

    let value = res.json::<Value>().unwrap();

    let mut version = package.version.clone();

    if version.chars().nth(0) == Some('^') || version.chars().nth(0) == Some('~') {
        version = version[1..].to_string();
    }
    println!("{}", version);

    let dependencies_value = &value["versions"][version.to_owned()]["dependencies"];

    println!("{:?}", dependencies_value);

    let mut dependencies = vec![];

    match dependencies_value {
        Value::Object(package) => {
            for x in package.iter() {
                if let Value::String(s) = x.1 {
                    dependencies.push(Package { name: x.0.to_string(), version: s.trim_start().to_owned()});
                }
            }
        },
        _ => {
            println!("e");
        }
    };

    return Some(dependencies)
}

pub fn generate_download_url(package: &Package) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package.name);
    
    let res = reqwest::blocking::get(url).unwrap();

    let value = res.json::<Value>().unwrap();

    let mut version = package.version.clone();

    if version.chars().nth(0) == Some('^') || version.chars().nth(0) == Some('~') {
        version = version[1..].to_string();
    }
    println!("{}", version);

    let download_url = &value["versions"][version.to_owned()]["dist"]["tarball"];
    println!("{:?} {}", download_url, version);

    if let Value::String(v) = download_url {
        return Some(v.to_string());
    }

    None
}

pub fn download_package(package: &Package) -> Option<bool> {
    if !find_package_in_root(package) {
        let download_url = generate_download_url(package);
    
        if let Some(url) = download_url {
            let res = reqwest::blocking::get(url).unwrap();
    
            let content = BufReader::new(res);
            let tarfile = GzDecoder::new(content);
            let mut archive = Archive::new(tarfile);
    
            println!("downloaded {:?}", package);
    
            archive.unpack(format!("/home/cosmix/.bnpm/{}/{}", package.name, package.version)).unwrap();
    
            Some(true)
        } else {
            Some(false)
        }
    } else {
        Some(true)
    }
}

pub fn download_packages(packages: &Vec<Package>) -> Option<Vec<bool>> {
    let cores = std::thread::available_parallelism().unwrap().get();
    
    let chunk_size = packages.len() / cores;

    let mut chunks: Vec<Vec<Package>> = vec![];

    chunks.push(vec![]);

    let mut start = 0;
    let mut end = chunk_size;
    let mut counter = 0;

    while packages.len() - end > chunk_size {
        chunks.push(vec![]);
        chunks[counter].append(&mut packages[start..end].to_vec());

        start = end + 1;
        end = end + chunk_size;
        counter = counter + 1;
    }

    chunks[counter].append(&mut packages[end..packages.len()].to_vec());

    let mut t = vec![];

    for core in 0..cores {
        let chunks = chunks.clone();
        let core = core.clone();

        t.push(std::thread::spawn(move || {
            for package in &chunks[core] {
                download_package(package);
            }
        }));
    }

    for thread in t {
        thread.join().unwrap();
    }

    Some(vec![true])
}