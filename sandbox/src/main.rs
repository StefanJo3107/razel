use razel::Application;

pub struct Sandbox;

impl Application for Sandbox {
    fn new() -> Sandbox {
        Sandbox {}
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {}
}

fn main() {
    let app = Sandbox::new();
    app.run();
}
