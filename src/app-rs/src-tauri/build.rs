use conan::{
    BuildCommandBuilder, BuildPolicy, ConanPackage, InstallCommandBuilder, PackageCommandBuilder,
};
use std::{
    env, iter,
    path::{Path, PathBuf},
    process,
};

use dircpy::copy_dir;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../../conanfile.py");
    println!("cargo:rerun-if-changed=src/main.rs");

    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        eprintln!("Error: OUT_DIR environment variable is not set");
        process::exit(1);
    });

    let libs_dir = out_dir.join("lib");
    let include_dir = out_dir.join("include");

    println!("cargo:libs={}", libs_dir.display());
    println!("cargo:includes={}", include_dir.display());

    // Conan Install
    let conan_profile = env::var("CONAN_PROFILE").unwrap_or_else(|_| "default".to_string());
    let install_command = InstallCommandBuilder::new()
        .with_profile(&conan_profile)
        .build_policy(BuildPolicy::Missing)
        .recipe_path(Path::new("../../../conanfile.py"))
        .output_dir(Path::new("../../../build/"))
        .with_options(&["shared=False", "fPIC=False"])
        .update_check()
        .build();

    let build_info = if let Some(build_info) = install_command.generate() {
        build_info
    } else {
        eprintln!("Error: failed to run conan install");
        process::exit(1);
    };
    build_info.cargo_emit();

    // Conan Build
    match BuildCommandBuilder::new()
        .with_recipe_path(PathBuf::from("../../../conanfile.py"))
        .with_build_path(PathBuf::from("../../../build/"))
        .build()
        .run()
    {
        Some(code) => {
            if !code.success() {
                eprintln!("fail to run conan build exit code = {}", code);
                process::exit(code.code().unwrap());
            }
        }
        None => unreachable!(),
    }

    let package_command = PackageCommandBuilder::new()
        .with_recipe_path(PathBuf::from("../../../conanfile.py"))
        .with_build_path(PathBuf::from("../../../build/"))
        .with_package_path(out_dir.clone())
        .build();

    if let Some(exit_status) = package_command.run() {
        println!("conan package exited with {}", exit_status);
    }

    let conan_package = ConanPackage::new(out_dir.clone());
    if let Err(err) = conan_package.emit_cargo_libs_linkage("lib".into()) {
        eprintln!("Error: Unable to emit cargo linkage: {:?}", err);
        process::exit(1);
    }

    let include_path = out_dir.join("include");
    let include_path = if let Some(include_path) = include_path.to_str() {
        include_path
    } else {
        eprintln!("Error: Unable to get include path");
        process::exit(1);
    };

    let conan_includes = build_info
        .dependencies()
        .iter()
        .flat_map(|dep| dep.get_include_dirs());

    println!("INCLUDE =>>> {}", include_path);

    let mut builder =
        autocxx_build::Builder::new("src/ext.rs", iter::once(include_path).chain(conan_includes))
            .build()
            .unwrap_or_else(|err| {
                eprintln!("Error: Unable to generate bindings: {:?}", err);
                process::exit(1);
            });

    builder.flag_if_supported("-std=c++14").compile("kms");

    copy_dir(&libs_dir, format!("{}/lib", out_dir.display())).expect("Unable to copy libs");
    copy_dir(&include_dir, format!("{}/include", out_dir.display()))
        .expect("Unable to copy includes");

    if let Err(err) = conan_package.emit_cargo_libs_linkage("lib".into()) {
        eprintln!("Error: Unable to emit cargo linkage: {:?}", err);
        process::exit(1);
    }

    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
    }

    tauri_build::build()
}
