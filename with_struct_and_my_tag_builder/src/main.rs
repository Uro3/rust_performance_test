// cargo run --release

use std::time;

mod table_renderer;
mod tag_builder;

fn main() {
    let now = time::Instant::now();
    let renderer = table_renderer::Renderer::new(800, 800);
    renderer.render();
    println!("{:?}", now.elapsed());
    // println!("{}", renderer.render());
}
