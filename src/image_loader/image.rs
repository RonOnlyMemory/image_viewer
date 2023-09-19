use egui::{Ui, ColorImage, TextureOptions};
use image::{codecs::{png::PngDecoder, gif::GifDecoder}, AnimationDecoder};
use jxl_oxide::{image::SizeHeader, JxlImage};
use resvg::usvg::TreeParsing;

use crate::animation::Animation;

use super::{frame::Frame, pixel_format::PixelFormat};

#[derive(Clone, Debug)]
pub struct Image {
	pub res: [u32; 2],
	pub frames: Box<[Frame]>,
}

impl Image {
	pub fn from_single_frame(res: [u32; 2], frame: Frame) -> Self {
		Self {
			res,
			frames: Box::new([frame]),
		}
	}
	pub fn from_multiple_frames(res: [u32; 2], frames: &[Frame]) -> Self {
		Self {
			res,
			frames: Box::from(frames),
		}
	}
	pub fn from_single_image(res: [u32; 2], data: &[u8]) -> Self {
		Self::from_single_frame(res, Frame::new(1.0, data))
	}
	pub fn load_img(data: &[u8]) -> Option<Self> {
		let img = image::load_from_memory(data).ok()?;
		let w = img.width();
		let h = img.height();
		let data = img.into_rgba8();
		let data = data.as_raw();
		Some(Self::from_single_frame([w, h], Frame::new(1.0, data)))
	}
	pub fn load_jxl(data: &[u8]) -> Option<Self> {
		let mut img = JxlImage::from_reader(data).ok()?;
		let SizeHeader { width, height, .. } = img.image_header().size;
		let mut r = img.renderer();
		let mut b = None;
		while {
			let a = r.render_next_frame().ok()?;
			match a {
				jxl_oxide::RenderResult::Done(r) => {
					b = Some(r);
					false
				},
				_ => true,
			}
		} {}
		let a = b.unwrap();
		let i = a.image();
		let d = i.buf();
		let d: Vec<u8> = d.into_iter().map(|a| (a*255.0) as u8).collect();
		Some(Self::from_single_image([width, height], &d))
	}
	pub fn load_web_p(data: &[u8]) -> Option<Self> {
		let dec = webp_animation::Decoder::new(data).ok()?;
		let frames = dec.into_iter();
		let mut iter = frames.peekable();
		let frame = iter.peek();
		if frame.is_none() {
			return None;
		}
		let frame = frame.unwrap();
		let (w, h) = frame.dimensions();
		let res = [w, h];
		let mut new_frames = Vec::new();
		let mut last_timestamp = -16;
		for frame in iter {
			let timestamp = frame.timestamp();
			let delta = timestamp -last_timestamp;
			last_timestamp = timestamp;
			let pixel_format = PixelFormat::from(frame.color_mode());
			let data = frame.data();
			let data = pixel_format.convert_to_rgba8(res, data)?;
			new_frames.push(Frame::new(delta.max(1) as f64/1000.0, &data));
		}
		Some(Self::from_multiple_frames(res, &new_frames))
	}
	pub fn load_gif(data: &[u8]) -> Option<Self> {
		let dec = GifDecoder::new(data).ok()?;
		let frames = dec.into_frames();
		let mut frames = frames.peekable();
		if frames.peek().is_none() {
			return None;
		}
		let frame = frames.peek().unwrap();
		let img = frame.as_ref().unwrap();
		let img = img.buffer().clone();
		let res = [img.width() as u32, img.height() as u32];
		let frames = frames.into_iter().map(|frame| {
			let frame = frame.unwrap();
			let data = frame.buffer();
			let (a, b) = frame.delay().numer_denom_ms();
			let mut delta = a as f64/b as f64/1000.0;
			if a == 0 || b == 0 {
				delta = 1.0/10.0;
			}
			Frame::new(delta, data)
		}).collect::<Vec<_>>();
		Some(Self::from_multiple_frames(res, &frames))
	}
	pub fn load_png(data: &[u8]) -> Option<Self> {
		let dec = PngDecoder::new(data).ok()?;
		if !dec.is_apng() {
			return None;
		}
		let dec = dec.apng();
		let frames = dec.into_frames();
		let mut frames = frames.peekable();
		if frames.peek().is_none() {
			return None;
		}
		let frame = frames.peek().unwrap();
		let img = frame.as_ref().unwrap();
		let img = img.buffer().clone();
		let res = [img.width() as u32, img.height() as u32];
		let frames = frames.into_iter().map(|frame| {
			let frame = frame.unwrap();
			let data = frame.buffer();
			let (a, b) = frame.delay().numer_denom_ms();
			let mut delta = a as f64/b as f64/1000.0;
			if a == 0 || b == 0 {
				delta = 1.0/10.0;
			}
			Frame::new(delta, data)
		}).collect::<Vec<_>>();
		Some(Self::from_multiple_frames(res, &frames))
	}
	pub fn load_svg(data: &[u8]) -> Option<Self> {
		use resvg::tiny_skia::Pixmap;
		use resvg::usvg::Transform;
		let options = resvg::usvg::Options::default();
		let tree = resvg::usvg::Tree::from_data(&data, &options).ok()?;
		let width = tree.size.width() as u32;
		let height = tree.size.height() as u32;
		let mut pixmap = Pixmap::new(width, height)?;
		let tree = resvg::Tree::from_usvg(&tree);
		let transform = Transform::default();
		tree.render(transform, &mut pixmap.as_mut());
		Some(Self::from_single_image([width, height], pixmap.data()))
	}
	pub fn load_wic(data: &[u8]) -> Option<Self> {
		let decoder = native_windows_gui::ImageDecoder::new().ok()?;
		let image = decoder.from_stream(data).ok()?;
		let frame = image.frame(0).ok()?;
		let (w, h) = frame.size();
		let res = [w, h];
		let pixel_format = frame.pixel_format();
		let mut frames = Vec::new();
		for c in 0..image.frame_count() {
			let frame = image.frame(c).ok()?;
			let pixel_format = PixelFormat::try_from(pixel_format).ok()?;
			let data = frame.pixels(pixel_format.pixel_size()).ok()?;
			let data = pixel_format
				.convert_to_rgba8(res, &data)?;
			frames.push(Frame {
				delta: 1.0/60.0,
				rgba8: data.into_boxed_slice(),
			});
		}
		Some(Self::from_multiple_frames([w, h], &frames))
	}
	pub fn load_vtf(data: &[u8]) -> Option<Self> {
		let mut data = data.to_vec();
		let vtf = vtf::from_bytes(&mut data).ok()?;
		let a = vtf.highres_image.get_frame(0).unwrap();
		let width = vtf.highres_image.width as u32;
		let height = vtf.highres_image.height as u32;
		let res = [width, height];
		let data = PixelFormat::from(vtf.header.highres_image_format).convert_to_rgba8(res, a).unwrap();
		Some(Self::from_single_image([vtf.highres_image.width as _, vtf.highres_image.height as _], &data))
	}
	pub fn load_dds(data: &[u8]) -> Option<Self> {
		let dds = ddsfile::Dds::read(data).ok()?;
		let width = dds.get_width();
		let height = dds.get_height();
		let res = [width, height];
		let data = PixelFormat::from(dds.get_d3d_format().unwrap()).convert_to_rgba8(res, &dds.data).unwrap();
		Some(Self::from_single_image([width, height], &data))
	}
}

impl Image {
	pub fn to_animation(&self, ui: &Ui, name: &str) -> Animation {
		let options = TextureOptions {
			magnification: egui::TextureFilter::Nearest,
			minification: egui::TextureFilter::Linear,
		};
		let [w, h] = self.res;
		let res = [w as usize, h as usize];
		Animation {
			frames: self.frames.iter().enumerate().map(|(c, a)| crate::animation::Frame {
				delta: a.delta,
				texture: {
					let img = ColorImage::from_rgba_unmultiplied(res, &a.rgba8);
					let texture = ui.ctx().load_texture(&format!("{} {}", name, c), img, options);
					texture
				},
			}).collect(),
		}
	}
}
