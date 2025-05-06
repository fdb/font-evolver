mod constants;
mod genotype;
mod render;
fn main() {
    println!("Hello, world!");
    let pixel_buffer = render::render_target_glyph("fonts/NotoSans-Light.ttf", 'g').unwrap();
    render::save_buffer(&pixel_buffer, "output.png").unwrap();
}
