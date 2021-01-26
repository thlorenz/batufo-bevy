#[allow(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    dead_code
)]
pub fn round(n: f32, decimals: usize) -> f32 {
    let factor = 10_u64.pow(decimals as u32) as f64;
    ((f64::from(n) * factor).round() / factor) as f32
}
