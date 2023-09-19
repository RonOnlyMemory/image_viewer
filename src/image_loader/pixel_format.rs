pub mod from_vtf_image_format;
pub mod from_ddsfile_d3d_format;
pub mod from_wic_pixel_format;
pub mod from_webp;

use half::f16;
use rayon::{slice::ParallelSlice, prelude::ParallelIterator};

use super::compression::Compression;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PixelFormat {
	Rgb8,
	Rgba8,
	Bgr8,
	Bgra8,
	Dxt1,
	Dxt3,
	Dxt5,
	Argb8,
	Abgr8,
	Xrgb8,
	Xbgr8,
	L8,
	A8,
	Al8,
	R8,
	Gr8,
	Abgr16,
	Bgr565,
	Bgr8Bluescreen,
	Bgra4,
	Bgra5551,
	Bgrx5551,
	Bgrx8,
	Dxt1Onebitalpha,
	La8,
	P8,
	Rgb565,
	Rgb8Bluescreen,
	Rgba16,
	Rgba16f,
	Uv8,
	Uvlx8,
	Uvwq8,
	Abgr16f,
	A1R5G5B5,
	A2B10G10R10,
	Abgr32f,
	A2R10G10B10,
	Al4,
	Argb4,
	Argb8332,
	CXV8U8,
	GrGb8,
	L16,
	Q16W16V16U16,
	R5G6B5,
	RgBg8,
	UYVY,
	X1R5G5B5,
	X4R4G4B4,
	YUY2,
	Gr16,
	R16f,
	Gr16f,
	R32f,
	Gr32f,
	Bgr5,
	Bgr10,
	Rgba1010102,
	Rgba1010102XR,
	R10G10B10A2HDR10,
	Rgbx8,
	B5G6R5,
	B2G3R3A8,
	B5R5G5A1,
}

fn map_channels<'a>(
	data: &[u8],
	chunk_len: usize,
	f: impl Fn(&[u8]) -> [u8; 4] +'static +Send +Sync,
) -> Vec<u8> {
	data.par_chunks(chunk_len).flat_map(f).collect()
}

