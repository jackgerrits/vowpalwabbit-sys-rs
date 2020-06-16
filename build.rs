use bindgen;
use cmake;
use std::env;

use std::path::PathBuf;
fn main() {
    // For some reason on Windows I had to force exception handling to be turned on.
    let exception_handling_flag = if cfg!(target_os = "windows") {
        "/EHsc"
    } else {
        ""
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = cmake::Config::new("external/vowpal_wabbit")
        // This flag is used as it forces dependencies to be statically linked but still produces a dynamic lib
        .define("STATIC_LINK_VW_JAVA", "On")
        .define("BUILD_SHARED_LIBS", "On")
        .define("VW_INSTALL", "Off")
        .define("BUILD_TESTS", "Off")
        .define("GIT_SUBMODULE", "Off")
        .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_LIBRARY_OUTPUT_DIRECTORY", out_path.join("lib"))
        .define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", out_path.join("bin"))
        .build_target("vw_c_api")
        .cxxflag(exception_handling_flag)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Debug").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("bin/Release").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib/Release").display()
    );
    println!("cargo:rustc-link-lib=vw_c_api");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
