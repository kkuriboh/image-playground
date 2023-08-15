use crate::{
    cli::{SortDirection, SortKind, SortMethod, Threshold},
    color_helpers,
};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub fn sort(
    img: DynamicImage,
    luminance_threshold: Threshold,
    kind: SortKind,
    direction: SortDirection,
    method: SortMethod,
) -> DynamicImage {
    let (width, height) = img.dimensions();
    let method = calculate_method(method);

    let sort_algo = match direction {
        SortDirection::Horizontal => sort_horizontal,
        SortDirection::Vertical => sort_vertical,
    };

    let mut ret = DynamicImage::new_rgba8(width, height);
    sort_algo(
        img,
        ret.as_mut_rgba8().unwrap(),
        luminance_threshold,
        kind,
        &method,
    );

    ret
}

fn calc_luminance(
    rgba: &Rgba<u8>,
    pxs: &mut Vec<(f32, Rgba<u8>)>,
    positions: &mut Vec<(usize, usize)>,
    ret_img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    luminance_threshold: Threshold,
    method: &dyn Fn(&Rgba<u8>) -> f32,
    pos: (usize, usize),
) {
    let sortable_data = method(rgba);
    let luminance = color_helpers::luminance(rgba);

    if luminance >= luminance_threshold.min && luminance <= luminance_threshold.max {
        positions.push(pos);
        pxs.push((sortable_data, rgba.to_owned()));
        return;
    }

    ret_img_buffer.put_pixel(pos.0 as _, pos.1 as _, rgba.to_owned());
}

fn sort_and_put_remaining_pixels(
    ret_img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    mut pxs: Vec<(f32, Rgba<u8>)>,
    positions: Vec<(usize, usize)>,
    kind: SortKind,
) {
    pxs.sort_by(|left, right| match kind {
        SortKind::LeftToRight => left.0.total_cmp(&right.0),
        SortKind::RightToLeft => right.0.total_cmp(&left.0),
    });

    for idx in 0..pxs.len() {
        let (col, row) = positions[idx];
        let (_, rgba) = pxs[idx];

        ret_img_buffer.put_pixel(col as _, row as _, rgba);
    }
}

fn sort_vertical(
    img: DynamicImage,
    ret_img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    luminance_threshold: Threshold,
    kind: SortKind,
    method: &dyn Fn(&Rgba<u8>) -> f32,
) {
    let (width, height) = img.dimensions();

    let img = img.to_rgba8();
    let rows = img
        .rows()
        .map(image::buffer::Pixels::collect)
        .collect::<Vec<Vec<_>>>();

    for col_idx in 0..width as usize - 1 {
        let mut pxs = vec![];
        let mut positions = vec![];

        for row_idx in 0..height as usize - 1 {
            let rgba = rows[row_idx][col_idx];

            calc_luminance(
                rgba,
                &mut pxs,
                &mut positions,
                ret_img_buffer,
                luminance_threshold.clone(),
                &method,
                (col_idx, row_idx),
            );
        }

        sort_and_put_remaining_pixels(ret_img_buffer, pxs, positions, kind.clone());
    }
}

fn sort_horizontal(
    img: DynamicImage,
    ret_img_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    luminance_threshold: Threshold,
    kind: SortKind,
    method: &dyn Fn(&Rgba<u8>) -> f32,
) {
    for (row, pixels) in img.to_rgba8().rows().enumerate() {
        let mut pxs = vec![];
        let mut positions = vec![];

        for (col, rgba) in pixels.enumerate() {
            calc_luminance(
                rgba,
                &mut pxs,
                &mut positions,
                ret_img_buffer,
                luminance_threshold.clone(),
                &method,
                (col, row),
            );
        }

        sort_and_put_remaining_pixels(ret_img_buffer, pxs, positions, kind.clone());
    }
}

fn calculate_method(method: SortMethod) -> impl Fn(&Rgba<u8>) -> f32 {
    match method {
        SortMethod::Default => |rgba: &Rgba<u8>| rgba[0] as f32 + rgba[1] as f32 + rgba[2] as f32,
        SortMethod::Red => |rgba: &Rgba<u8>| rgba[0] as f32,
        SortMethod::Green => |rgba: &Rgba<u8>| rgba[1] as f32,
        SortMethod::Blue => |rgba: &Rgba<u8>| rgba[2] as f32,
        SortMethod::Alpha => |rgba: &Rgba<u8>| rgba[3] as f32,
        SortMethod::Hue => color_helpers::hue_of_rgb,
        SortMethod::Saturation => color_helpers::saturation_of_rgb,
        SortMethod::Lightness => color_helpers::lightness_of_rgb,
    }
}
