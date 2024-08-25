use std::io::prelude::*;

fn bool_to_string(value: bool) -> String {
    if value { "true" } else { "false" }.to_owned()
}

fn main() {
    let root = std::env::current_dir().unwrap();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = root.join(out_dir);
    let schema_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();
    let target_os = target.as_str().splitn(3, '-').nth(2).unwrap();
    let target_is_windows = target_os.contains("windows");
    let target_is_macos = target_os == "darwin";
    if !target_is_windows {
        panic!("!target_is_windows");
    }
    if target_is_macos || target_is_windows {
        let anglebuild_dir = schema_dir.join("anglebuild");
        let _ = std::fs::create_dir(anglebuild_dir.clone());

        if target_is_windows {
            std::env::set_var("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");
        }

        std::process::Command::new("git")
            .current_dir(anglebuild_dir.clone())
            .args([
                "clone",
                "https://chromium.googlesource.com/chromium/tools/depot_tools.git",
            ])
            .status()
            .unwrap();

        std::env::set_var(
            "PATH",
            format!(
                "{}{}{}",
                anglebuild_dir.join("depot_tools").display(),
                if host.contains("windows") { ";" } else { ":" },
                std::env::var("PATH").unwrap()
            ),
        );

        let angle_dir = anglebuild_dir.join("angle");
        let _ = std::fs::create_dir(angle_dir.clone());
        let mut gclient_file = std::fs::File::create(angle_dir.join(".gclient")).unwrap();
        gclient_file
            .write_all(
                br#"solutions = [
  {
    "name": ".",
    "url": "https://chromium.googlesource.com/angle/angle.git",
    "deps_file": "DEPS",
    "managed": False,
    "custom_vars": {},
  },
]
"#,
            )
            .unwrap();
        drop(gclient_file);

        std::process::Command::new(anglebuild_dir.join("depot_tools").join(
            if host.contains("windows") {
                "gclient.bat"
            } else {
                "gclient"
            },
        ))
        .current_dir(angle_dir.clone())
        .args(["sync"])
        .status()
        .unwrap();

        let is_debug_arg = bool_to_string(std::env::var("PROFILE").unwrap().as_str() == "debug" && !target_is_windows);

        let enable_d3d9 = bool_to_string(target_is_windows);
        let enable_d3d11 = bool_to_string(target_is_windows);
        let enable_metal = bool_to_string(target_is_macos);

        let gn_args = format!(
            "--args=angle_build_all=false angle_enable_null=false angle_has_frame_capture=false angle_enable_gl=false angle_enable_vulkan=false is_debug={} angle_enable_d3d9={} angle_enable_d3d11={} angle_enable_gl=false angle_enable_null=false angle_enable_metal={} angle_enable_vulkan=false angle_enable_essl=false angle_enable_wgpu=false angle_enable_glsl=true is_component_build=false", 
            is_debug_arg,
            enable_d3d9,
            enable_d3d11,
            enable_metal
        );

        std::process::Command::new(anglebuild_dir.join("depot_tools").join(
            if host.contains("windows") {
                "gn.bat"
            } else {
                "gn"
            },
        ))
        .current_dir(angle_dir.clone()) // TODO is_debug = false
        .args(["gen", "out/Release", gn_args.as_str()])
        .status()
        .unwrap();

        std::process::Command::new(anglebuild_dir.join("depot_tools").join(
            if host.contains("windows") {
                "autoninja.bat"
            } else {
                "autoninja"
            },
        ))
        .current_dir(angle_dir.clone())
        .args(["-C", "out/Release", "libGLESv2", "libEGL"])
        .status()
        .unwrap();

        if target_is_macos {
            std::fs::copy(
                angle_dir.join("out/Release/libEGL.dylib"),
                schema_dir.join("libEGL.dylib"),
            )
            .unwrap();

            std::fs::copy(
                angle_dir.join("out/Release/libGLESv2.dylib"),
                schema_dir.join("libGLESv2.dylib"),
            )
            .unwrap();
        } else if target_is_windows {
            std::fs::copy(
                angle_dir.join("out/Release/libEGL.dll"),
                schema_dir.join("libEGL.dll"),
            )
            .unwrap();

            std::fs::copy(
                angle_dir.join("out/Release/libGLESv2.dll"),
                schema_dir.join("libGLESv2.dll"),
            )
            .unwrap();
        }
    };
}
