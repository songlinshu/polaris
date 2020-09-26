use crate::directories::PolarisDirectories;
use anyhow::*;
use app_dirs::{app_root, AppDataType, AppInfo};
use std::fs;
use std::path::PathBuf;

pub struct Directories {}

impl Directories {
	fn get_data_root() -> Result<PathBuf> {
		let app_info = AppInfo {
			name: "Polaris",
			author: "Permafrost",
		};

		if let Ok(root) = app_root(AppDataType::UserData, &app_info) {
			fs::create_dir_all(&root)?;
			return Ok(root);
		}

		bail!("Could not retrieve data directory root");
	}
}

impl PolarisDirectories for Directories {
	fn get_web_directory() -> Result<PathBuf> {
		Directories::get_data_root().map(|mut p| {
			p.push("web");
			p
		})
	}

	fn get_swagger_directory() -> Result<PathBuf> {
		Directories::get_data_root().map(|mut p| {
			p.push("swagger");
			p
		})
	}

	fn get_db_directory() -> Result<PathBuf> {
		Directories::get_data_root()
	}

	fn get_log_directory() -> Result<PathBuf> {
		Directories::get_data_root()
	}

	fn get_thumbnail_directory() -> Result<PathBuf> {
		Directories::get_data_root().map(|mut p| {
			p.push("thumbnails");
			p
		})
	}
}