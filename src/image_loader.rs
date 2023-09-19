pub mod compression;
pub mod frame;
pub mod image;
pub mod pixel_format;

use std::path::Path;

use self::image::Image;

pub fn load(data: &[u8]) -> Result<Image, String> {
	let format = ::image::guess_format(data).ok();
	match format {
		Some(::image::ImageFormat::Gif) => Image::load_gif(data).ok_or_else(|| "gif error".to_string()),
		Some(::image::ImageFormat::Png) => {
			let img = Image::load_png(data);
			if img.is_some() {
				img.ok_or_else(|| "png error".to_string())
			} else {
				Image::load_img(data).ok_or_else(|| "png img error".to_string())
			}
		},
		Some(::image::ImageFormat::WebP) => Image::load_web_p(data).ok_or_else(|| "webp error".to_string()),
		Some(::image::ImageFormat::Dds) => Image::load_dds(data).ok_or_else(|| "dds error".to_string()),
		Some(_) => Image::load_img(data).ok_or_else(|| "img error".to_string()),
		_ => {
			let img = Image::load_jxl(data);
			if img.is_some() {
				return img.ok_or_else(|| "jxl error".to_string());
			}
			let img = Image::load_svg(data);
			if img.is_some() {
				return img.ok_or_else(|| "svg error".to_string());
			}
			let img = Image::load_vtf(data);
			if img.is_some() {
				return img.ok_or_else(|| "vtf error".to_string());
			}
			let img = Image::load_wic(data);
			if img.is_some() {
				return img.ok_or_else(|| "wic error".to_string());
			}
			Err("image format not supported".to_string())
		}
	}
}

pub fn open<P>(path: P) -> Result<Image, String> where P: AsRef<Path> {
	let data = std::fs::read(path).map_err(|a| a.to_string())?;
	load(&data)
}
