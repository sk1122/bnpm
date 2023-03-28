use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.name {
            if self.version == other.version {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[derive(Clone, Debug)]
pub enum ReturnType {
    Full,
    Partial,
    Incomplete
}

#[derive(Clone, Debug)]
pub struct ProjectRoot {
    pub name: String,
    pub path: String,
    pub package: Package
}