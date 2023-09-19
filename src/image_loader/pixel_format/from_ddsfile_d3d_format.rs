use super::PixelFormat;



impl From<ddsfile::D3DFormat> for PixelFormat {
	fn from(value: ddsfile::D3DFormat) -> Self {
		use ddsfile::D3DFormat;
		match value {
			D3DFormat::R8G8B8 => PixelFormat::Bgr8,
			D3DFormat::DXT1 => PixelFormat::Dxt1,
			D3DFormat::DXT2 => PixelFormat::Dxt3,
			D3DFormat::DXT3 => PixelFormat::Dxt3,
			D3DFormat::DXT4 => PixelFormat::Dxt5,
			D3DFormat::DXT5 => PixelFormat::Dxt5,
			D3DFormat::A8R8G8B8 => PixelFormat::Bgra8,
			D3DFormat::X8B8G8R8 => PixelFormat::Rgbx8,
			D3DFormat::A8B8G8R8 => PixelFormat::Rgba8,
			D3DFormat::L8 => PixelFormat::L8,
			D3DFormat::A8 => PixelFormat::A8,
			D3DFormat::A8L8 => PixelFormat::La8,
			D3DFormat::A16B16G16R16 => PixelFormat::Abgr16,
			D3DFormat::A16B16G16R16F => PixelFormat::Abgr16f,
			D3DFormat::A1R5G5B5 => PixelFormat::B5R5G5A1,
			D3DFormat::A2B10G10R10 => PixelFormat::A2B10G10R10,
			D3DFormat::A32B32G32R32F => PixelFormat::Abgr32f,
			D3DFormat::A2R10G10B10 => PixelFormat::A2R10G10B10,
			D3DFormat::A4L4 => PixelFormat::Al4,
			D3DFormat::A4R4G4B4 => PixelFormat::Bgra4,
			D3DFormat::A8R3G3B2 => PixelFormat::B2G3R3A8,
			D3DFormat::CXV8U8 => PixelFormat::CXV8U8,
			D3DFormat::G8R8_G8B8 => PixelFormat::GrGb8,
			D3DFormat::L16 => PixelFormat::L16,
			D3DFormat::Q16W16V16U16 => PixelFormat::Q16W16V16U16,
			D3DFormat::R5G6B5 => PixelFormat::B5G6R5,
			D3DFormat::R8G8_B8G8 => PixelFormat::RgBg8,
			D3DFormat::UYVY => PixelFormat::UYVY,
			D3DFormat::X1R5G5B5 => PixelFormat::X1R5G5B5,
			D3DFormat::X4R4G4B4 => PixelFormat::X4R4G4B4,
			D3DFormat::X8R8G8B8 => PixelFormat::Xrgb8,
			D3DFormat::YUY2 => PixelFormat::YUY2,
			D3DFormat::G16R16 => PixelFormat::Gr16,
			D3DFormat::R16F => PixelFormat::R16f,
			D3DFormat::G16R16F => PixelFormat::Gr16f,
			D3DFormat::R32F => PixelFormat::R32f,
			D3DFormat::G32R32F => PixelFormat::Gr32f,
		}
	}
}
