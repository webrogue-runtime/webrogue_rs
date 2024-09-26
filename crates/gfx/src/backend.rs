pub trait System {
    fn make_window(&self) -> Box<dyn Window>;
    fn gl_get_proc_address(&self, procname: &str) -> *const ();
}
pub trait Window {
    fn present(&self);
    fn get_size(&self) -> (u32, u32);
    fn get_gl_size(&self) -> (u32, u32);
}
