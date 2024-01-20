use steganography::{util::{file_as_dynamic_image, save_image_buffer}, encoder::Encoder};

pub fn encode_to_image(location: &str, message: &str, output: &str) {
    let message_bytes = message.as_bytes();
    let image_buffer = file_as_dynamic_image(location.to_string());
    let encoder = Encoder::new(message_bytes, image_buffer);
    let result = encoder.encode_alpha();
    save_image_buffer(result, output.to_string());
}