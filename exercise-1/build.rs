use std::env;
use std::process::Command;

fn main() {
    // if build.rs or src/main.rs is changed, rerun this build.rs
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");

    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
    let xpdf_dir = format!("{}/xpdf", cwd);

    // make clean
    Command::new("make")
        .arg("clean")
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Couldn't clean xpdf directory");

    Command::new("rm")
        .arg("-r")
        .arg("-v")
        .arg("-f")
        .arg(&format!("{}/install", xpdf_dir))
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Couldn't clean xpdf's install directory");

    env::set_var("LLVM_CONFIG", "llvm-config-11");

    // configure with afl-clang-fast and set install directory to ./xpdf/install
    Command::new("./configure")
        .arg(&format!("--prefix={}/install", xpdf_dir))
        .env("CC", "/home/h1k0/sectools/AFLplusplus/afl-clang-fast")
        .env("CXX", "/home/h1k0/sectools/AFLplusplus/afl-clang-fast++")
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Couldn't configure xpdf to build using afl-clang-fast");

    Command::new("make")
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Couldn't make xpdf");

    Command::new("make")
        .arg("install")
        .current_dir(xpdf_dir)
        .status()
        .expect("Couldn't install xpdf");
}