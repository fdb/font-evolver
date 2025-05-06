use crate::{
    constants::{MAX_COORD, RENDER_SIZE},
    genotype::Genotype,
};
use fontdue::{Font, FontSettings};
use image::GrayImage;
use std::fs;

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

fn set_pixel_fraction(buffer: &mut PixelBuffer, x: i32, y: i32, intensity_fraction: f32) {
    if x >= 0 && x < RENDER_SIZE as i32 && y >= 0 && y < RENDER_SIZE as i32 {
        let color_value = (intensity_fraction * 255.0).round() as u8;
        let index = (y as u32 * RENDER_SIZE + x as u32) as usize;
        buffer[index] = buffer[index].max(color_value);
    }
}

fn scale_coord(coord: i32) -> f32 {
    coord as f32 / MAX_COORD as f32 * (RENDER_SIZE - 1) as f32
}

fn ipart(x: f32) -> i32 {
    x.floor() as i32
}

fn fpart(x: f32) -> f32 {
    x - x.floor()
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

fn draw_line_wu(buffer: &mut PixelBuffer, mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32) {
    let steep = (y1 - y0).abs() > (x1 - x0).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let gradient = if dx.abs() < 1e-6 {
        // Avoid division by zero if dx is very small
        if dy >= 0.0 {
            1.0e6
        } else {
            -1.0e6
        }
    } else {
        dy / dx
    };

    // First endpoint
    let x_end_0 = x0.round();
    let y_end_0 = y0 + gradient * (x_end_0 - x0);
    let x_gap_0 = rfpart(x0 + 0.5);
    let x_px_0 = x_end_0 as i32;
    let y_px_0 = ipart(y_end_0);

    if steep {
        set_pixel_fraction(buffer, y_px_0, x_px_0, rfpart(y_end_0) * x_gap_0);
        set_pixel_fraction(buffer, y_px_0 + 1, x_px_0, fpart(y_end_0) * x_gap_0);
    } else {
        set_pixel_fraction(buffer, x_px_0, y_px_0, rfpart(y_end_0) * x_gap_0);
        set_pixel_fraction(buffer, x_px_0, y_px_0 + 1, fpart(y_end_0) * x_gap_0);
    }

    let mut inter_y = y_end_0 + gradient;

    // Second endpoint
    let x_end_1 = x1.round();
    let y_end_1 = y1 + gradient * (x_end_1 - x1);
    let x_gap_1 = rfpart(x1 + 0.5);
    let x_px_1 = x_end_1 as i32;
    let y_px_1 = ipart(y_end_1);

    if steep {
        set_pixel_fraction(buffer, y_px_1, x_px_1, rfpart(y_end_1) * x_gap_1);
        set_pixel_fraction(buffer, y_px_1 + 1, x_px_1, fpart(y_end_1) * x_gap_1);
    } else {
        set_pixel_fraction(buffer, x_px_1, y_px_1, rfpart(y_end_1) * x_gap_1);
        set_pixel_fraction(buffer, x_px_1, y_px_1 + 1, fpart(y_end_1) * x_gap_1);
    }

    // Iterate from x_px_0 + 1 to x_px_1 -1
    if steep {
        for x in (x_px_0 + 1)..x_px_1 {
            set_pixel_fraction(buffer, ipart(inter_y), x, rfpart(inter_y));
            set_pixel_fraction(buffer, ipart(inter_y) + 1, x, fpart(inter_y));
            inter_y += gradient;
        }
    } else {
        for x in (x_px_0 + 1)..x_px_1 {
            set_pixel_fraction(buffer, x, ipart(inter_y), rfpart(inter_y));
            set_pixel_fraction(buffer, x, ipart(inter_y) + 1, fpart(inter_y));
            inter_y += gradient;
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
        draw_line_wu(&mut buffer, x0, y0, x1, y1);
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
