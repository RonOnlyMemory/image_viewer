use egui::TextureHandle;



#[derive(Clone)]
pub struct Frame {
	pub delta: f64,
	pub texture: TextureHandle,
}

#[derive(Clone)]
pub struct Animation {
	pub frames: Vec<Frame>,
}
