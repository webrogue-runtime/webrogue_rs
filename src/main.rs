use anyhow::Result;
use clap::Parser;
use webrogue_runtime::WasiFactory;

// embed_plist::embed_info_plist!("Info.plist");

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
        module: webrogue_wasi::wasi_snapshot_preview1
    },
    "webrogue_gl": {
        attribute: "#[cfg(feature = \"gl\")]",
        module: webrogue_gl::api
    },
    "webrogue_gfx": {
        attribute: "#[cfg(feature = \"_gfx\")]",
        module: webrogue_gfx
    }
});

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

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

        #[cfg(feature = "_gfx")]
        let mut webrogue_gfx_context = webrogue_gfx::Context::new();
        #[cfg(feature = "gl")]
        let mut webrogue_gl_context = webrogue_gl::api::Context {
            gfx_context: &mut webrogue_gfx_context,
        };
        lifecycle.run(
            backend,
            make_imports(),
            make_context_vec(
                &mut wasi,
                #[cfg(feature = "gl")]
                &mut webrogue_gl_context,
                #[cfg(feature = "_gfx")]
                &mut webrogue_gfx_context,
            ),
            reader,
        )?;
        #[cfg(feature = "gl")]
        drop(webrogue_gl_context);
        #[cfg(feature = "_gfx")]
        drop(webrogue_gfx_context);
        drop(wasi);
    }

    Ok(())
}
