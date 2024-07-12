use sdl2::{event::Event, pixels::Color};
use std::collections::BTreeMap;
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
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context
        .load_font_from_rwops(
            sdl2::rwops::RWops::from_bytes(include_bytes!("../Hack-Regular.ttf")).unwrap(),
            16,
        )
        .unwrap();
    let (font_width, font_height) = font.size_of_char('a').unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window_width = 800;
    let mut window_height = 600;

    let window = video_subsystem
        .window("webrogue", window_width, window_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut utf_parser = utf8parse::Parser::new();
    let mut lines = std::collections::VecDeque::<Vec<char>>::new();
    lines.push_back(Vec::new());

    let mut char_map = BTreeMap::<char, sdl2::render::Texture>::new();

    'running: loop {
        let mut last_data: Vec<u8> = Vec::new();
        {
            let mut guard = output_data.lock().unwrap();
            let _ = last_data.write(&guard);
            guard.clear();
        };

        struct Utf8Receiver<'a>(&'a mut std::collections::VecDeque<Vec<char>>);

        impl<'a> utf8parse::Receiver for Utf8Receiver<'a> {
            fn codepoint(&mut self, c: char) {
                match c {
                    '\n' => {
                        (*self).0.push_back(Vec::new());
                    }
                    c => {
                        (*self).0.back_mut().unwrap().push(c);
                    }
                }
            }
            fn invalid_sequence(&mut self) {}
        }

        let mut reciver = Utf8Receiver { 0: &mut lines };

        for byte in last_data {
            utf_parser.advance(&mut reciver, byte)
        }

        while lines.len() > 1000 {
            lines.pop_front();
        }

        let cells_x_count = window_width / font_width;
        let cells_y_count = window_height / font_height;

        let visible_lines = lines.iter().rev().take(cells_y_count as usize).rev();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut line_i: i32 = 0;

        for line in visible_lines.clone() {
            line_i += line.len() as i32 / cells_x_count as i32;
            line_i += 1;
        }

        line_i = std::cmp::min(0, cells_y_count as i32 - line_i);

        for line in visible_lines {
            let mut ch_i: u32 = 0;
            for ch in line {
                if line_i >= 0 {
                    let ch_texture = char_map.get(&ch);
                    let ch_texture = match ch_texture {
                        Some(ch_texture) => ch_texture,
                        None => {
                            let surface = font
                                .render_char(*ch)
                                .blended(Color::RGBA(255, 255, 255, 255))
                                .map_err(|e| e.to_string())
                                .unwrap();
                            let texture = texture_creator
                                .create_texture_from_surface(&surface)
                                .map_err(|e| e.to_string())
                                .unwrap();
                            char_map.insert(*ch, texture);
                            &char_map[&ch]
                        }
                    };
                    canvas
                        .copy(
                            &ch_texture,
                            None,
                            Some(sdl2::rect::Rect::new(
                                (window_width * ch_i as u32 / cells_x_count as u32) as i32,
                                (window_height * line_i as u32 / cells_y_count as u32) as i32,
                                font_width,
                                font_height,
                            )),
                        )
                        .unwrap();
                }
                ch_i += 1;
                if ch_i >= cells_x_count {
                    line_i += 1;
                    ch_i = 0;
                }
            }

            line_i += 1;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized(new_width, new_height) => {
                        window_width = new_width as u32;
                        window_height = new_height as u32;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if handle.is_finished() {
            break 'running;
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
