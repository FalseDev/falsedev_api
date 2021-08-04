use rocket::{
    http::Status,
    serde::json::{json, Error as JsonError, Value as JsonValue},
};

#[derive(Debug)]
pub enum Errors {
    JsonIo(std::io::Error),
    JsonParse(serde_json::error::Error),
    InvalidInput(String),
    InvalidImageName,
}

impl Errors {
    fn json(&self) -> JsonValue {
        match self {
            Self::JsonIo(error) => json!({"kind": "json_io", "message": error.to_string()}),
            Self::JsonParse(error) => json!({"kind": "json_parse", "message": error.to_string()}),
            Self::InvalidInput(error) => json!({"kind": "invalid_input", "message": error}),
            Self::InvalidImageName => {
                json!({"kind": "template_not_found", "message": "The requested image template is not found"})
            }
        }
    }

    fn status(&self) -> Status {
        match self {
            Self::JsonIo(..) => Status::BadRequest,
            Self::JsonParse(..) | Self::InvalidInput(..) => Status::UnprocessableEntity,
            Self::InvalidImageName => Status::NotFound,
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Errors {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .status(self.status())
            .join(json!({"error": self.json()}).respond_to(req).unwrap())
            .ok()
    }
}

impl From<JsonError<'_>> for Errors {
    fn from(error: JsonError) -> Self {
        match error {
            JsonError::Io(error) => Self::JsonIo(error),
            JsonError::Parse(_, error) => Self::JsonParse(error),
        }
    }
}

impl From<reqwest::Error> for Errors {
    fn from(error: reqwest::Error) -> Self {
        Self::InvalidInput(error.to_string())
    }
}

impl From<image::ImageError> for Errors {
    fn from(error: image::ImageError) -> Self {
        Self::InvalidInput(error.to_string())
    }
}

impl From<std::io::Error> for Errors {
    fn from(error: std::io::Error) -> Self {
        Self::InvalidInput(error.to_string())
    }
}

#[cfg(feature = "redis_ratelimit")]
impl From<redis::RedisError> for Errors {
    fn from(error: redis::RedisError) -> Self {
        Self::InvalidInput(error.to_string())
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::JsonIo(error) => error.fmt(fmt),
            Self::JsonParse(error) => error.fmt(fmt),
            Self::InvalidInput(error) => write!(fmt, "{}", error),
            Self::InvalidImageName => write!(fmt, "Invalid image name was provided"),
        }
    }
}
impl std::error::Error for Errors {}