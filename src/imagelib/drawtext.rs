use conv::ValueInto;
use image::Pixel;
use imageproc::{definitions::Clamp, drawing::draw_text_mut};
use rusttype::{point, Font, Scale};

fn get_font_height(font: &Font, scale: Scale) -> f32 {
    let v_metrics = font.v_metrics(scale);
    v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
}

pub fn draw_text<'a, C>(
    image: &'a mut C,
    color: C::Pixel,
    font: &Font,
    fulltext: &str,
    scale: Scale,
    mid: &(u32, u32),
) where
    C: imageproc::drawing::Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let (raw_x, raw_y) = mid;
    let text_height = get_font_height(font, scale);
    let line_count = fulltext.lines().count() as u32;

    for (index, text) in fulltext.lines().enumerate() {
        if text.is_empty() {
            continue;
        }

        let text_width = measure_line_width(font, text, scale);
        let x = *raw_x - (text_width as u32) / 2;
        let y_delta = ((index as f32 - (line_count - 1) as f32 / 2f32) * text_height) as i32;
        let y = (*raw_y as i32 + y_delta) as u32;

        draw_text_mut(image, color, x, y, scale, font, text);
    }
}

pub fn measure_line_width(font: &Font, text: &str, scale: Scale) -> f32 {
    let width = font
        .layout(text, scale, point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    width
}
