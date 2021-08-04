use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage, Rgba};
use rocket::serde::json::{Error as JsonError, Json};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use super::image::ImageJson;
use crate::{errors::Errors, state::serverstate::ServerState};

#[derive(Deserialize)]
pub struct TemplateInputJson {
    pub texts: Vec<String>,
    pub images: Vec<ImageJson>,
}

#[derive(Deserialize)]
pub struct Overlay {
    coords: (u32, u32),
    resize: (u32, u32),
    input_size: u32,
}

impl Overlay {
    fn process(
        &self,
        mut image: DynamicImage,
        layer: &DynamicImage,
    ) -> Result<DynamicImage, Errors> {
        let layer_resized = layer.resize(
            self.resize.0,
            self.resize.1,
            image::imageops::FilterType::Nearest,
        );
        image::imageops::overlay(&mut image, &layer_resized, self.coords.0, self.coords.1);
        Ok(image)
    }
}

#[derive(Deserialize)]
pub struct DrawText {
    coords: (u32, u32),
    color: [u8; 4],
    scale: (f32, f32),
    max_width: usize,
    font: String,
}

impl DrawText {
    fn process(
        &self,
        mut image: DynamicImage,
        state: &ServerState,
        text: &str,
    ) -> Result<DynamicImage, Errors> {
        let text = textwrap::fill(text, self.max_width);
        crate::imagelib::drawtext::draw_text(
            &mut image,
            Rgba(self.color),
            &*(state.cache.get_font(&self.font)?),
            &text,
            rusttype::Scale {
                x: self.scale.0,
                y: self.scale.1,
            },
            &self.coords,
        );
        Ok(image)
    }
}

#[derive(Deserialize)]
pub enum Operation {
    DrawText(DrawText),
    Overlay(Overlay),
}

#[derive(Deserialize)]
pub struct Template {
    pub name: String,
    startfile: String,
    operations: Vec<Operation>,
}

impl Template {
    pub async fn process(
        &'static self,
        state: &'static ServerState,
        input: TemplateInputJson,
    ) -> Result<DynamicImage, Errors> {
        let cursor = Cursor::new(state.cache.get_image(&self.startfile).await?.to_vec());

        let overlays = self
            .operations
            .iter()
            .filter(|o| matches!(o, Operation::Overlay(..)))
            .map(|o| match o {
                Operation::Overlay(o) => o,
                _ => unreachable!(),
            });

        let mut overlay_layers = vec![];
        for (overlay, image) in overlays.zip(input.images.iter()) {
            overlay_layers.push(image.to_image(overlay.input_size, state).await?);
        }

        spawn_blocking(move || {
            let mut img = ImageReader::new(cursor).with_guessed_format()?.decode()?;

            let mut text_index = 0;
            let mut overlay_index = 0;

            for op in self.operations.iter() {
                match op {
                    Operation::DrawText(dt) => {
                        img = dt.process(img, state, &input.texts[text_index])?;
                        text_index += 1;
                    }
                    Operation::Overlay(overlay) => {
                        img = overlay.process(img, &overlay_layers[overlay_index])?;
                        overlay_index += 1;
                    }
                };
            }
            Ok(img)
        })
        .await
        .unwrap()
    }
}

pub type TemplateInput<'a> = Result<Json<TemplateInputJson>, JsonError<'a>>;
