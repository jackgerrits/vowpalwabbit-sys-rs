use bindgen;
use cmake;
use num_cpus;
use std::borrow::Borrow;
use std::env;
use std::path::PathBuf;

fn envvar_is_set<T: Borrow<str>>(s: T) -> bool {
    match env::var(s.borrow()) {
        Ok(_) => true,
        _ => false,
    }
}

fn main() {
    // For some reason on Windows I had to force exception handling to be turned on.
    let exception_handling_flag = if cfg!(target_os = "windows") {
        "/EHsc"
    } else {
        ""
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // The CMake lib uses an environment var to set parallelization, if it's already set by
    // an external user then respect the value, otherwise use all cores.
    if !envvar_is_set("NUM_JOBS") {
        env::set_var("NUM_JOBS", num_cpus::get().to_string());
    }

    let dst = cmake::Config::new("external/vowpal_wabbit")
        // This flag is used as it forces dependencies to be statically linked but still produces a dynamic lib.
        .define("STATIC_LINK_VW_JAVA", "On")
        // Produce the target as a shared lib.
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

    // Only support consuming as a shared object.
    println!("cargo:rustc-link-lib=dylib=vw_c_api");

    // There are some headers copied to the binary directory as part of the build so
    // we must add it to the include path.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!(
            "-I{}",
            out_path.join("build/bindings/c/include/").display()
        ))
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
