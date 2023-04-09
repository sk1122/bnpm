use std::path::Path;

use crate::{tree::Node, common::utils::get_project_root_path};

use std::fs;
use std::path::{PathBuf};

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
            fs::create_dir_all(&dest.join("node_modules"))?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}


pub fn resolve_packages(tree: Vec<Node>) {
    // println!("length {:?}", tree.len());
    for leaf in tree {
        let project_root = get_project_root_path().unwrap();

        let node_modules = Path::new(project_root.path.as_str()).join("node_modules");

        // println!("22");
        std::fs::create_dir_all(&node_modules).unwrap();
        // println!("23");
        let leaf_dir = Path::new("/home/cosmix/.bnpm");

        // let parent: Node = *leaf.parent;

        // println!("{:?} 123", leaf);

        if let Some(parent_box) = leaf.parent {
            let parent = *parent_box;

            println!("{:?}", parent.package);
            
            if parent.package == project_root.package {
                let copy_dir = &node_modules;

                let mut version = leaf.package.version.clone();
                
                if version.chars().nth(0) == Some('^') || version.chars().nth(0) == Some('~') {
                    version = version[1..].to_string();
                }
                
                copy(leaf_dir.join(&leaf.package.name).join(&version).join("package"), &copy_dir.join(&leaf.package.name)).unwrap();
                println!("COPY DIR (PARENT) => {:?} {}", copy_dir, leaf_dir.join(&leaf.package.name).join(&version).to_string_lossy());
            } else {
                
                let mut cloned_parent = parent.clone();
                
                let mut copy_dir = node_modules.join(&cloned_parent.package.name).join("node_modules");
                
                while cloned_parent.parent.as_ref().unwrap_or(&Box::new(Node { package: cloned_parent.package.clone(), parent: None })).parent.is_some() {
                    copy_dir = copy_dir.join(cloned_parent.package.name.clone()).join("node_modules");
                    // println!("{:?}", copy_dir);
    
                    cloned_parent = *cloned_parent.parent.as_ref().unwrap_or(&Box::new(Node { package: cloned_parent.package.clone(), parent: None })).clone();
                }

                let mut version = leaf.package.version.clone();
                
                if version.chars().nth(0) == Some('^') || version.chars().nth(0) == Some('~') {
                    version = version[1..].to_string();
                }
                
                println!("COPY DIR (CHILD) => {:?} {}", copy_dir, leaf_dir.join(&leaf.package.name).join(&version).to_string_lossy());
                copy(leaf_dir.join(&leaf.package.name).join(version).join("package"), &copy_dir.join(&leaf.package.name)).unwrap();
            }
        } else {
            // println!("none");
        }
    }
}