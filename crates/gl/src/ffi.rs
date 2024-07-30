extern "C" {
    pub fn glViewport(x: i32, y: i32, width: i32, height: i32);
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    pub fn glClear(mask: u32);
}
