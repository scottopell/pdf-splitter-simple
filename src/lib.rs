use imageproc::drawing::draw_hollow_circle_mut;
use imageproc::drawing::draw_line_segment_mut;
use imageproc::rect::Rect;
use js_sys::Error;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::FileReader;

use base64::prelude::*;
use std::io::Cursor;

use image::{ImageBuffer, ImageFormat, Rgba};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_rect_mut};

#[wasm_bindgen]
pub fn create_smiley_face_png(mouth_char: char) -> Base64Png {
    let mouth_char = mouth_char.to_ascii_uppercase();
    let width = 100;
    let height = 100;
    let mut img = ImageBuffer::new(width, height);

    // Draw a simple smiley face
    // (You would replace this logic with whatever designs you need)
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx = x as i32 - 50; // center x
        let cy = y as i32 - 50; // center y
        if cx.pow(2) + cy.pow(2) <= 40_i32.pow(2) {
            *pixel = Rgba([255, 255, 0, 255]); // Yellow circle
        } else {
            *pixel = Rgba([0, 0, 0, 255]); // Black background
        }
    }

    // Drawing eyes and mouth would go here
    draw_filled_circle_mut(&mut img, (35, 35), 5, Rgba([0, 0, 0, 255])); // Left eye
    draw_filled_circle_mut(&mut img, (65, 35), 5, Rgba([0, 0, 0, 255])); // Right eye
    match mouth_char {
        'A' => draw_hollow_rect_mut(
            &mut img,
            Rect::at(40, 60).of_size(20, 10),
            Rgba([0, 0, 0, 255]),
        ),
        'B' => draw_filled_rect_mut(
            &mut img,
            Rect::at(45, 60).of_size(10, 10),
            Rgba([0, 0, 0, 255]),
        ),
        'C' => draw_hollow_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255])),
        'D' => draw_filled_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255])),
        'E' => draw_line_segment_mut(&mut img, (40.0, 65.0), (60.0, 65.0), Rgba([0, 0, 0, 255])),
        'F' => draw_line_segment_mut(&mut img, (40.0, 65.0), (60.0, 65.0), Rgba([0, 0, 0, 255])),
        'G' => {
            draw_hollow_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (60.0, 65.0), Rgba([0, 0, 0, 255]));
        }
        'H' => {
            draw_filled_rect_mut(
                &mut img,
                Rect::at(45, 60).of_size(10, 10),
                Rgba([0, 0, 0, 255]),
            );
            draw_filled_rect_mut(
                &mut img,
                Rect::at(47, 62).of_size(6, 6),
                Rgba([255, 255, 0, 255]),
            ); // Create a "hole" in the middle
        }
        'I' => draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255])),
        'J' => {
            draw_line_segment_mut(&mut img, (45.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 70.0), (55.0, 60.0), Rgba([0, 0, 0, 255]));
        }
        'K' => {
            draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (45.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'L' => {
            draw_line_segment_mut(&mut img, (45.0, 60.0), (45.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (45.0, 70.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'M' => {
            draw_filled_circle_mut(&mut img, (45, 60), 5, Rgba([0, 0, 0, 255]));
            draw_filled_circle_mut(&mut img, (55, 60), 5, Rgba([0, 0, 0, 255]));
        }
        'N' => {
            draw_filled_circle_mut(&mut img, (45, 60), 5, Rgba([0, 0, 0, 255]));
            draw_filled_circle_mut(&mut img, (55, 70), 5, Rgba([0, 0, 0, 255]));
        }
        'O' => draw_hollow_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255])),
        'P' => {
            draw_hollow_circle_mut(&mut img, (50, 60), 5, Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'Q' => {
            draw_hollow_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'R' => {
            draw_hollow_circle_mut(&mut img, (50, 60), 5, Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'S' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (60.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 70.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'T' => {
            draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (45.0, 60.0), (55.0, 60.0), Rgba([0, 0, 0, 255]));
        }
        'U' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 70.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'V' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 70.0), (60.0, 60.0), Rgba([0, 0, 0, 255]));
        }
        'W' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (45.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (45.0, 70.0), (50.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 60.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (55.0, 70.0), (60.0, 60.0), Rgba([0, 0, 0, 255]));
        }
        'X' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'Y' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (50.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (50.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        'Z' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (60.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 70.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '0' => draw_hollow_circle_mut(&mut img, (50, 65), 10, Rgba([0, 0, 0, 255])),
        '1' => draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255])),
        '2' => {
            draw_line_segment_mut(&mut img, (40.0, 60.0), (60.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 60.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 70.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '3' => {
            draw_hollow_circle_mut(&mut img, (50, 60), 5, Rgba([0, 0, 0, 255]));
            draw_hollow_circle_mut(&mut img, (50, 70), 5, Rgba([0, 0, 0, 255]));
        }
        '4' => {
            draw_line_segment_mut(&mut img, (45.0, 60.0), (45.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (45.0, 65.0), (55.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (55.0, 60.0), (55.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '5' => {
            draw_line_segment_mut(&mut img, (60.0, 60.0), (40.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 60.0), (40.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (40.0, 65.0), (60.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 65.0), (60.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (60.0, 70.0), (40.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '6' => {
            draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (55.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 65.0), (45.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '7' => {
            draw_line_segment_mut(&mut img, (45.0, 60.0), (55.0, 60.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (55.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        '8' => {
            draw_hollow_circle_mut(&mut img, (50, 60), 5, Rgba([0, 0, 0, 255]));
            draw_hollow_circle_mut(&mut img, (50, 70), 5, Rgba([0, 0, 0, 255]));
        }
        '9' => {
            draw_line_segment_mut(&mut img, (50.0, 60.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (50.0, 60.0), (55.0, 65.0), Rgba([0, 0, 0, 255]));
            draw_line_segment_mut(&mut img, (55.0, 65.0), (50.0, 70.0), Rgba([0, 0, 0, 255]));
        }
        _ => {}
    }

    // Convert to PNG and then to a Base64 string
    let mut png_data = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(img)
        .write_to(&mut png_data, ImageFormat::Png)
        .unwrap();
    BASE64_STANDARD.encode(&png_data.into_inner())
}

#[wasm_bindgen]
pub struct PdfProcessor;

type Base64Png = String;

#[derive(Serialize, Deserialize)]
struct PdfMetadata {
    title: String,
    num_pages: u32,
    num_bytes: u32,
    images: Vec<Base64Png>,
}

fn process_pdf_impl(pdf_data: Uint8Array) -> Result<PdfMetadata, Error> {
    // Mock processing
    let metadata = PdfMetadata {
        title: "Mock Title".to_string(),
        num_pages: 5,
        num_bytes: pdf_data.length(),
        images: vec![
            create_smiley_face_png('1'),
            create_smiley_face_png('2'),
            create_smiley_face_png('3'),
            create_smiley_face_png('4'),
            create_smiley_face_png('5'),
        ],
    };

    Ok(metadata)
}

#[wasm_bindgen]
impl PdfProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PdfProcessor {
        PdfProcessor {}
    }

    pub fn process_pdf(&self, file: web_sys::File) -> js_sys::Promise {
        let fr = FileReader::new().unwrap();

        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let fr_clone = fr.clone();
            let onloadend = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                let array_buffer = fr_clone.result().unwrap();
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let metadata = process_pdf_impl(uint8_array);
                match metadata {
                    Ok(metadata) => {
                        let metadata_json = serde_json::to_string(&metadata).unwrap();
                        let metadata_js = JsValue::from_str(&metadata_json);

                        resolve.call1(&JsValue::NULL, &metadata_js).unwrap();
                    }
                    Err(e) => {
                        reject.call1(&JsValue::NULL, &e.clone()).unwrap();
                    }
                }
            }) as Box<dyn FnMut(_)>);

            fr.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&file).unwrap();
            onloadend.forget();
        });

        promise
    }
}
