fn main() {
    run();
}

pub fn run() {
    let settings = voxgen::window::WindowSettings::default();
    voxgen::core::init(settings);
}
