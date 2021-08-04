use std::io::Cursor;

use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView};
use rocket::serde::json::{Error as JsonError, Json};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use crate::{errors::Errors, imagelib::fillcolor::fill_color, state::serverstate::ServerState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageJson {
    DiscordProfile {
        id: u64,
        hash: String,
    },
    GithubProfile {
        username: String,
    },
    GithubAsset {
        owner: String,
        repo: String,
        path: String,
    },
    Imgur {
        id: String,
        subdomain: String,
    },
    Color(u8, u8, u8),
    Base64(String),
    #[cfg(feature = "file_input")]
    File(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum GithubContentsResponse {
    GithubContents {
        size: u32,
        #[serde(rename = "type")]
        type_: String,
        download_url: String,
    },
    GithubError {
        message: String,
        errors: Vec<GithubError>,
    },
}

#[derive(Deserialize)]
struct GithubError {
    code: String,
}

impl ImageJson {
    fn url(&self, size: u32) -> Result<String, Errors> {
        match self {
            Self::DiscordProfile { id, hash } => Ok(format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}?size={}",
                id,
                hash,
                "png",
                if size == 0 { 1024 } else { size }
            )),
            Self::GithubProfile { username } => Ok(format!(
                "https://github.com/{}.png{}",
                username,
                if size == 0 {
                    "".into()
                } else {
                    format!("?size={}", size)
                }
            )),
            Self::Imgur { id, subdomain } => {
                let size = if size != 0 {
                    [(90, "s"), (160, "b"), (320, "m"), (640, "l"), (1024, "h")]
                        .iter()
                        .find(|(s, _)| *s >= size)
                        .unwrap()
                        .1
                } else {
                    ""
                };
                let subdomain = if subdomain.is_empty() {
                    String::new()
                } else {
                    subdomain.to_owned() + "."
                };
                Ok(format!("https://{}imgur.com/{}{}.png", subdomain, id, size))
            }
            Self::Base64(..) | Self::GithubAsset { .. } | Self::Color(..) => unreachable!(),
            #[cfg(feature = "file_input")]
            Self::File(..) => unreachable!(),
        }
    }
    async fn to_vec(&self, size: u32, state: &ServerState) -> Result<Vec<u8>, Errors> {
        match self {
            Self::Base64(text) => base64::decode(text)
                .map_err(|_| Errors::InvalidInput("Invalid base64 string provided".into())),
            Self::GithubAsset { owner, repo, path } => {
                self.get_github_asset(owner, repo, path, state.client())
                    .await
            }
            Self::Imgur { .. } | Self::GithubProfile { .. } | Self::DiscordProfile { .. } => {
                Ok(state
                    .client()
                    .get(self.url(size)?)
                    .send()
                    .await?
                    .bytes()
                    .await?
                    .to_vec())
            }
            #[cfg(feature = "file_input")]
            Self::File(filename) => {
                if !state.allow_local_file_input {
                    return Err(Errors::InvalidInput("Local file input is disabled.".into()));
                }
                state.cache.get_image(filename).await.unwrap()
            }
            Self::Color(..) => unreachable!(),
        }
    }

    pub async fn to_image(&self, size: u32, state: &ServerState) -> Result<DynamicImage, Errors> {
        let mut image = match self {
            Self::Color(r, g, b) => {
                let size = if size == 0 { 1024 } else { size };
                let (r, g, b) = (*r, *g, *b);
                spawn_blocking(move || DynamicImage::ImageRgb8(fill_color([r, g, b], (size, size))))
                    .await
                    .unwrap()
            }

            _ => {
                let bytes = self.to_vec(size, state).await?;

                spawn_blocking(move || {
                    let reader = Reader::new(Cursor::new(&bytes));
                    reader.with_guessed_format()?.decode()
                })
                .await
                .unwrap()?
            }
        };

        // Resize if required
        if size != 0 && (image.width() != size || image.height() != size) {
            image = spawn_blocking(move || image.resize(size, size, FilterType::Nearest))
                .await
                .unwrap();
        }

        Ok(image)
    }

    async fn get_github_asset(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        client: &reqwest::Client,
    ) -> Result<Vec<u8>, Errors> {
        match client
            .get(format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                owner, repo, path
            ))
            .send()
            .await?
            .json::<GithubContentsResponse>()
            .await?
        {
            GithubContentsResponse::GithubError { message, errors } => {
                if errors.iter().any(|e| e.code == "too_large") {
                    return Err(Errors::InvalidInput("File too large".into()));
                }
                return Err(Errors::InvalidInput(format!(
                    "Message from github: {}",
                    message
                )));
            }
            GithubContentsResponse::GithubContents {
                size,
                type_,
                download_url,
            } => {
                if type_ != "file" {
                    return Err(Errors::InvalidInput(format!(
                        "Given path is not a file, it is {}",
                        type_
                    )));
                }
                if size > 500000 {
                    return Err(Errors::InvalidInput(format!(
                        "File size too large, must be below {}",
                        500000
                    )));
                }
                Ok(client
                    .get(download_url)
                    .send()
                    .await?
                    .bytes()
                    .await?
                    .to_vec())
            }
        }
    }
}

pub type Image<'a> = Result<Json<ImageJson>, JsonError<'a>>;
