mod utils;

use flate2::read::GzDecoder;
use image::GenericImageView;
use std::convert::TryInto;
use std::io::Read;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, stealth-watermark-reader!");
}

#[wasm_bindgen]
pub struct DataReader {
    data: Vec<u8>,
    index: usize,
}

#[wasm_bindgen]
impl DataReader {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> DataReader {
        DataReader { data, index: 0 }
    }

    pub fn read_bit(&mut self) -> u8 {
        let bit = self.data[self.index] & 1; // 只读取最低位
        self.index += 1;
        bit
    }

    pub fn read_byte(&mut self) -> u8 {
        let mut byte = 0;
        for i in 0..8 {
            byte |= self.read_bit() << (7 - i);
        }
        byte
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.read_byte()).collect()
    }

    pub fn read_int32(&mut self) -> i32 {
        let bytes = self.read_bytes(4);
        let bytes4: [u8; 4] = bytes.try_into().unwrap();
        i32::from_be_bytes(bytes4)
    }
}

#[wasm_bindgen]
pub fn decode_image_data(input_bytes: Vec<u8>) -> Result<String, JsValue> {
    let img =
        image::load_from_memory(&input_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let (width, height) = img.dimensions();

    let mut lowest_data = vec![];

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let a = pixel[3]; // 获取 alpha 值
            lowest_data.push(a & 1);
        }
    }

    let mut reader = DataReader::new(lowest_data);
    let magic = "stealth_pngcomp";
    let magic_string = String::from_utf8(reader.read_bytes(magic.len())).unwrap();

    if magic == magic_string {
        let data_length = reader.read_int32() as usize;
        let gzip_bytes = reader.read_bytes(data_length / 8);
        let mut gz = GzDecoder::new(gzip_bytes.as_slice());
        let mut decompressed_data = String::new();
        gz.read_to_string(&mut decompressed_data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(decompressed_data)
    } else {
        Err(JsValue::from("Magic number not found"))
    }
}
