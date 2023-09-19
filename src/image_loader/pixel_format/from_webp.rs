use super::PixelFormat;



impl From<webp_animation::ColorMode> for PixelFormat {
	fn from(value: webp_animation::ColorMode) -> Self {
		match value {
			webp_animation::ColorMode::Rgba => Self::Rgba8,
			webp_animation::ColorMode::Bgra => Self::Bgra8,
		}
	}
}
