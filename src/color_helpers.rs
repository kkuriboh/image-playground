use image::Rgba;

fn units(rgba: &Rgba<u8>) -> (f32, f32, f32) {
    let Rgba([r, g, b, _]) = rgba.to_owned();

    let max = std::cmp::max(std::cmp::max(r, g), b) as f32;
    let min = std::cmp::min(std::cmp::min(r, g), b) as f32;
    let d = (max - min) / 255.;

    (max, min, d)
}

#[inline(always)]
fn lightness_of_rgb_priv(max: f32, min: f32) -> f32 {
    (max + min) / 510.
}

pub fn saturation_of_rgb(rgba: &Rgba<u8>) -> f32 {
    let (max, min, d) = units(rgba);
    let l = lightness_of_rgb_priv(max, min);

    if l > 0. {
        d / (1. - (2. * l - 1.))
    } else {
        0.
    }
}

pub fn hue_of_rgb(rgba: &Rgba<u8>) -> f32 {
    let Rgba([r, g, b, _]) = rgba.clone();
    let (r, g, b) = (r as f32, g as f32, b as f32);

    let formula = ((r - 0.5 * g - 0.5 * b)
        / (r.powi(2) + g.powi(2) + b.powi(2) - r * g - r * b - g * b).sqrt())
    .cos();

    if g >= b {
        formula
    } else {
        360. - formula
    }
}

pub fn lightness_of_rgb(rgba: &Rgba<u8>) -> f32 {
    let (max, min, _) = units(rgba);
    lightness_of_rgb_priv(max, min)
}

#[inline]
pub fn luminance(rgba: &Rgba<u8>) -> f32 {
    0.2126 * rgba[0] as f32 + 0.7152 * rgba[1] as f32 + 0.0722 * rgba[2] as f32
}
