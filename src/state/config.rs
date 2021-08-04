use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

use crate::{datastructures::template::Template, errors::Errors};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub templates: Vec<Template>,

    pub default_font: String,
    pub textdraw_text_max_len: usize,
    pub blur_sigma: f32,
    pub colorfill_image_size: u32,
    pub allow_local_file_input: bool,
}

impl ServerConfig {
    pub fn new(config_filename: &str) -> Self {
        Figment::from(Toml::file(config_filename))
            .extract()
            .map_err(|errors| {
                for e in errors {
                    eprintln!("{}", e.to_string());
                }
                std::process::exit(1);
            })
            .unwrap()
    }

    pub fn get_template(&self, name: String) -> Result<&Template, Errors> {
        for template in self.templates.iter() {
            if template.name == name {
                return Ok(template);
            }
        }
        Err(Errors::InvalidImageName)
    }
}
