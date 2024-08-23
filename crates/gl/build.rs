use std::io::prelude::*;

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
    let target_os = target.as_str().splitn(3, '-').nth(2).unwrap();
    if target_os == "darwin" {
        let anglebuild_dir = schema_dir.join("anglebuild");
        let _ = std::fs::create_dir(anglebuild_dir.clone());

        std::process::Command::new("git")
            .current_dir(anglebuild_dir.clone())
            .args([
                "clone",
                "https://chromium.googlesource.com/chromium/tools/depot_tools.git",
            ])
            .output()
            .unwrap();

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
        std::env::set_var(
            "PATH",
            format!(
                "{}:{}",
                anglebuild_dir.join("depot_tools").display(),
                std::env::var("PATH").unwrap()
            ),
        );

        std::process::Command::new("gclient")
            .current_dir(angle_dir.clone())
            .args(["sync"])
            .output()
            .unwrap();

        let is_debug_arg = match std::env::var("PROFILE").unwrap().as_str() {
            "debug" => "true",
            _ => "false",
        };

        let gn_args = format!(
            "--args=is_debug={} angle_enable_d3d9=false angle_enable_d3d11=false angle_enable_gl=false angle_enable_null=false angle_enable_metal=true angle_enable_vulkan=false angle_enable_essl=false angle_enable_wgpu=false angle_enable_glsl=true is_component_build=false", 
            is_debug_arg
        );

        std::process::Command::new("gn")
            .current_dir(angle_dir.clone()) // TODO is_debug = false
            .args(["gen", "out/Release", gn_args.as_str()])
            .output()
            .unwrap();

        std::process::Command::new("autoninja")
            .current_dir(angle_dir.clone())
            .args(["-C", "out/Release", "libGLESv2", "libEGL"])
            .output()
            .unwrap();

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
    };
}
