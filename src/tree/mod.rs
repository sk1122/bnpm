use std::collections::VecDeque;

use serde::Serialize;

use crate::{common::types::{Package, ReturnType}, client::download::get_dependency_of_package};

#[derive(Serialize, Clone, Debug)]
pub struct Node {
    pub package: Package,
    pub parent: Option<Box<Node>>,
    // pub dependencies: Vec<Node>
}

type Graph = Node;

impl Graph {
    pub fn new(package: Package, parent: Option<Box<Graph>>) -> Self {
        Graph {
            package,
            // dependencies,
            parent
        }
    }

    // pub fn add_dep(&mut self, dep: &Graph) -> &mut Self {
    //     // self.dependencies.push(dep.clone());

    //     self
    // }
}

fn package_contain_partial_or_full(packages: &Vec<Package>, package: &Package) -> Option<ReturnType> {
    for p in packages {
        if p.name == package.name {
            if p.version == package.version {
                return Some(ReturnType::Full);
            } else {
                return Some(ReturnType::Partial);
            }
        }
    }

    Some(ReturnType::Incomplete)
}

pub fn generate_tree_for_packages(parent: &Package, packages: &Vec<Package>) -> Option<Vec<Node>> {
    let parent_node = Graph::new(parent.to_owned(), None);

    let mut nodes = vec![parent_node.clone()];

    let mut first_line_packages = vec![];

    let mut queue: VecDeque<Graph> = VecDeque::new();

    // traverse through packages
    // get their dependencies for each package
        // if a dependency exists in packages with same version, pass
        // if a dependency exists in packages with different version, push that dependency to dependencies of respective package
        // if a dependency doesn't exist in packages, add it in packages

    for package in packages {
        first_line_packages.push(package.to_owned());

        let node = Graph {
            package: package.to_owned(),
            parent: Some(Box::new(parent_node.clone())),
        };
        
        nodes.push(node.clone());
        queue.push_back(node);
    }

    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();

        let dependencies = get_dependency_of_package(&node.package).unwrap();

        for dependency in dependencies {
            let contains = package_contain_partial_or_full(&first_line_packages, &dependency).unwrap();

            match contains {
                ReturnType::Full => {
                    // pass
                    println!("found a full match => {:?}", dependency);
                },
                ReturnType::Partial => {
                    println!("found a partial match => {:?}", dependency);

                    let dependency_node = Graph {
                        package: dependency.to_owned(),
                        parent: Some(Box::new(node.clone())),
                    };

                    nodes.push(dependency_node.clone());
                    queue.push_back(dependency_node);
                },
                ReturnType::Incomplete => {
                    println!("found a incomplete match => {:?}", dependency);

                    let dependency_node = Graph {
                        package: dependency.to_owned(),
                        parent: Some(Box::new(parent_node.clone())),
                    };

                    first_line_packages.push(dependency);
                    
                    nodes.push(dependency_node.clone());
                    queue.push_back(dependency_node);
                }
            }
        }
    }

    return Some(nodes);
}