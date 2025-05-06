pub fn calculate_mse(buffer_a: &PixelBuffer, buffer_b: &PixelBuffer) -> f64 {
    assert_eq!(buffer_a.len(), buffer_b.len(), "Buffer sizes must match!");
    let mut sum_squared_error = 0.0;

    for (i, pixel) in buffer_a.iter().enumerate() {
        let diff = (*pixel as f64) - (buffer_b[i] as f64);
        sum_squared_error += diff * diff;
    }

    sum_squared_error / buffer_a.len() as f64
}
