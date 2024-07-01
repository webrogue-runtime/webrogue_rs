use std::{path::PathBuf, str::FromStr};
pub struct WasiFactory {}

impl webrogue_runtime::WasiFactory for WasiFactory {
    fn make(&self) -> wasi_common::WasiCtx {
        wasi_common::WasiCtx::new(
            wasi_common::sync::random_ctx(),
            wasi_common::sync::clocks_ctx(),
            wasi_common::sync::sched::sched_ctx(),
            wasi_common::Table::new(),
        )
    }

    fn add_actual_dir(&self, wasi: &mut wasi_common::WasiCtx, actual_path: &str, guest_path: &str) {
        #[cfg(not(windows))]
        let cap_std_dir =
            cap_std::fs::Dir::from_std_file(std::fs::File::open(actual_path).unwrap());
        wasi.push_dir(
            Box::new(wasi_common::sync::dir::Dir::from_cap_std(cap_std_dir)),
            PathBuf::from_str(guest_path).unwrap(),
        )
        .unwrap();
    }
}
