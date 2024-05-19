use js_sys::Error;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::FileReader;

#[wasm_bindgen]
pub struct PdfProcessor;

#[derive(Serialize, Deserialize)]
struct PdfMetadata {
    title: String,
    num_pages: u32,
    num_bytes: u32,
    images: Vec<String>, // Base64 encoded images
}

fn process_pdf_impl(pdf_data: Uint8Array) -> Result<PdfMetadata, Error> {
    // Mock processing
    let metadata = PdfMetadata {
        title: "Mock Title".to_string(),
        num_pages: 5,
        num_bytes: pdf_data.length(),
        images: vec![
            "base64mockimage1".to_string(),
            "base64mockimage2".to_string(),
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
