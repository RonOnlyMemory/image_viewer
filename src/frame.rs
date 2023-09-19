use egui::Vec2;
use winit::dpi::{PhysicalSize, PhysicalPosition};

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
}
