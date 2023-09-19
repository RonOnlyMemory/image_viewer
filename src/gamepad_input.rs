use egui::Vec2;
use gilrs::{Gilrs, Button, EventType, Axis};

use crate::{app::App, frame::Frame};

pub struct GamepadInput {
	pub gilrs: Gilrs,
	pub gp_ls: Vec2,
	pub gp_rs: Vec2,
}

impl GamepadInput {
	pub fn new() -> Self {
		Self {
			gilrs: gilrs::Gilrs::new().unwrap(),
			gp_ls: Vec2::ZERO,
			gp_rs: Vec2::ZERO,
		}
	}
	pub fn update(&mut self, app: &mut App, frame: &mut Frame) {
		while let Some(e) = self.gilrs.next_event() {
			match e.event {
				EventType::ButtonPressed(Button::DPadLeft, _) => {
					app.add_file_offset(-1);
				}
				EventType::ButtonPressed(Button::DPadRight, _) => {
					app.add_file_offset(1);
				}
				EventType::ButtonPressed(Button::LeftTrigger, _) => {
					app.add_file_offset(-1);
				}
				EventType::ButtonPressed(Button::RightTrigger, _) => {
					app.add_file_offset(1);
				}
				EventType::ButtonPressed(Button::West, _) => {
					app.fit_image(frame);
				}
				EventType::ButtonPressed(Button::North, _) => {
					app.scale_pixel_perfect(frame);
				}
				EventType::ButtonPressed(Button::DPadDown, _) => {
					app.fit_image(frame);
				}
				EventType::ButtonPressed(Button::DPadUp, _) => {
					app.scale_pixel_perfect(frame);
				}
				EventType::ButtonPressed(Button::RightThumb, _) => {
					app.fit_image(frame);
				}
				EventType::AxisChanged(Axis::LeftStickX, value, _) => {
					self.gp_ls.x = value;
				}
				EventType::AxisChanged(Axis::LeftStickY, value, _) => {
					self.gp_ls.y = value;
				}
				EventType::AxisChanged(Axis::RightStickX, value, _) => {
					self.gp_rs.x = value;
				}
				EventType::AxisChanged(Axis::RightStickY, value, _) => {
					self.gp_rs.y = value;
				}
				EventType::ButtonPressed(Button::East, _) => {
					app.close_time = Some(3.0);
					app.target_pos = Vec2::ZERO;
					app.scale_target = -0.3;
				}
				_ => {}
			}
		}
		if self.gp_ls.length() > 0.1 {
			let res = frame.screen_res_2();
			let p = app.pos +self.gp_ls*Vec2::new(-1.0, 1.0)*(frame.delta as f32)*res.y;
			app.pos = p;
			app.target_pos = p;
		}
		if self.gp_rs.length() > 0.1 {
			let s = 1.0 +0.000_000_1_f32.powf(frame.delta as f32)*self.gp_rs.y*0.02;
			app.target_pos = app.target_pos*s;
			app.pos = app.target_pos;
			app.scale *= s;
			app.scale_target = app.scale;
		}
	}
}
