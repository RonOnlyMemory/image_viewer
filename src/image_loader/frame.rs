


#[derive(Clone, Debug)]
pub struct Frame {
	pub delta: f64,
	pub rgba8: Box<[u8]>,
}

impl Frame {
	pub fn new(delta: f64, data: &[u8]) -> Self {
		Self {
			delta,
			rgba8: Box::from(data),
		}
	}
}
