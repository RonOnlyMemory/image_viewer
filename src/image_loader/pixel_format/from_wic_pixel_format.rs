use winapi::{shared::guiddef::IsEqualGUID, um::wincodec::*};
use super::PixelFormat;



impl TryFrom<winapi::shared::guiddef::GUID> for PixelFormat {
	type Error = ();
	fn try_from(value: winapi::shared::guiddef::GUID) -> Result<Self, Self::Error> {
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppRGBA) {
			return Ok(Self::Rgba8);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppBGR) {
			return Ok(Self::Bgr8);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppBGRA) {
			return Ok(Self::Bgra8);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat24bppRGB) {
			return Ok(Self::Rgb8);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat24bppBGR) {
			return Ok(Self::Bgr8);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat16bppBGR555) {
			return Ok(Self::Bgr5);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat16bppBGR565) {
			return Ok(Self::Bgr565);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat16bppBGRA5551) {
			return Ok(Self::Bgra5551);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppBGR101010) {
			return Ok(Self::Bgr10);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppRGBA1010102) {
			return Ok(Self::Rgba1010102);
		}
		if IsEqualGUID(&value, &GUID_WICPixelFormat32bppRGBA1010102XR) {
			return Ok(Self::Rgba1010102XR);
		}
		Err(())
	}
}
