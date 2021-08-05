use image::DynamicImage;
use rocket::State;
use tokio::task::spawn_blocking;

use crate::{
    datastructures::image::Image,
    errors::Errors,
    imagelib::{fillcolor::fill_color, image_response::ImageResponse},
    state::serverstate::ServerState,
};

#[get("/color?<r>&<g>&<b>")]
pub async fn color(
    r: u8,
    g: u8,
    b: u8,
    state: &State<&'static ServerState>,
) -> Result<ImageResponse, Errors> {
    let size = state.config.colorfill_image_size;
    let color = [r, g, b];
    let img = spawn_blocking(move || fill_color(color, (size, size)))
        .await
        .unwrap();
    ImageResponse(DynamicImage::ImageRgb8(img)).ok()
}

#[post("/colorblend?<r>&<g>&<b>", data = "<image>")]
pub async fn blend(
    r: u8,
    g: u8,
    b: u8,
    image: Image<'_>,
    state: &State<&'static ServerState>,
) -> Result<ImageResponse, Errors> {
    let size = 256;
    let color = [r, g, b];

    let image = image?.to_image(size, state).await?;
    let image = spawn_blocking(move || {
        let mut image = image.to_rgb8();
        for x in 0..size {
            for y in 0..size {
                let pixel = image.get_pixel_mut(x, y);
                for i in 0..3 {
                    pixel[i] = pixel[i] / 2 + color[i] / 2;
                }
            }
        }
        image
    })
    .await
    .unwrap();
    ImageResponse(DynamicImage::ImageRgb8(image)).ok()
}
