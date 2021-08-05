pub mod color;
pub mod manipulation;
pub mod templates;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        manipulation::flipv,
        manipulation::fliph,
        manipulation::rotate90,
        manipulation::rotate180,
        manipulation::rotate270,
        manipulation::grayscale,
        manipulation::invert,
        manipulation::blur,
        color::color,
        color::blend,
        templates::template,
    ]
}
