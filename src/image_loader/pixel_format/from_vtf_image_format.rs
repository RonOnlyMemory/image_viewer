use super::PixelFormat;



impl From<vtf::ImageFormat> for PixelFormat {
	fn from(value: vtf::ImageFormat) -> Self {
		use vtf::ImageFormat;
		match value {
			ImageFormat::Dxt5 => PixelFormat::Dxt5,
			ImageFormat::Dxt1 => PixelFormat::Dxt1,
			ImageFormat::Dxt3 => PixelFormat::Dxt3,
			ImageFormat::Rgb888 => PixelFormat::Rgb8,
			ImageFormat::Rgba8888 => PixelFormat::Rgba8,
			ImageFormat::Bgr888 => PixelFormat::Bgr8,
			ImageFormat::Bgra8888 => PixelFormat::Bgra8,
			ImageFormat::A8 => PixelFormat::A8,
			ImageFormat::Abgr8888 => PixelFormat::Abgr8,
			ImageFormat::Argb8888 => PixelFormat::Argb8,
			ImageFormat::Bgr565 => PixelFormat::Bgr565,
			ImageFormat::Bgr888Bluescreen => PixelFormat::Bgr8Bluescreen,
			ImageFormat::Bgra4444 => PixelFormat::Bgra4,
			ImageFormat::Bgra5551 => PixelFormat::Bgra5551,
			ImageFormat::Bgrx5551 => PixelFormat::Bgrx5551,
			ImageFormat::Bgrx8888 => PixelFormat::Bgrx8,
			ImageFormat::Dxt1Onebitalpha => PixelFormat::Dxt1Onebitalpha,
			ImageFormat::I8 => PixelFormat::L8,
			ImageFormat::Ia88 => PixelFormat::La8,
			ImageFormat::P8 => PixelFormat::P8,
			ImageFormat::Rgb565 => PixelFormat::Rgb565,
			ImageFormat::Rgb888Bluescreen => PixelFormat::Rgb8Bluescreen,
			ImageFormat::Rgba16161616 => PixelFormat::Rgba16,
			ImageFormat::Rgba16161616f => PixelFormat::Rgba16f,
			ImageFormat::Uv88 => PixelFormat::Uv8,
			ImageFormat::Uvlx8888 => PixelFormat::Uvlx8,
			ImageFormat::Uvwq8888 => PixelFormat::Uvwq8,
			ImageFormat::None => panic!(),
		}
	}
}
