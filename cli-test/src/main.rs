use flate2::read::GzDecoder;
use image::GenericImageView;
use std::convert::TryInto;
use std::fs;
use std::io::{self, Read};

pub struct DataReader {
    data: Vec<u8>,
    index: usize,
}

impl DataReader {
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

pub fn decode_image_data(input_bytes: Vec<u8>) -> Result<String, String> {
    let img = image::load_from_memory(&input_bytes).map_err(|e| e.to_string())?;
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
            .map_err(|e| e.to_string())?;
        Ok(decompressed_data)
    } else {
        Err("Magic number not found".to_string())
    }
}

fn main() {
    let image_path = "E:\\FFOutput\\08-04-02.webp"; // 请确保这里是你的图片路径
    let image_bytes = fs::read(image_path).expect("Failed to read image file");

    match decode_image_data(image_bytes) {
        Ok(data) => println!("Decoded data: {}", data),
        Err(e) => println!("Failed to decode image data: {}", e),
    }
}
