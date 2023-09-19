use egui::{Vec2, Pos2};

use crate::{image_loader, widget::{close_button::CloseButton, image_viewer::ImageViewer}};

use super::App;

impl App {
	pub fn update_main_panel(&mut self, ctx: &egui::Context, frame: &mut crate::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let path = self.path().clone();
			let mut new_image = false;
			self.animation_player.animation.get_or_insert_with(|| {
				frame.window.set_title(path.to_str().unwrap());
				let img = image_loader::open(path.clone());
				if img.is_err() {
					return image_loader::image::Image::from_single_image([1, 1], &[255, 0, 255, 255]).to_animation(ui, "dummy");
				}
				let img = img.unwrap();
				let [w, h] = img.res;
				let res = if let Some(screen_size) = frame.screen_res() {
					screen_size
				} else {
					let (w, h): (u32, u32) = frame.window.inner_size().into();
					Vec2::new(w as _, h as _)
				};
				self.image_size = Vec2::new(w as f32, h as f32);
				self.scale_target = find_scale(res, self.image_size);
				if !frame.window.is_maximized() {
					self.scale_target = 1.0;
				}
				self.target_pos = Vec2::ZERO;

				if !frame.window.is_maximized() {
					let s = self.image_size*self.scale_target;
					frame.set_window_size(s +Vec2::new(2.0, 2.0));
					frame.set_centered();
				}

				new_image = true;

				img.to_animation(ui, path.to_str().unwrap())
			});
			if new_image {
				self.fit_image(frame);
				if frame.window.is_maximized() {
					self.scale = self.scale_target*0.7;
				} else {
					self.scale = self.scale_target;
				}
			}
			let f = self.animation_player.current_frame_clone().unwrap();
			let delta = frame.delta as f64;
			self.animation_player.advance(delta);
			let texture = &f.texture;

			let s = texture.size_vec2()*self.scale;
			let ivr = ui.add(ImageViewer {
				pos: self.pos,
				scale: Vec2::new(self.scale, self.scale),
				texture,
				background_shadow: frame.window.is_maximized(),
			});
			if ivr.double_clicked() {
				if !frame.window.is_maximized() {
					frame.set_maximized(true);
				}
				self.fit_image(frame);
			}
			if !frame.window.is_maximized() && ui.input(|i| i.pointer.primary_down()) {
				frame.states.drag_window = true;
			}
			if !frame.window.is_maximized() {
				self.target_pos = Vec2::ZERO;
			}
			if ivr.dragged() && frame.window.is_maximized() {
				self.target_pos += ivr.drag_delta();
				self.pos += ivr.drag_delta();
			}
			if ivr.clicked_elsewhere() && !ui.ctx().is_using_pointer() {
				if frame.window.is_maximized() {
					frame.set_maximized(false);
					frame.set_window_size(s +Vec2::new(2.0, 2.0));
					frame.set_centered();
					self.pos = Vec2::ZERO;
					self.target_pos = Vec2::ZERO;
				}
			}
			if frame.focus() {
				if ui.add(CloseButton).clicked() {
					if frame.window.is_maximized() {
						self.close_time = Some(0.1);
					} else {
						frame.close();
					}
					self.target_pos = Vec2::ZERO;
					self.scale_target = -0.3;
				}
			}

			if let Some(time) = &mut self.close_time {
				*time -= delta;
				if self.scale <= 0.0 {
					frame.close();
				}
			}

			if let Some(screen_res) = frame.screen_res() {
				let a = self.image_target_size() -screen_res;
				if (a.x > 0.0 || a.y > 0.0) && !frame.window.is_maximized() {
					frame.set_maximized(true);
				}
			}

			if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
				self.add_file_offset(-1);
			}
			if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
				self.add_file_offset(1);
			}

			{
				let sdy = ui.input(|i| i.scroll_delta).y;
				if sdy.abs() > 0.01 {
					let a = 1.0 +sdy*0.2/50.0;
					let res = {
						let (w, h): (u32, u32) = frame.window.inner_size().into();
						Vec2::new(w as _, h as _)
					};
					if let Some(p) = ui.input(|i| i.pointer.hover_pos()) {
						let p = p -Pos2::ZERO -res/2.0;
						self.target_pos = (self.target_pos -p)*a +p;
						self.scale_target *= a;
						if !frame.window.is_maximized() {
							self.scale = self.scale_target;
							let nws = self.image_target_size();
							if let Some(owp) = frame.window_position() {
								let ws = frame.window_size();
								let nwp = owp +ws/2.0 -nws/2.0;
								frame.set_window_position(nwp);
							}
							frame.set_window_size(nws +Vec2::new(2.0, 2.0));
						}
					}
				}
			}
			{
				if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
					if frame.window.is_maximized() {
						self.close_time = Some(0.1);
					} else {
						frame.close();
					}
				}
				if ui.input(|i| i.key_pressed(egui::Key::Space)) {
					self.fit_image(frame);
				}
				if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
					self.scale_pixel_perfect(frame);
				}
			}
			
			let delta = frame.delta as f64;
			self.scale = lerp_f64(self.scale as _, self.scale_target as _, 1.0 -0.000_000_1_f64.powf(delta)) as _;
			self.scale = self.scale.max(0.0);
			self.pos = lerp_vec_2(self.pos, self.target_pos, (1.0 -0.000_000_1_f64.powf(delta)) as f32);
		});
	}
}

fn lerp_f64(a: f64, b: f64, c: f64) -> f64 {
	a*(1.0 -c) +b*c
}

fn lerp_vec_2(a: Vec2, b: Vec2, c: f32) -> Vec2 {
	a*(1.0 -c) +b*c
}

fn extend_to_fit_scale(res: Vec2, a: Vec2) -> f32 {
	let mut s = 1.0;
	while res.x > s*a.x && res.y > s*a.y {
		s *= 2.0;
	}
	s
}

fn scale_down_til_fit(res: Vec2, a: Vec2) -> f32 {
	let mut s = 1.0;
	while res.x < s*a.x || res.y < s*a.y {
		s /= 2.0;
	}
	s
}

fn find_scale(res: Vec2, a: Vec2) -> f32 {
	let s = extend_to_fit_scale(res, a);
	scale_down_til_fit(res, a*s)*s
	
}
