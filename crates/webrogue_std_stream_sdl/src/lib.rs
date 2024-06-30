use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::io::Write;

use webrogue_runtime::wasi_common::{
    file::{FdFlags, FileType, WasiFile},
    Error, ErrorExt,
};

pub struct Stdout {
    output: Arc<Mutex<Vec<u8>>>,
}

#[wiggle::async_trait]
impl WasiFile for Stdout {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    #[cfg(unix)]
    fn pollable(&self) -> Option<rustix::fd::BorrowedFd> {
        None
    }
    #[cfg(windows)]
    fn pollable(&self) -> Option<io_extras::os::windows::RawHandleOrSocket> {
        None
    }

    async fn get_filetype(&self) -> Result<FileType, Error> {
        Ok(FileType::CharacterDevice)
    }
    async fn get_fdflags(&self) -> Result<FdFlags, Error> {
        Ok(FdFlags::APPEND)
    }
    async fn write_vectored<'a>(&self, bufs: &[std::io::IoSlice<'a>]) -> Result<u64, Error> {
        let mut n: usize = 0;
        let mut guard = self.output.lock().unwrap();
        for buf in bufs {
            n += guard.write(buf)?;
        }
        Ok(n as u64)
    }
    async fn write_vectored_at<'a>(
        &self,
        _bufs: &[std::io::IoSlice<'a>],
        _offset: u64,
    ) -> Result<u64, Error> {
        Err(Error::seek_pipe())
    }
    async fn seek(&self, _pos: std::io::SeekFrom) -> Result<u64, Error> {
        Err(Error::seek_pipe())
    }
    async fn set_times(
        &self,
        _atime: Option<webrogue_runtime::wasi_common::SystemTimeSpec>,
        _mtime: Option<webrogue_runtime::wasi_common::SystemTimeSpec>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

pub fn run_in_terminal<T>(
    wasi: webrogue_runtime::wasi_common::WasiCtx,
    func: Arc<dyn Fn(webrogue_runtime::wasi_common::WasiCtx) -> T + Sync + Send>,
) where
    T: Sync + Send + 'static,
{
    let vec: Vec<u8> = vec![];
    let output_data = Arc::new(Mutex::new(vec));
    wasi.set_stdout(Box::new(Stdout {
        output: output_data.clone(),
    }));
    wasi.set_stderr(Box::new(Stdout {
        output: output_data.clone(),
    }));
    let func_copy = func.clone();
    let handle = std::thread::spawn(move || {
        func_copy.as_ref()(wasi);
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("webrogue", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i: u8 = 0;
    'running: loop {
        let data = output_data.lock().unwrap().clone();
        let str = String::from_utf8_lossy(&data);

        // i = (i + 1) % 255;

        let i = (100 * str.len()) as u8;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        if handle.is_finished() {
            break 'running;
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
