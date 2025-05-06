use crate::{
    constants::{MAX_COORD, RENDER_SIZE},
    genotype::Genotype,
};
use fontdue::{Font, FontSettings};
use image::GrayImage;
use std::{fs, io::Write};

const RASTER_FONT_SIZE_RATIO: f32 = 0.75;

pub type PixelBuffer = Vec<u8>;

pub fn create_pixel_buffer() -> PixelBuffer {
    vec![0; (RENDER_SIZE * RENDER_SIZE) as usize]
}

pub fn set_pixel(buffer: &mut PixelBuffer, x: u32, y: u32, color: u8) {
    let index = (y * RENDER_SIZE + x) as usize;
    if index < buffer.len() {
        buffer[index] = color;
    }
}

fn scale_coord(coord: i32) -> u32 {
    (coord as f32 / MAX_COORD as f32 * RENDER_SIZE as f32).round() as u32
}

fn draw_line_bresenham(
    buffer: &mut PixelBuffer,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: u8,
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
        set_pixel(buffer, x0 as u32, y0 as u32, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let err2 = err * 2;
        if err2 >= dy {
            err += dy;
            x0 += sx;
        }
        if err2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn render_genotype(genotype: &Genotype) -> PixelBuffer {
    let mut buffer = create_pixel_buffer();
    for line in &genotype.lines {
        let x0 = scale_coord(line.start.x);
        let y0 = scale_coord(line.start.y);
        let x1 = scale_coord(line.end.x);
        let y1 = scale_coord(line.end.y);
        draw_line_bresenham(&mut buffer, x0 as i32, y0 as i32, x1 as i32, y1 as i32, 255);
    }
    buffer
}

pub fn render_target_glyph(font_path: &str, c: char) -> Result<PixelBuffer, String> {
    let font_bytes = fs::read(font_path).map_err(|e| format!("Failed to read font file: {}", e))?;
    let font = Font::from_bytes(font_bytes.as_slice(), FontSettings::default())
        .map_err(|e| format!("Failed to load font: {}", e))?;

    let raster_px_size = (RENDER_SIZE as f32 * RASTER_FONT_SIZE_RATIO).max(1.0);
    let line_metrics = font
        .horizontal_line_metrics(raster_px_size)
        .ok_or_else(|| format!("Failed to get line metrics for character: {}", c))?;

    let typographic_height = line_metrics.ascent - line_metrics.descent;
    let margin_y = (RENDER_SIZE as f32 - typographic_height) / 2.0;
    let canvas_baseline_y = (margin_y + line_metrics.ascent).round() as u32;

    let (metrics, bitmap) = font.rasterize(c, raster_px_size);

    let mut target_buffer = create_pixel_buffer();
    let canvas_glyph_start_x = ((RENDER_SIZE as i32 - metrics.width as i32) / 2) + metrics.xmin;
    let glyph_top_relative_to_baseline = metrics.ymin + metrics.height as i32;
    let canvas_glyph_start_y = canvas_baseline_y as i32 - glyph_top_relative_to_baseline;

    blit_bitmap(
        &mut target_buffer,
        &bitmap,
        canvas_glyph_start_x as u32,
        canvas_glyph_start_y as u32,
        metrics.width as u32,
        metrics.height as u32,
    );
    Ok(target_buffer)
}

fn blit_bitmap(
    buffer: &mut PixelBuffer,
    bitmap: &Vec<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    for j in 0..height {
        for i in 0..width {
            let pixel = bitmap[(j * width + i) as usize];
            if pixel > 0 {
                set_pixel(buffer, x + i, y + j, pixel);
            }
        }
    }
}

pub fn save_buffer(buffer: &PixelBuffer, filename: &str) -> Result<(), String> {
    GrayImage::from_raw(RENDER_SIZE, RENDER_SIZE, buffer.clone())
        .ok_or_else(|| "Failed to create image from buffer".to_string())?
        .save(filename)
        .map_err(|e| format!("Failed to save image: {}", e))?;
    Ok(())
}

pub fn calculate_mse(buffer_a: &PixelBuffer, buffer_b: &PixelBuffer) -> f64 {
    assert_eq!(buffer_a.len(), buffer_b.len(), "Buffer sizes must match!");
    let mut sum_squared_error = 0.0;

    for (i, pixel) in buffer_a.iter().enumerate() {
        let diff = (*pixel as f64) - (buffer_b[i] as f64);
        sum_squared_error += diff * diff;
    }

    sum_squared_error / buffer_a.len() as f64
}
