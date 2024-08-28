use std::sync::Arc;

use webrogue_runtime::WasiFactory;

extern "C" {
    fn wr_rs_sleep(ms: u32);
}

webrogue_backend_web::make_funcs!({
    "wasi_snapshot_preview1": {
        module: webrogue_wasi::wasi_snapshot_preview1
    },
    "webrogue_gl": {
        attribute: "#[cfg(feature = \"gl\")]",
        module: webrogue_gl::api
    },
    "webrogue_gfx": {
        attribute: "#[cfg(feature = \"gl\")]",
        module: webrogue_gfx
    }
});

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let lifecycle = webrogue_runtime::Lifecycle::new();

    let mut wasi_factory = webrogue_wasi_sync::WasiFactory::new();
    wasi_factory.sleep = Some(webrogue_wasi_sync::Sleep {
        f: Arc::new(|duration| unsafe { wr_rs_sleep(duration.as_millis() as u32) }),
    });
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    let reader = webrogue_runtime::wrapp::Reader::from_static_slice(include_bytes!(
        "../../examples/simple/simple.wrapp"
    ))?;

    webrogue_std_stream_os::bind_streams(&mut wasi);

    #[cfg(feature = "gl")]
    let mut webrogue_gfx_context = webrogue_gfx::Context::new();
    #[cfg(feature = "gl")]
    let mut webrogue_gl_context = webrogue_gl::api::Context {
        gfx_context: &mut webrogue_gfx_context,
    };

    let backend = webrogue_backend_web::Backend::new();
    lifecycle.run(
        backend,
        make_imports(),
        make_context_vec(
            &mut wasi,
            #[cfg(feature = "gl")]
            &mut webrogue_gl_context,
            #[cfg(feature = "gl")]
            &mut webrogue_gfx_context,
        ),
        reader,
    )?;
    drop(webrogue_gl_context);
    drop(webrogue_gfx_context);
    drop(wasi);

    Ok(())
}

#[no_mangle]
extern "C" fn rust_main() {
    match main() {
        Err(e) => {
            panic!("{}", e.to_string())
        }
        Ok(_) => {}
    }
}
