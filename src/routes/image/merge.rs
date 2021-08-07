use image::GenericImageView;
use rocket::{
    serde::json::{Error as JsonError, Json},
    State,
};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use crate::{
    datastructures::image::ImageJson, errors::Errors, imagelib::image_response::ImageResponse,
    state::serverstate::ServerState,
};

#[derive(Deserialize)]
pub struct TwoImagesJson([ImageJson; 2]);
type TwoImages<'a> = Result<Json<TwoImagesJson>, JsonError<'a>>;

#[post("/merge", data = "<two_images>")]
pub async fn merge(
    two_images: TwoImages<'_>,
    server_state: &State<&'static ServerState>,
) -> Result<ImageResponse, Errors> {
    let two_images = two_images?;
    let [base, layer] = two_images.into_inner().0;
    let (mut base, mut layer) = (
        base.to_image(256, server_state).await?,
        layer.to_image(256, server_state).await?,
    );

    let image = spawn_blocking(move || {
        let (b_x, b_y) = base.dimensions();
        let (l_x, l_y) = layer.dimensions();
        let x = *[b_x, l_x].iter().min().unwrap();
        let y = *[b_y, l_y].iter().min().unwrap();

        if x != b_x || y != b_x {
            base = base.crop(0, 0, x, y);
        }

        if x != b_y || y != b_y {
            layer = layer.crop(0, 0, x, y);
        }

        imageproc::map::map_colors2(&base, &layer, |a, b| {
            imageproc::pixelops::weighted_sum(a, b, 0.5, 0.5)
        })
    })
    .await?;
    ImageResponse(image::DynamicImage::ImageRgba8(image)).ok()
}
