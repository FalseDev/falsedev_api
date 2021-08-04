use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rusttype::Font;

use crate::errors::Errors;

pub struct AssetCache {
    image_cache: RwLock<HashMap<String, Arc<Vec<u8>>>>,
    font_cache: RwLock<HashMap<String, Arc<Font<'static>>>>,
}

impl AssetCache {
    #[inline]
    pub fn new() -> Self {
        Self {
            image_cache: RwLock::new(HashMap::new()),
            font_cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_image(&self, name: &str) -> Result<Arc<Vec<u8>>, Errors> {
        if let Some(arc) = self.image_cache.read().unwrap().get(name) {
            return Ok(arc.clone());
        }
        let bytes = tokio::fs::read(name).await?.to_vec();
        let arc = Arc::new(bytes);
        self.image_cache
            .write()
            .unwrap()
            .insert(name.to_string(), Arc::clone(&arc));
        Ok(arc)
    }

    pub fn get_font(&self, name: &str) -> Result<Arc<Font<'static>>, Errors> {
        if let Some(font) = self.font_cache.read().unwrap().get(name) {
            return Ok(font.clone());
        }
        let font = Font::try_from_vec(std::fs::read(name)?.to_vec()).expect("Invalid font");
        let arc = Arc::new(font);
        self.font_cache
            .write()
            .unwrap()
            .insert(name.to_string(), Arc::clone(&arc));
        Ok(arc)
    }
}

impl Default for AssetCache {
    fn default() -> Self {
        Self::new()
    }
}
