use std::path::Path;

use crate::{tree::Node, common::utils::get_project_root_path};

pub fn resolve_packages(tree: Vec<Node>) {
    println!("length {:?}", tree.len());
    for leaf in tree {
        let project_root = get_project_root_path().unwrap();

        let node_modules = Path::new(project_root.path.as_str());

        println!("22");
        std::fs::create_dir_all(node_modules.join("node_modules")).unwrap();
        println!("23");
        let leaf_dir = format!("~/.bnpm/{}/{}", leaf.package.name, leaf.package.version);

        // let parent: Node = *leaf.parent;

        println!("{:?} 123", leaf);

        if let Some(parent_box) = leaf.parent {
            let parent = *parent_box;
            
            if parent.package == project_root.package {
                let copy_dir = node_modules.join("/node_modules").join(parent.package.name);
    
                println!("COPY DIR (PARENT) => {:?}", copy_dir)
            } else {
                
                let mut cloned_parent = parent.clone();
                
                let mut copy_dir = node_modules.join("/node_modules").join(&cloned_parent.package.name);
                
                while cloned_parent.parent.as_ref().unwrap_or(&Box::new(Node { package: cloned_parent.package.clone(), parent: None })).parent.is_some() {
                    copy_dir = copy_dir.join(cloned_parent.package.name.clone());
                    println!("{:?}", copy_dir);
    
                    cloned_parent = *cloned_parent.parent.as_ref().unwrap_or(&Box::new(Node { package: cloned_parent.package.clone(), parent: None })).clone();
                }
    
                println!("COPY DIR (CHILD) => {:?}", copy_dir)
            }
        } else {
            println!("none");
        }
    }
}