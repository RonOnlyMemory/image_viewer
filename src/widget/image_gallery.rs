use egui::{Widget, Sense, Vec2, Color32};

pub struct ImageGallery<'a> {
	pub list: &'a [&'a egui::TextureHandle],
}

impl<'a> Widget for ImageGallery<'a> {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		let screen_size = ui.ctx().used_size();

		let sense = Sense::click_and_drag();
		let size = Vec2::new(screen_size.x, 48.0);
		let (rect, response) = ui.allocate_at_least(size, sense);

		let painter = ui.painter();
		let color = Color32::GREEN;
		painter.debug_rect(rect, color, "hi :DS");

		response
	}
}
