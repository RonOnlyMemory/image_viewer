#![cfg_attr(
	all(
		target_os = "windows",
		not(debug_assertions),
		not(feature = "release_console"),
	),
	windows_subsystem = "windows"
)]

use std::{sync::Arc, time::Instant};

use app::App;
use egui::{Visuals, Color32, Vec2};
use frame::Frame;
use gamepad_input::GamepadInput;
use glow::HasContext;
use glutin::surface::GlSurface;
use window::SimpleWindow;
use winit::{event::{Event, WindowEvent}, dpi::PhysicalPosition};

pub mod app;
pub mod window;
pub mod widget;
pub mod animation;
pub mod animation_player;
pub mod frame;
pub mod image_loader;
pub mod gamepad_input;
pub mod valid;
pub mod async_image_loader;

pub struct AppStates {
	pub drag_window: bool,
	pub focus: bool,
	pub focus_timer: f32,
	pub cursor_visible: bool,
	pub fix_delta: bool,
}

impl AppStates {
	pub fn new() -> Self {
		Self {
			drag_window: false,
			focus: false,
			focus_timer: 3.0,
			cursor_visible: true,
			fix_delta: false,
		}
	}
	pub fn update(&mut self, delta: f32) {
		self.focus_timer -= delta;
	}
	pub fn update_focus(&mut self, window: &winit::window::Window) {
		self.focus_timer = 3.0;
		if !self.cursor_visible {
			window.set_cursor_visible(true);
		}
	}
}

fn main() {
	let SimpleWindow {
		event_loop,
		window,
		ctx,
		gl,
		surface,
	} = SimpleWindow::new().unwrap();
	let gl = Arc::new(gl);
	let mut egui_glow = egui_glow::winit::EguiGlow::new(&event_loop, gl.clone(), Some(egui_glow::ShaderVersion::Gl120));
	let mut app = App::new();
	let mut visual = Visuals::dark();
	visual.panel_fill = Color32::TRANSPARENT;
	egui_glow.egui_ctx.set_visuals(visual);
	window.set_minimized(true);
	window.set_visible(true);
	window.set_maximized(true);
	window.focus_window();
	let mut cursor_pos = None;
	let mut app_states = AppStates::new();

	let mut mouse_delta = Vec2::ZERO;

	let mut gpi = GamepadInput::new();

	let mut time = std::time::Instant::now();
	event_loop.run(move |e, _, cf| {
		match e {
			Event::DeviceEvent { event, .. } => match event {
				winit::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
					mouse_delta += Vec2::new(x as _, y as _);
				}
				_ => {}
			}
			Event::WindowEvent { event, .. } => {
				let er = egui_glow.on_event(&event);
				if er.consumed {
					return;
				}
				match event {
					WindowEvent::CursorEntered { .. } => {
						app_states.focus = true;
					}
					WindowEvent::CursorLeft { .. } => {
						app_states.focus = false;
					}
					WindowEvent::CloseRequested => {
						window.set_visible(false);
						cf.set_exit();
					}
					WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
						egui_glow.egui_ctx.set_pixels_per_point(scale_factor as _);
					}
					WindowEvent::DroppedFile(path) => {
						app.set_new_image_path(path.to_str().unwrap().to_string());
						window.focus_window();
					}
					WindowEvent::MouseInput { .. } => {
						app_states.update_focus(&window);
					}
					WindowEvent::MouseWheel { .. } => {
						app_states.update_focus(&window);
					}
					WindowEvent::CursorMoved { position, .. } => {
						let (x, y): (f32, f32) = position.into();
						app_states.update_focus(&window);
						cursor_pos = Some(Vec2::new(x, y));
					}
					_ => {}
				}
			}
			Event::MainEventsCleared | Event::RedrawRequested(_) => {
				let mut close = false;
				app_states.drag_window = false;
				let mut delta = time.elapsed().as_secs_f64();
				time = Instant::now();
				app_states.update(delta as _);
				egui_glow.run(&window, |ctx| {
					let mut frame = Frame::new(&window, &mut app_states);
					if frame.states.fix_delta {
						delta = 0.0;
						frame.states.fix_delta = false;
					}
					frame.delta = delta;
					frame.cursor_pos = cursor_pos;
					if e == Event::MainEventsCleared && window.is_maximized() {
						gpi.update(&mut app, &mut frame);
					}
					app.update(ctx, &mut frame);
					close = frame.close;
					ctx.output(|o| {
						if let Some(open_url) = o.open_url.clone() {
							let _ = webbrowser::open(&open_url.url);
						}
					});
				});
				unsafe {
					if window.is_maximized() {
						gl.clear_color(0.0, 0.0, 0.0, 0.4);
					} else {
						gl.clear_color(0.0, 0.0, 0.0, 0.0);
					}
					gl.clear(glow::COLOR_BUFFER_BIT);
				}
				egui_glow.paint(&window);
				surface.swap_buffers(&ctx).unwrap();

				if close {
					window.set_visible(false);
					cf.set_exit();
				}

				if e == Event::MainEventsCleared {
					let md = mouse_delta;
					if app_states.drag_window && window.outer_position().is_ok() {
						let owp = window.outer_position().unwrap();
						let (wx, wy): (i32, i32) = owp.into();
						let nwp = PhysicalPosition::<i32>::from((
							md.x as i32 +wx,
							md.y as i32 +wy,
						));
						window.set_outer_position(nwp);
					}
				}

				mouse_delta = Vec2::ZERO;
			}
			_ => {}
		}
	});
}
