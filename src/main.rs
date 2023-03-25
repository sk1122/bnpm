use bnpm::{client::{resolve_package_versions, install::{need_to_install, get_installed_packages}, download::{get_dependency_of_package, download_packages}, get_parent_package}, tree::generate_tree_for_packages, common::types::Package};
use clap::{Parser, Subcommand};

// things to remember
// warn for missing or mismatched peer dependency

// commands to support
// install || i
// -D dev dependency
#[derive(Subcommand, Debug)]
enum SubCommand {
    Install {
        #[clap(name = "packages", required = true)]
        packages: Vec<String>,
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, trailing_var_arg = true)]
struct Args {
    #[command(subcommand)]
    install: SubCommand
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    match args.install {
        SubCommand::Install { packages } => {
            // 1. resolve package versions
            // 2. check which packages are already installed and which are not
            // 3. create tree resolution for packages
                // a. we need to evaluate package.json for each packages and list out their dependencies
                // if package a depends on package b v1.0 and package c depends on package b v2.0, then we need to install both but the latter will be installed in node_module of using package itself (package c in this case)
            // 4. download packages which needs to be installed in node_modules
            // 5. copy already downloaded packages

            // get parent package from package.json
            let parent = get_parent_package().unwrap();

            // return packages with their versions
            let package_with_versions = resolve_package_versions(&packages).unwrap();

            // generate tree of packages and their dependencies
            let tree = generate_tree_for_packages(&parent, &package_with_versions).unwrap();
            
            // return a list of already installed packages
            let (need_to_be_installed, need_not_be_installed) = need_to_install(&tree).unwrap();

            // download packages that needs to be download
            let downloaded_packages = download_packages(&need_to_be_installed).unwrap();

            println!("{:?}", downloaded_packages);

            // package has dependencies a v1.0, b v2.0, c v1.0
            // package a has dependencies b v1.0, d v1.0
            // package b has dependecies c v2.0
            // package c has no dependencies
            // package d has dependencies a v1.0

            // package -> a v1.0, b v2.0, c v1.0, d v1.0 --> (a v1.0 => b v1.0) --> (b v1.0 => c v2.0) --> (c v1.0 => ) --> (d v1.0 => ) ---> (b v1.0 => c v1.0)

            // resolve_packages(downloaded_packages, need_not_be_installed, tree);
        }
    }
}
