use egui::{Sense, Vec2};

use super::App;

impl App {
	pub fn update_bottom_bar(&mut self, ctx: &egui::Context, frame: &mut crate::Frame) {
		let (w, _h): (u32, u32) = frame.window.inner_size().into();
		egui::TopBottomPanel::bottom("bottom 2")
		.show_separator_line(false)
		.show(ctx, |ui| {
			if frame.window.is_maximized() && frame.focus() {
				ui.horizontal(|ui| {
					let title = frame.window.title();
					let path = std::path::PathBuf::from(&title);
					let title_2 = path.file_name().map(|a| a.to_string_lossy().to_string()).unwrap_or("".to_string());
					let title_3 = format!("{} ({}x{})", title_2, self.image_size.x as u32, self.image_size.y as u32);
					let label = egui::Label::new(&title_3).sense(Sense::click());
					let (_p, wtg, _r) = label.layout_in_ui(ui);
					let label = egui::Label::new(&title_3).sense(Sense::click());
					let size = wtg.size();
					ui.allocate_exact_size(Vec2::new(w as f32/2.0 -size.x*1.5 as f32, 0.0), Sense { click: false, drag: false, focusable: false });
					let widget = ui.add(label);
					widget.rect.width();
					if widget.clicked() {
						_ = std::process::Command::new("explorer.exe")
							.args(&["/select,", &title])
							.spawn().expect("failed to execute process");
					}
				});
			}
			if frame.focus() {
				ui.horizontal(|ui| {
					ui.allocate_exact_size(Vec2::new(w as f32/2.0 -128.0 +32.0, 0.0), Sense { click: false, drag: false, focusable: false });
					if ui.button("<-").clicked() {
						self.add_file_offset(-1);
					}
					if ui.button("+").clicked() {
						self.scale_target *= 1.5;
						self.update_window_size(frame);
					}
					if ui.button("-").clicked() {
						self.scale_target /= 1.5;
						self.update_window_size(frame);
					}
					if ui.button("1:1").clicked() {
						self.scale_pixel_perfect(frame);
					}
					if ui.button("fit").clicked() {
						self.fit_image(frame);
					}
					if ui.button("->").clicked() {
						self.add_file_offset(1);
					}
					ui.hyperlink_to("♥ donate", "https://www.paypal.me/RonaldKoroll");
				});
			}
		});
	}
}
