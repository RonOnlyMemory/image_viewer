use egui::ColorImage;
use std::sync::Arc;



#[derive(Clone)]
pub struct Frame {
	pub delta: f64,
	pub color_image: Arc<ColorImage>,
}

impl Frame {
	pub fn new(delta: f64, [w, h]: [u32; 2], data: &[u8]) -> Self {
		Self {
			delta,
			color_image: Arc::new(ColorImage::from_rgba_unmultiplied([w as _, h as _], data)),
		}
	}
}
