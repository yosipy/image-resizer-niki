use image::imageops::FilterType;
use image::DynamicImage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Calc resized image size for fit is inside type
fn calc_inside_output_sizes(
    original_width: u32,
    original_height: u32,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> (u32, u32) {
    let width_ratio = target_width
        .map(|w| w as f64 / original_width as f64)
        .unwrap_or(1.0);
    let height_ratio = target_height
        .map(|h| h as f64 / original_height as f64)
        .unwrap_or(1.0);

    // Max scale is 1.0. Downsize only.
    let scale = width_ratio.min(height_ratio).min(1.0);

    let new_width = (original_width as f64 * scale).floor() as u32;
    let new_height = (original_height as f64 * scale).floor() as u32;

    (new_width, new_height)
}

// Calc resized image size for fit is cover type
// Returns (scaled_width, scaled_height, crop_x, crop_y, target_width, target_height)
fn calc_cover_output_sizes(
    original_width: u32,
    original_height: u32,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> (u32, u32, u32, u32, u32, u32) {
    // If no target size is specified, return original size with no crop
    let final_target_width = target_width.unwrap_or(original_width);
    let final_target_height = target_height.unwrap_or(original_height);

    let width_ratio = final_target_width as f64 / original_width as f64;
    let height_ratio = final_target_height as f64 / original_height as f64;

    // For cover, we use the larger ratio to ensure the target is completely filled
    // Max scale is 1.0. Downsize only.
    let scale = width_ratio.max(height_ratio).min(1.0);

    let scaled_width = (original_width as f64 * scale).round() as u32;
    let scaled_height = (original_height as f64 * scale).round() as u32;

    // Calculate crop position (center crop)
    let crop_x = if scaled_width > final_target_width {
        (scaled_width - final_target_width) / 2
    } else {
        0
    };

    let crop_y = if scaled_height > final_target_height {
        (scaled_height - final_target_height) / 2
    } else {
        0
    };

    (
        scaled_width,
        scaled_height,
        crop_x,
        crop_y,
        final_target_width,
        final_target_height,
    )
}

#[wasm_bindgen]
pub fn resize_inside(
    source_canvas: HtmlCanvasElement,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<HtmlCanvasElement, JsValue> {
    let source_ctx = source_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let source_width = source_canvas.width();
    let source_height = source_canvas.height();

    let (actual_width, actual_height) = calc_inside_output_sizes(
        source_width,
        source_height,
        target_width,
        target_height,
    );

    let image_data = source_ctx.get_image_data(0.0, 0.0, source_width as f64, source_height as f64)?;
    let data = image_data.data();

    // Resize by image crate
    let img = image::RgbaImage::from_raw(source_width, source_height, data.to_vec())
        .ok_or_else(|| JsValue::from_str("Failed to create image from canvas data"))?;

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let resized = dynamic_img.resize(actual_width, actual_height, FilterType::Lanczos3);

    // Create canvas for output
    let document = web_sys::window()
        .unwrap()
        .document()
        .ok_or_else(|| JsValue::from_str("Failed to get document"))?;

    let target_canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    target_canvas.set_width(actual_width);
    target_canvas.set_height(actual_height);

    let target_ctx = target_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Puts to output canvas
    let resized_data = resized.to_rgba8();
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&resized_data.into_raw()),
        actual_width,
        actual_height,
    )?;

    target_ctx.put_image_data(&image_data, 0.0, 0.0)?;

    log(&format!(
        "Image resized from {}x{} to {}x{}",
        source_width, source_height, actual_width, actual_height
    ));

    Ok(target_canvas)
}

#[wasm_bindgen]
pub fn resize_cover(
    source_canvas: HtmlCanvasElement,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<HtmlCanvasElement, JsValue> {
    let source_ctx = source_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let source_width = source_canvas.width();
    let source_height = source_canvas.height();

    let (scaled_width, scaled_height, crop_x, crop_y, final_width, final_height) =
        calc_cover_output_sizes(source_width, source_height, target_width, target_height);

    let image_data = source_ctx.get_image_data(0.0, 0.0, source_width as f64, source_height as f64)?;
    let data = image_data.data();

    // Resize by image crate
    let img = image::RgbaImage::from_raw(source_width, source_height, data.to_vec())
        .ok_or_else(|| JsValue::from_str("Failed to create image from canvas data"))?;

    let dynamic_img = DynamicImage::ImageRgba8(img);
    let resized = dynamic_img.resize(scaled_width, scaled_height, FilterType::Lanczos3);

    // Crop the resized image to the target dimensions
    let cropped = resized.crop_imm(crop_x, crop_y, final_width, final_height);

    // Create canvas for output
    let document = web_sys::window()
        .unwrap()
        .document()
        .ok_or_else(|| JsValue::from_str("Failed to get document"))?;

    let target_canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    target_canvas.set_width(final_width);
    target_canvas.set_height(final_height);

    let target_ctx = target_canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Puts to output canvas
    let cropped_data = cropped.to_rgba8();
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&cropped_data.into_raw()),
        final_width,
        final_height,
    )?;

    target_ctx.put_image_data(&image_data, 0.0, 0.0)?;

    log(&format!(
        "Image resized (cover) from {}x{} to {}x{} (scaled to {}x{}, cropped at {}, {})",
        source_width, source_height, final_width, final_height, scaled_width, scaled_height, crop_x, crop_y
    ));

    Ok(target_canvas)
}