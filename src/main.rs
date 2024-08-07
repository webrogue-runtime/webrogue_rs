use anyhow::Result;
use clap::Parser;
use webrogue_runtime::WasiFactory;

#[cfg(feature = "backend_wasmtime")]
fn make_backend() -> webrogue_backend_wasmtime::Backend {
    webrogue_backend_wasmtime::Backend::new()
}
#[cfg(feature = "backend_wasmtime")]
use webrogue_backend_wasmtime::make_funcs;

#[cfg(feature = "_backend_wasmer")]
fn make_backend() -> webrogue_backend_wasmer::Backend {
    webrogue_backend_wasmer::Backend::new()
}
#[cfg(feature = "_backend_wasmer")]
use webrogue_backend_wasmer::make_funcs;

#[cfg(feature = "backend_v8")]
fn make_backend() -> webrogue_backend_v8::Backend {
    webrogue_backend_v8::Backend::new()
}
#[cfg(feature = "backend_v8")]
use webrogue_backend_v8::make_funcs;

#[cfg(feature = "wasi_sync")]
fn make_wasi_factory() -> impl webrogue_runtime::WasiFactory {
    webrogue_wasi_sync::WasiFactory::new()
}

make_funcs!({
    "wasi_snapshot_preview1": {
        // defs: "crates/wasi/defs.in",
        module: webrogue_wasi::wasi_snapshot_preview1
    },
    "wr_gl": {
        attribute: "#[cfg(feature = \"gl\")]",
        // defs: "crates/gl/defs.in",
        module: webrogue_gl::wr_gl
    }
});

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[cfg(feature = "gl")]
use webrogue_gl::sdl2;

fn main() -> Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();

    let wasi_factory = make_wasi_factory();
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    let args = Cli::parse();
    let reader = webrogue_runtime::wrapp::Reader::from_file_path(args.path)?;

    #[cfg(all(feature = "std_stream_sdl", feature = "std_stream_os"))]
    compile_error!("webrogue_rs currently can't use more than one std_stream implementation");

    #[cfg(feature = "std_stream_sdl")]
    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            let backend = make_backend();
            lifecycle.run(backend, wasi, reader.clone()).unwrap();
        }),
    );

    #[cfg(not(feature = "std_stream_sdl"))]
    {
        #[cfg(feature = "std_stream_os")]
        webrogue_std_stream_os::bind_streams(&mut wasi);

        let backend = make_backend();
        #[cfg(feature = "gl")]
        let sdl_context = sdl2::init().unwrap();
        #[cfg(feature = "gl")]
        let video_subsystem = sdl_context.video().unwrap();
        #[cfg(feature = "gl")]
        let window = video_subsystem
            .window("webrogue", 600, 300)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();
        #[cfg(feature = "gl")]
        let gl_context = window.gl_create_context().unwrap();
        #[cfg(feature = "gl")]
        let mut webrogue_gl_context = webrogue_gl::wr_gl::Context { window: window };
        lifecycle.run(
            backend,
            make_imports(),
            make_context_vec(
                &mut wasi,
                #[cfg(feature = "gl")]
                &mut webrogue_gl_context,
            ),
            reader,
        )?;
        #[cfg(feature = "gl")]
        drop(webrogue_gl_context);
        #[cfg(feature = "gl")]
        drop(gl_context);
        drop(wasi);
    }

    Ok(())
}
