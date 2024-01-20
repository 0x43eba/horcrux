use steganography::{util::{file_as_image_buffer, bytes_to_str}, decoder::Decoder};

pub fn decode_from_image(location: &str) -> String{
    let image_buffer = file_as_image_buffer(location.to_string());
    let decder = Decoder::new(image_buffer);
    let output_buffer = decder.decode_alpha();
    let clean_buffer: Vec<u8> = output_buffer.into_iter().filter(|b| {*b != 0xff_u8}).collect();
    bytes_to_str(clean_buffer.as_slice()).to_string()
}