impl PixelFormat {
	pub fn convert_to_rgba8(self, res: [u32; 2], data: &[u8]) -> Option<Vec<u8>> {
		let data = match self {
			Self::Rgb8 => map_channels(data, 3, |a| [a[0], a[1], a[2], 255]),
			Self::Rgba8 => data.to_vec(),
			Self::Bgr8 => map_channels(data, 3, |a| [a[2], a[1], a[0], 255]),
			Self::Bgra8 => map_channels(data, 4, |a| [a[2], a[1], a[0], a[3]]),
			Self::Dxt1 => Compression::Bc1.decompress(res, data)?,
			Self::Dxt3 => Compression::Bc2.decompress(res, data)?,
			Self::Dxt5 => Compression::Bc3.decompress(res, data)?,
			Self::Argb8 => map_channels(data, 4, |a| [a[1], a[2], a[3], a[0]]),
			Self::Abgr8 => map_channels(data, 4, |a| [a[3], a[2], a[1], a[0]]),
			Self::Xrgb8 => map_channels(data, 3, |a| [a[1], a[2], a[3], 255]),
			Self::Xbgr8 => map_channels(data, 3, |a| [a[3], a[2], a[1], 255]),
			Self::L8 => map_channels(data, 1, |a| [a[0], a[0], a[0], 255]),
			Self::A8 => map_channels(data, 1, |a| [255, 255, 255, a[0]]),
			Self::Al8 => map_channels(data, 2, |a| [a[1], a[1], a[1], a[0]]),
			Self::R8 => map_channels(data, 1, |a| [a[0], 0, 0, 255]),
			Self::Gr8 => map_channels(data, 2, |a| [a[1], a[0], 0, 255]),
			Self::Abgr16 => map_channels(data, 8, |a| [a[1], a[3], a[5], a[7]]),
			PixelFormat::Bgr565 => todo!(),
			PixelFormat::Bgr8Bluescreen => map_channels(data, 3, |a| [a[2], a[1], a[0], 255]),
			PixelFormat::Bgra5551 => todo!(),
			PixelFormat::Bgrx5551 => todo!(),
			PixelFormat::Bgrx8 => map_channels(data, 4, |a| [a[2], a[1], a[0], 255]),
			PixelFormat::Dxt1Onebitalpha => todo!(),
			PixelFormat::La8 => map_channels(data, 2, |a| [a[0], a[0], a[0], a[1]]),
			PixelFormat::P8 => todo!(),
			PixelFormat::Rgb565 => todo!(),
			PixelFormat::Rgb8Bluescreen => map_channels(data, 3, |a| [a[0], a[1], a[2], 255]),
			PixelFormat::Rgba16 => map_channels(data, 8, |a| [a[7], a[5], a[3], a[1]]),
			PixelFormat::Rgba16f => map_channels(data, 8, |a| {
				let r = f16::from_le_bytes([a[1], a[0]]).to_f32()*255.0;
				let r = r as u8;
				let g = f16::from_le_bytes([a[3], a[2]]).to_f32()*255.0;
				let g = g as u8;
				let b = f16::from_le_bytes([a[5], a[4]]).to_f32()*255.0;
				let b = b as u8;
				let a = f16::from_le_bytes([a[7], a[6]]).to_f32()*255.0;
				let a = a as u8;
				[r, g, b, a]
			}),
			PixelFormat::Uv8 => todo!(),
			PixelFormat::Uvlx8 => todo!(),
			PixelFormat::Uvwq8 => todo!(),
			PixelFormat::Abgr16f => map_channels(data, 8, |a| {
				let r = f16::from_le_bytes([a[7], a[6]]).to_f32()*255.0;
				let r = r as u8;
				let g = f16::from_le_bytes([a[5], a[4]]).to_f32()*255.0;
				let g = g as u8;
				let b = f16::from_le_bytes([a[3], a[2]]).to_f32()*255.0;
				let b = b as u8;
				let a = f16::from_le_bytes([a[1], a[0]]).to_f32()*255.0;
				let a = a as u8;
				[r, g, b, a]
			}),
			PixelFormat::A1R5G5B5 => todo!(),
			PixelFormat::A2B10G10R10 => todo!(),
			PixelFormat::Abgr32f => map_channels(data, 16, |a| {
				let r = f32::from_le_bytes([a[15], a[14], a[13], a[12]])*255.0;
				let r = r as u8;
				let g = f32::from_le_bytes([a[11], a[10], a[9], a[8]])*255.0;
				let g = g as u8;
				let b = f32::from_le_bytes([a[7], a[6], a[5], a[4]])*255.0;
				let b = b as u8;
				let a = f32::from_le_bytes([a[3], a[2], a[1], a[0]])*255.0;
				let a = a as u8;
				[r, g, b, a]
			}),
			PixelFormat::A2R10G10B10 => todo!(),
			PixelFormat::Al4 => todo!(),
			PixelFormat::Bgra4 => map_channels(data, 2, |a| {
				let c = u16::from_le_bytes([a[0], a[1]]);
				let [b, g, r, a] = seperate_bits(c as _, [4, 4, 4, 4]);
				let (r, g, b, a) = (r as f32, g as f32, b as f32, a as f32);
				let (r, g, b, a) = (r*255.0/16.0, g*255.0/16.0, b*255.0/16.0, a*255.0/16.0);
				[r as _, g as _, b as _, a as _]
			}),
			PixelFormat::Argb8332 => todo!(),
			PixelFormat::CXV8U8 => todo!(),
			PixelFormat::GrGb8 => todo!(),
			PixelFormat::L16 => map_channels(data, 2, |a| [a[1], a[1], a[1], 255]),
			PixelFormat::Q16W16V16U16 => todo!(),
			PixelFormat::R5G6B5 => todo!(),
			PixelFormat::RgBg8 => todo!(),
			PixelFormat::UYVY => todo!(),
			PixelFormat::X1R5G5B5 => todo!(),
			PixelFormat::X4R4G4B4 => todo!(),
			PixelFormat::YUY2 => todo!(),
			PixelFormat::Gr16 => map_channels(data, 4, |a| [a[3], a[1], 0, 255]),
			PixelFormat::R16f => map_channels(data, 2, |a| {
				let r = f16::from_le_bytes([a[1], a[0]]).to_f32()*255.0;
				let r = r as u8;
				[r, 0, 0, 255]
			}),
			PixelFormat::Gr16f => map_channels(data, 4, |a| {
				let r = f16::from_le_bytes([a[3], a[2]]).to_f32()*255.0;
				let r = r as u8;
				let g = f16::from_le_bytes([a[1], a[0]]).to_f32()*255.0;
				let g = g as u8;
				[r, g, 0, 255]
			}),
			PixelFormat::R32f => map_channels(data, 4, |a| {
				let r = f32::from_le_bytes([a[3], a[2], a[1], a[0]])*255.0;
				let r = r as u8;
				[r, 0, 0, 255]
			}),
			PixelFormat::Gr32f => map_channels(data, 8, |a| {
				let r = f32::from_le_bytes([a[7], a[6], a[5], a[4]])*255.0;
				let r = r as u8;
				let g = f32::from_le_bytes([a[3], a[2], a[1], a[0]])*255.0;
				let g = g as u8;
				[r, g, 0, 255]
			}),
			PixelFormat::Bgr5 => todo!(),
			PixelFormat::Bgr10 => todo!(),
			PixelFormat::Rgba1010102 => todo!(),
			PixelFormat::Rgba1010102XR => todo!(),
			PixelFormat::R10G10B10A2HDR10 => todo!(),
			PixelFormat::Rgbx8 => map_channels(data, 4, |a| [a[0], a[1], a[2], 255]),
			PixelFormat::B5G6R5 => map_channels(data, 2, |a| {
				let c = u16::from_le_bytes([a[0], a[1]]);
				let [b, g, r, _] = seperate_bits(c as _, [5, 6, 5, 0]);
				let (r, g, b) = (r as f32, g as f32, b as f32);
				let (r, g, b) = (r*255.0/32.0, g*255.0/64.0, b*255.0/32.0);
				[r as _, g as _, b as _, 255]
			}),
			PixelFormat::Argb4 => todo!(),
			PixelFormat::B2G3R3A8 => map_channels(data, 2, |a| {
				let c = u16::from_le_bytes([a[0], a[1]]);
				let [b, g, r, a] = seperate_bits(c as _, [2, 3, 3, 8]);
				let (r, g, b, a) = (r as f32, g as f32, b as f32, a as f32);
				let (r, g, b, a) = (r*255.0/8.0, g*255.0/8.0, b*255.0/4.0, a);
				[r as _, g as _, b as _, a as _]
			}),
			PixelFormat::B5R5G5A1 => map_channels(data, 2, |a| {
				let c = u16::from_le_bytes([a[0], a[1]]);
				let [b, g, r, a] = seperate_bits(c as _, [5, 5, 5, 1]);
				let (r, g, b, a) = (r as f32, g as f32, b as f32, a as f32);
				let (r, g, b, a) = (r*255.0/32.0, g*255.0/32.0, b*255.0/32.0, a*255.0);
				[r as _, g as _, b as _, a as _]
			}),
		};
		Some(data)
	}
	pub fn pixel_size(self) -> u32 {
		match self {
			Self::Rgb8 => 3,
			Self::Bgr8 => 3,
			Self::Rgba8 => 4,
			Self::Bgra8 => 4,
			_ => unimplemented!()
		}
	}
}

fn seperate_bits(mut data: u128, channel_sizes: [u8; 4]) -> [u128; 4] {
	let mut ret = [0; 4];
	for c in 0..4 {
		if channel_sizes[c] == 0 {
			continue;
		}
		ret[c] = data & ((2 << (channel_sizes[c] -1)) -1);
		data = data >> channel_sizes[c];
	}
	ret
}
