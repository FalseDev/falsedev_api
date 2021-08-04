use std::io::Cursor;

use image::DynamicImage;
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
    Request,
};

use crate::{errors::Errors, state::serverstate::ServerState};

pub struct ImageResponse(pub DynamicImage);

impl ImageResponse {
    #[inline]
    pub fn ok<T>(self) -> Result<Self, T> {
        Ok(self)
    }
}

impl<'r> Responder<'r, 'static> for ImageResponse {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let _state = request.rocket().state::<&ServerState>().unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        if let Err(err) = self.0.write_to(&mut bytes, image::ImageOutputFormat::Png) {
            return Errors::from(err).respond_to(request);
        };
        Response::build()
            .header(ContentType::PNG)
            .sized_body(bytes.len(), Cursor::new(bytes))
            .ok()
    }
}
