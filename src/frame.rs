use egui::{Vec2, ColorImage};
use winit::{dpi::{PhysicalSize, PhysicalPosition}, window::BadIcon, platform::windows::WindowExtWindows};

use crate::AppStates;

pub struct Frame<'a> {
	pub window: &'a winit::window::Window,
	pub states: &'a mut AppStates,
	pub close: bool,
	pub delta: f64,
	pub cursor_pos: Option<Vec2>,
	pub fix_delta: bool,
}

impl<'a> Frame<'a> {
	pub fn new(window: &'a winit::window::Window, states: &'a mut AppStates) -> Self {
		Self {
			window,
			states,
			close: false,
			delta: 0.0,
			cursor_pos: None,
			fix_delta: false,
		}
	}
	pub fn window_size(&self) -> Vec2 {
		let (w, h): (u32, u32) = self.window.inner_size().into();
		Vec2::new(w as _, h as _)
	}
	pub fn set_maximized(&self, value: bool) {
		self.window.set_maximized(value);
	}
	pub fn set_window_size(&self, Vec2 { x, y }: Vec2) {
		self.window.set_inner_size(PhysicalSize::new(x, y));
	}
	pub fn set_centered(&self) {
		let (w, h): (u32, u32) = self.window.current_monitor().unwrap().size().into();
		let s = Vec2::new(w as _, h as _);
		let Vec2 { x, y } = s/2.0;
		let Vec2 { x: x2, y: y2 } = self.window_size()/2.0;
		self.window.set_outer_position(PhysicalPosition::new(x -x2, y -y2));
	}
	pub fn close(&mut self) {
		self.close = true;
	}
	pub fn focus(&self) -> bool {
		self.states.focus_timer > 0.0
	}
	pub fn set_window_position(&self, p: Vec2) {
		self.window.set_outer_position(PhysicalPosition::<f32>::from((p.x, p.y)));
	}
	pub fn window_position(&self) -> Option<Vec2> {
		let a = self.window.outer_position().ok()?;
		let p = Vec2::new(a.x as _, a.y as _);
		Some(p)
	}
	pub fn screen_res(&self) -> Option<Vec2> {
		let (w, h): (u32, u32) = self.window.primary_monitor()?.size().into();
		Some(Vec2::new(w as _, h as _))
	}
	pub fn screen_res_2(&self) -> Vec2 {
		self.screen_res().unwrap_or_else(|| self.window_size())
	}
	pub fn set_icon(&self, img: &ColorImage) -> Result<(), BadIcon> {
		let [w, h] = img.size;
		let mut data = Vec::<u8>::with_capacity(48*48*4);
		let aspect = w as f32/h as f32;
		for y in 0..48 {
			let y = y as f32;
			let y = y -24.0 +(1.0/aspect).min(1.0)*24.0;
			let y = y*h as f32/48.0;
			let y = y*aspect.max(1.0);
			let y = y as isize;
			for x in 0..48 {
				let x = x as f32;
				let x = x -24.0 +aspect.min(1.0)*24.0;
				let x = x*w as f32/48.0;
				let x = x*(1.0/aspect).max(1.0);
				let x = x as isize;
				if x >= 0 && y >= 0 && x < w as isize && y < h as isize {
					let p = img.pixels[y as usize*w +x as usize];
					data.push(p.r());
					data.push(p.g());
					data.push(p.b());
					data.push(p.a());
				} else {
					for _ in 0..4 {
						data.push(0);
					}
				}
			}
		}
		let icon = winit::window::Icon::from_rgba(data, 48, 48)?;
		self.window.set_taskbar_icon(Some(icon));
		Ok(())
	}
}
