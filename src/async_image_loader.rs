use std::{sync::{Arc, RwLock, Mutex}, collections::{HashMap, BTreeMap}};

use crate::image_loader::{self, image::Image};

pub struct AsyncImageLoader {
	index: Arc<Mutex<usize>>,
	map: Arc<RwLock<HashMap<String, Arc<Image>>>>,
}

impl AsyncImageLoader {
	pub fn new(path_list: Vec<String>, index: usize) -> Self {
		let map = Arc::new(RwLock::new(HashMap::new()));
		let index = Arc::new(Mutex::new(index));
		{
			let map = map.clone();
			let index = index.clone();
			let mut path_list: BTreeMap<usize, String> = path_list.into_iter().enumerate().collect();
			std::thread::spawn(move || {
				loop {
					let index: usize = *index.lock().unwrap();
					if let Some(path) = pop_value_by_closest_key(&mut path_list, index) {
						if let Ok(img) = image_loader::open(&path) {
							map.write().unwrap().insert(path, Arc::new(img));
						} else {
							continue;
						}
					} else {
						return;
					}
				}
			});
		}
		Self {
			index,
			map,
		}
	}
	pub fn set_index(&mut self, index: usize) {
		*self.index.lock().unwrap() = index;
	}
	pub fn get(&self, path: &str) -> Option<Arc<Image>> {
		let map = self.map.read().unwrap();
		map.get(path).cloned()
		
	}
}

fn pop_value_by_closest_key(map: &mut BTreeMap<usize, String>, key: usize) -> Option<String> {
	let keys: Vec<usize> = map.keys().cloned().collect();
	let key = binary_search_closest_key(&keys, key)?;
	map.remove(&key)
}

fn binary_search_closest_key(keys: &[usize], to_find: usize) -> Option<usize> {
	let len = keys.len();
	let (left, right) = keys.split_at(len/2);
	let left_key = left.last();
	let right_key = right.first();
	if left_key.is_none() {
		return right_key.cloned();
	}
	if right_key.is_none() {
		return left_key.cloned();
	}
	let left_dif = left_key.unwrap().abs_diff(to_find);
	let right_dif = right_key.unwrap().abs_diff(to_find);
	if left_dif <= right_dif {
		binary_search_closest_key(left, to_find)
	} else {
		binary_search_closest_key(right, to_find)
	}
}
