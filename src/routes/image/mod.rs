mod color;
mod manipulation;
mod merge;
mod templates;

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
        merge::merge,
        templates::template,
    ]
}
