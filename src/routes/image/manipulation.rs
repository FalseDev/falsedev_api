use rocket::State;

use crate::{datastructures::image::Image, errors::Errors, state::serverstate::ServerState};

macro_rules! image_route {
    ($path: literal, $name: ident, $method: ident, mutable $(,$args: expr)*) => {
        #[post($path, data = "<image>")]
        pub async fn $name(
            image: Image<'_>,
            server_state: &State<&ServerState>,
        ) -> Result<Vec<u8>, Errors> {
            let mut bytes: Vec<u8> = Vec::new();
            let mut image = image?.to_image(256, &server_state).await?;
            image.$method($(server_state.config.$args,)*);
            image.write_to(&mut bytes, image::ImageOutputFormat::Png)?;
            Ok(bytes)
        }
    };

    ($path: literal, $name: ident, $method: ident $(,$args: ident)*) => {
        #[post($path, data = "<image>")]
        pub async fn $name(
            image: Image<'_>,
            server_state: &State<&ServerState>,
        ) -> Result<Vec<u8>, Errors> {
            let mut bytes: Vec<u8> = Vec::new();
            image?
                .to_image(256, &server_state)
                .await?
                .$method($(server_state.config.$args,)*)
                .write_to(&mut bytes, image::ImageOutputFormat::Png)?;
            Ok(bytes)
        }
    };
}

image_route!("/flipv", flipv, flipv);
image_route!("/fliph", fliph, fliph);
image_route!("/rotate90", rotate90, rotate90);
image_route!("/rotate180", rotate180, rotate180);
image_route!("/rotate270", rotate270, rotate270);
image_route!("/grayscale", grayscale, grayscale);
image_route!("/invert", invert, invert, mutable);
image_route!("/blur", blur, blur, blur_sigma);

pub fn image_manipulation_routes() -> Vec<rocket::Route> {
    return routes![flipv, fliph, rotate90, rotate180, rotate270, grayscale, invert, blur];
}
