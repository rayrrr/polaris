use image;
use image::GenericImage;
use image::ImageBuffer;
use image::FilterType;
use image::imageops::resize;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::fs::DirBuilder;
use std::hash::{Hash, Hasher};
use std::path::*;

use errors::*;
use utils;

const THUMBNAILS_PATH: &'static str = "thumbnails";

fn hash(path: &Path, dimension: u32) -> u64 {
	let path_string = path.to_string_lossy();
	let hash_input = format!("{}:{}", path_string, dimension.to_string());
	let mut hasher = DefaultHasher::new();
	hash_input.hash(&mut hasher);
	hasher.finish()
}

pub fn get_thumbnail(real_path: &Path, max_dimension: u32) -> Result<PathBuf> {

	let mut out_path = utils::get_cache_root()?;
	out_path.push(THUMBNAILS_PATH);

	let mut dir_builder = DirBuilder::new();
	dir_builder.recursive(true);
	dir_builder.create(out_path.as_path())?;

	let source_image = image::open(real_path)?;
	let (source_width, source_height) = source_image.dimensions();
	let cropped_dimension = cmp::max(source_width, source_height);
	let out_dimension = cmp::min(max_dimension, cropped_dimension);

	let hash = hash(real_path, out_dimension);
	out_path.push(format!("{}.png", hash.to_string()));

	if !out_path.exists() {
		let source_aspect_ratio: f32 = source_width as f32 / source_height as f32;
		if source_aspect_ratio < 0.8 || source_aspect_ratio > 1.2 {
			let mut cropped_image = ImageBuffer::new(cropped_dimension, cropped_dimension);
			cropped_image.copy_from(&source_image,
			                        (cropped_dimension - source_width) / 2,
			                        (cropped_dimension - source_height) / 2);
			let out_image = resize(&cropped_image,
			                       out_dimension,
			                       out_dimension,
			                       FilterType::Lanczos3);
			out_image.save(out_path.as_path())?;
		} else {
			let out_image = resize(&source_image,
			                       out_dimension,
			                       out_dimension,
			                       FilterType::Lanczos3);
			out_image.save(out_path.as_path())?;
		}
	}

	Ok(out_path)
}
