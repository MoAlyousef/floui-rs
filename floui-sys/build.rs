use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/floui.h");
    println!("cargo:rerun-if-changed=src/floui.mm");
    println!("cargo:rerun-if-changed=src/floui.cpp");
    println!("cargo:rerun-if-changed=src/floui.rs");
    let target = env::var("TARGET").unwrap();
    if target.contains("ios") {
        let sdk = if target.contains("sim") {
            "iphonesimulator"
        } else {
            "iphoneos"
        };
        let sdk = String::from_utf8(
            Command::new("xcrun")
                .args(&["--sdk", sdk, "--show-sdk-path"])
                .output()
                .expect("Couldn't run xcrun! Verify your XCode installation.")
                .stdout,
        )
        .unwrap();
        println!("cargo:rustc-link-lib=framework=UIKit");
        println!("cargo:rustc-link-lib=framework=WebKit");
        let mut cc = cc::Build::new();
        if cfg!(feature = "ios-webview") {
            cc.define("FLOUI_IOS_WEBVIEW", None);
        }
        cc.file("src/floui.mm")
            .cpp(true)
            .flag_if_supported(&format!("-isysroot={}", sdk))
            .flag_if_supported("-std=c++17")
            .flag_if_supported("-w")
            .compile("floui");
    } else if target.contains("android") {
        println!("cargo:rerun-if-changed=CMakeLists.txt");
        println!("cargo:rerun-if-env-changed=ANDROID_SDK_ROOT");
        println!("cargo:rerun-if-env-changed=ANDROID_NDK_ROOT");
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let sdk =
            PathBuf::from(env::var("ANDROID_SDK_ROOT").expect("ANDROID_SDK_ROOT should be set!"));
        let ndk =
            PathBuf::from(env::var("ANDROID_NDK_ROOT").expect("ANDROID_NDK_ROOT should be set!"));
        let target_triple = env::var("TARGET").unwrap();
        let cmake_build_dir = out_dir.join("cmake_build").to_str().unwrap().to_string();
        let mut cmd = vec![];
        cmd.push(format!("-B{}", cmake_build_dir));
        cmd.push("-DCMAKE_BUILD_TYPE=Release".to_string());
        cmd.push(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            out_dir.to_str().unwrap()
        ));
        cmd.push("-GNinja".to_string());
        cmd.push("-DCMAKE_SYSTEM_NAME=Android".to_string());
        cmd.push("-DCMAKE_SYSTEM_VERSION=21".to_string());
        cmd.push("-DANDROID_PLATFORM=android-21".to_string());
        cmd.push(format!("-DCMAKE_ANDROID_NDK={}", &ndk.to_str().unwrap()));
        cmd.push(format!("-DANDROID_NDK={}", &ndk.to_str().unwrap()));
        cmd.push(format!(
            "-DCMAKE_MAKE_PROGRAM={}",
            find_ninja(&sdk)
                .expect("Couldn't find NDK ninja!")
                .to_str()
                .unwrap()
        ));
        cmd.push(format!(
            "-DCMAKE_TOOLCHAIN_FILE={}",
            ndk.join("build")
                .join("cmake")
                .join("android.toolchain.cmake")
                .to_str()
                .unwrap()
        ));
        match target_triple.as_str() {
            "i686-linux-android" => {
                cmd.push("-DANDROID_ABI=x86".to_string());
                cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86".to_string());
            }
            "aarch64-linux-android" => {
                cmd.push("-DANDROID_ABI=arm64-v8a".to_string());
                cmd.push("-DCMAKE_ANDROID_ARCH_ABI=arm64-v8a".to_string());
            }
            "armv7-linux-androideabi" => {
                cmd.push("-DANDROID_ABI=armeabi-v7a".to_string());
                cmd.push("-DCMAKE_ANDROID_ARCH_ABI=armeabi-v7a".to_string());
            }
            "x86_64-linux-android" => {
                cmd.push("-DANDROID_ABI=x86_64".to_string());
                cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86_64".to_string());
            }
            _ => panic!("Unknown android triple"),
        }
        Command::new("cmake")
            .args(&cmd)
            .current_dir(".")
            .status()
            .expect("CMake is needed for android builds!");
        Command::new("cmake")
            .args(&["--build", &cmake_build_dir, "--target", "install"])
            .current_dir(".")
            .status()
            .expect("CMake is needed for android builds!");
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=floui");
    } else {
        println!("cargo:warning=Building against the host jni and current arch!");
        // let host = env::var("HOST").unwrap();
        // let host = if host.contains("windows") {
        //     "win32"
        // } else if host.contains("apple-darwin") {
        //     "darwin"
        // } else {
        //     "linux"
        // };
        // let include_path = env::var("JAVA_HOME").expect("JAVA_HOME should be set!");
        // let include_path = PathBuf::from(include_path);
        // cc::Build::new()
        //     .file("src/floui.cpp")
        //     .cpp(true)
        //     .include(include_path.join("include"))
        //     .include(include_path.join(&format!("include/{}", host)))
        //     .flag_if_supported("-std=c++17")
        //     .flag_if_supported("-w")
        //     .compile("floui");
    }
}

fn find_ninja(sdk_path: &Path) -> Option<PathBuf> {
    let cmk = sdk_path.join("cmake");
    for subdir in fs::read_dir(cmk).unwrap() {
        let subdir = subdir
            .unwrap() // Shouldn't fail!
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if subdir.starts_with("3.") {
            return Some(
                sdk_path
                    .join("cmake")
                    .join(subdir)
                    .join("bin")
                    .join("ninja"),
            );
        }
    }
    None
}
