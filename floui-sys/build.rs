fn main() {
    println!("cargo:rerun-if-changed=src/floui.h");
    println!("cargo:rerun-if-changed=src/floui.mm");
    println!("cargo:rerun-if-changed=src/floui.cpp");
    println!("cargo:rerun-if-changed=src/floui.rs");
    let target = std::env::var("TARGET").unwrap();
    if target.contains("ios") {
        let sdk = if target.contains("sim")
        {
            "iphonesimulator"
        } else {
            "iphoneos"
        };
        let sdk = String::from_utf8(std::process::Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output().unwrap()
        .stdout).unwrap();
        println!("cargo:rustc-link-lib=framework=UIKit");
        cc::Build::new()
            .file("src/floui.mm")
            .cpp(true)
            .flag_if_supported(&format!("-isysroot={}", sdk))
            .flag_if_supported("-std=c++17")
            .flag_if_supported("-w")
            .compile("floui");
    } else if target.contains("android") {
        cc::Build::new()
            .file("src/floui.cpp")
            .cpp(true)
            .flag_if_supported("-std=c++17")
            .flag_if_supported("-w")
            .compile("floui");
    } else {
        panic!("Unsupported platform!");
    }
}