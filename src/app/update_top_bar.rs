use egui::Sense;

use super::App;



impl App {
	pub fn update_top_bar(&mut self, ctx: &egui::Context, frame: &mut crate::Frame) {
		egui::TopBottomPanel::top("top")
		.show_separator_line(false)
		.show(ctx, |ui| {
			let ws = frame.window_size();
			if frame.window.is_maximized() || frame.states.focus_timer < 2.0 || ws.x < 512.0 {
				return;
			}
			ui.horizontal(|ui| {
				let title = frame.window.title();
				let path = std::path::PathBuf::from(&title);
				let title_2 = path.file_name().map(|a| a.to_string_lossy().to_string()).unwrap_or("".to_string());
				let title_3 = format!("{} ({}x{})", title_2, self.image_size.x as u32, self.image_size.y as u32);
				let label = egui::Label::new(&title_3).sense(Sense::click());
				let widget = ui.add(label);
				widget.rect.width();
				if widget.clicked() {
					_ = std::process::Command::new("explorer.exe")
						.args(&["/select,", &title])
						.spawn().expect("failed to execute process");
				}
			});
		});
	}
}
