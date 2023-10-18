pub mod update_bottom_bar;
pub mod update_main_panel;
pub mod update_top_bar;

use std::{fs, path::PathBuf, ffi::OsStr};

use egui::epaint::Vec2;
use image::ImageFormat;
use native_dialog::FileDialog;

use crate::{animation_player::AnimationPlayer, frame::Frame};

pub struct App {
	pub pos: Vec2,
	pub target_pos: Vec2,
	pub scale: f32,
	pub scale_target: f32,
	files: Vec<PathBuf>,
	file_index: usize,
	image_size: Vec2,
	animation_player: AnimationPlayer,
	pub close_time: Option<f64>,
}

impl App {
	pub fn new() -> Self {
		let mut args = std::env::args_os();
		args.next();
		let path = args.next()
			.map(|a| a.to_str().unwrap().to_string())
			.unwrap_or_else(|| FileDialog::new().show_open_single_file().unwrap().unwrap().to_string_lossy().to_string());
		let p2 = std::path::Path::new(&path);
		let mut files = Vec::new();
		let mut iter = p2.ancestors();
		iter.next();
		let mut file_index = 0;
		let mut c = 0;
		for p in fs::read_dir(iter.next().unwrap()).unwrap().into_iter() {
			let p = p.unwrap();
			let p3 = p.path();
			if !valid_extension(p.path().extension()) {
				continue;
			}
			files.push(p3.clone());
			if p3 == p2 {
				file_index = c;
			}
			c += 1;
		}
		Self {
			pos: Vec2::ZERO,
			target_pos: Vec2::ZERO,
			scale: 0.0,
			scale_target: 1.0,
			files,
			file_index,
			image_size: Vec2::ZERO,
			animation_player: AnimationPlayer::new(),
			close_time: None,
		}
	}
	pub fn set_new_image_path(&mut self, path: String) {
		let p2 = std::path::Path::new(&path);
		let mut files = Vec::new();
		let mut iter = p2.ancestors();
		iter.next();
		let mut file_index = 0;
		let mut c = 0;
		for p in fs::read_dir(iter.next().unwrap()).unwrap().into_iter() {
			let p = p.unwrap();
			let p3 = p.path();
			if !valid_extension(p.path().extension()) {
				continue;
			}
			files.push(p3.clone());
			if p3 == p2 {
				file_index = c;
			}
			c += 1;
		}
		self.file_index = file_index;
		self.files = files;
		self.animation_player.clear();
	}
	pub fn add_file_offset(&mut self, offset: isize) {
		self.file_index = ((self.file_index as isize +self.files.len() as isize +offset) as usize)%self.files.len();
		self.animation_player.clear();
	}
	pub fn path(&self) -> &PathBuf {
		&self.files[self.file_index]
	}
}

impl App {
	pub fn image_res(&self) -> Vec2 {
		self.image_size
	}
	pub fn image_size(&self) -> Vec2 {
		self.scale*self.image_size
	}
	pub fn image_target_size(&self) -> Vec2 {
		self.scale_target*self.image_size
	}
	pub fn image_pos(&self) -> Vec2 {
		self.scale*self.pos
	}
}

impl App {
	pub fn fit_image(&mut self, frame: &mut Frame) {
		let res = frame.screen_res_2();
		let image_res = self.image_res();
		let mir = image_res.x.min(image_res.y);
		let x = res.x/image_res.x;
		let y = res.y/image_res.y;
		self.scale_target = (x.min(y)*0.8).min(1.0).max((256.0 +64.0)/mir);
		self.target_pos = Vec2::ZERO;
		self.update_window_size(frame);
	}
	pub fn scale_pixel_perfect(&mut self, frame: &mut Frame) {
		self.target_pos = self.target_pos/self.scale_target;
		self.scale_target = 1.0;
		self.update_window_size(frame);
	}
	pub fn update_window_size(&self, frame: &mut Frame) {
		if !frame.window.is_maximized() {
			let ws = frame.window_size();
			if let Some(wp) = frame.window_position() {
				frame.set_window_position(wp -self.image_target_size()/2.0 +ws/2.0);
				frame.set_window_size(self.image_target_size());
			} else {
				frame.set_centered();
			}
		}
	}
}

impl App {
	pub fn update(&mut self, ctx: &egui::Context, frame: &mut crate::Frame) {
		if !frame.focus() && frame.states.focus {
			frame.states.cursor_visible = false;
			frame.window.set_cursor_visible(false);
		}
		egui::CentralPanel::default().show(ctx, |_ui| {});
		self.update_main_panel(ctx, frame);
		self.update_bottom_bar(ctx, frame);
	}
}

fn valid_extension(p: Option<&OsStr>) -> bool {
	if p.is_none() {
		return false;
	}
	let p = p.unwrap();
	let p = p.to_str().unwrap().to_ascii_lowercase();
	match &p[..] {
		"svg" => true,
		"jxl" => true,
		"jfif" => true,
		"vtf" => true,
		"dds" => true,
		_ => ImageFormat::from_extension(p).map(|_| true).unwrap_or(false),
	}
}
