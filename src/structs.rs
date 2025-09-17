use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BurningTextForm {
	logo_id: u64,
	pub text: String,
	pub font_size: u32,
	#[serde(rename = "Color1_color")]
	pub font_color: String,
	#[serde(rename = "Integer1")]
	pub burn_angle: u32,
	#[serde(rename = "Boolean1")]
	pub transparent: bool,
	#[serde(rename = "Integer9")]
	pub alignment: u8, // ranges from 0 to 8 inclusive
	#[serde(rename = "Integer11")]
	pub image_width: u32,
	#[serde(rename = "Integer10")]
	pub image_height: u32,
	#[serde(rename = "Integer13")]
	pub auto_width: bool,
	#[serde(rename = "Integer12")]
	pub auto_height: bool,
	#[serde(rename = "BackgroundColor_color")]
	pub bg_color: String,
}

impl Default for BurningTextForm {
	fn default() -> Self {
		Self {
			logo_id: 4,
			text: String::new(),
			font_size: 70,
			font_color: "#FF0000".to_string(),
			burn_angle: 0,
			transparent: true,
			alignment: 0,
			image_width: 600,
			image_height: 100,
			auto_width: true,
			auto_height: true,
			bg_color: "#FFFFFF".to_string(),
		}
	}
}

impl BurningTextForm {
	pub(crate) fn new(text: String) -> Self {
		Self {
			text,
			..Self::default()
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoolTextResponse {
	pub logo_id: u64,
	pub new_id: u64,
	pub render_location: String,
	pub is_animated: bool,
}