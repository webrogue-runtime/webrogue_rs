use std::{path::PathBuf, str::FromStr, sync::Arc};

use webrogue_runtime::wiggle;

#[derive(Clone)]
pub struct Sleep {
    pub f: Arc<dyn Fn(std::time::Duration)>,
}

pub struct WrSyncSched {
    pub actual: Box<dyn wasi_common::WasiSched>,
    pub sleep: Sleep,
}

unsafe impl Sync for Sleep {}
unsafe impl Send for Sleep {}

#[wiggle::async_trait]
impl wasi_common::WasiSched for WrSyncSched {
    async fn poll_oneoff<'a>(
        &self,
        poll: &mut wasi_common::sched::Poll<'a>,
    ) -> Result<(), wasi_common::snapshots::preview_1::types::Error> {
        self.actual.poll_oneoff(poll).await
    }
    async fn sched_yield(&self) -> Result<(), wasi_common::snapshots::preview_1::types::Error> {
        self.actual.sched_yield().await
    }
    async fn sleep(
        &self,
        duration: std::time::Duration,
    ) -> Result<(), wasi_common::snapshots::preview_1::types::Error> {
        (*self.sleep.f)(duration);
        Ok(())
    }
}

pub struct WasiFactory {
    pub sleep: Option<Sleep>,
}

impl WasiFactory {
    pub fn new() -> Self {
        Self { sleep: None }
    }
}

impl webrogue_runtime::WasiFactory for WasiFactory {
    fn make(&self) -> wasi_common::WasiCtx {
        let shed = match &self.sleep {
            None => wasi_common::sync::sched::sched_ctx(),
            Some(sleep) => Box::new(WrSyncSched {
                actual: wasi_common::sync::sched::sched_ctx(),
                sleep: sleep.clone(),
            }),
        };
        wasi_common::WasiCtx::new(
            wasi_common::sync::random_ctx(),
            wasi_common::sync::clocks_ctx(),
            shed,
            wasi_common::Table::new(),
        )
    }

    fn add_actual_dir(
        &self,
        wasi: &mut wasi_common::WasiCtx,
        actual_path: std::path::PathBuf,
        guest_path: &str,
    ) {
        wasi.push_dir(
            Box::new(wasi_common::sync::dir::Dir::from_cap_std(
                cap_std::fs::Dir::open_ambient_dir(actual_path, cap_std::ambient_authority())
                    .unwrap(),
            )),
            PathBuf::from_str(guest_path).unwrap(),
        )
        .unwrap();
    }
}
