#[cfg(not(all(feature = "dynamic-mac-colors", target_os = "macos")))]
fn main() {
    return;
}

#[cfg(all(feature = "dynamic-mac-colors", target_os = "macos"))]
fn main() {
    use std::process::Command;
    use std::io::Write;

    let _ = std::fs::create_dir("./swift_bridge");
    let bridging_header = std::fs::File::create("./swift_bridge/bridging-header.h");

    let _ = match bridging_header {
        Ok(mut f) => f.write_all(
        r#"
        #ifndef BRIDGING_HEADER_H
        #define BRIDGING_HEADER_H

        #import "./SwiftBridgeCore.h"
        #import "./egui_system_theme/egui_system_theme.h"

        #endif BRIDGING_HEADER_H
        "#.as_bytes()
        ),
        Err(_) => Ok(()),
    };

    // 1. Use `swift-bridge-build` to generate Swift/C FFI glue.
    //    You can also use the `swift-bridge` CLI.
    let bridge_files = vec!["src/macos/dynamic.rs"];
    swift_bridge_build::parse_bridges(bridge_files)
        .write_all_concatenated("./swift_bridge", "egui_system_theme");

    // 2. Compile Swift library
    let mut cmd = Command::new("swiftc");
    
    cmd.arg("-emit-library")
        .arg("-static")

        .arg("-module-name")
        .arg("swift_colors")

        .arg("-import-objc-header")
        .arg("./swift_bridge/bridging-header.h")

        .arg("./src/macos/colors.swift")
        .arg("./swift_bridge/egui_system_theme/egui_system_theme.swift")
        .arg("./swift_bridge/SwiftBridgeCore.swift");

    if std::env::var("PROFILE").unwrap() == "release" {
        cmd.args(&["-c", "release"]);
    }

    // 3. Link to Swift library
    println!("cargo:rustc-link-lib=static=swift_colors");
    println!("cargo:rustc-link-search=./");

    let exit_status = cmd.spawn().unwrap().wait_with_output().unwrap();

    if !exit_status.status.success() {
        panic!(
            r#"
Stderr: {}
Stdout: {}
"#,
            String::from_utf8(exit_status.stderr).unwrap(),
            String::from_utf8(exit_status.stdout).unwrap(),
        )
    }
}
