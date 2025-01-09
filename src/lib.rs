use bincode::Options;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Character([u8; 5]);

#[derive(Serialize, Deserialize, Debug)]
pub struct Font {
    pub characters: [Character; 16],
}

impl Font {
    pub fn from_png(data: &[u8]) -> Result<Self, &'static str> {
        // Decode header data
        let header = minipng::decode_png_header(&data).map_err(|_| "Failed to load PNG")?;

        // Check if dimensions are correct
        if header.width() != 16 || header.height() != 20 {
            return Err("PNG must be exactly 16x20");
        }

        // Allocate buffer of size
        let mut buffer = vec![0; header.required_bytes_rgba8bpc()];

        // Load png into buffer
        let mut image =
            minipng::decode_png(&data, &mut buffer).map_err(|_| "Failed to load PNG")?;
        image
            .convert_to_rgba8bpc()
            .map_err(|_| "Failed to convert to RGBA")?;

        let pixels = image.pixels();
        let mut characters = [Character([0; 5]); 16];

        // Process each character
        for char_index in 0..16 {
            let char_x = (char_index % 4) * 4;
            let char_y = (char_index / 4) * 5;
            let mut char_data = [0u8; 5];

            // Iterate over each row of the character
            for row in 0..5 {
                let mut byte = 0u8;
                // Iterate over each column of the character
                for col in 0..4 {
                    let pixel_x = char_x + col;
                    let pixel_y = char_y + row;
                    let pixel_index = (pixel_y * 16 + pixel_x) * 4;

                    // Check if pixel is set (assuming black is character pixel)
                    if pixels[pixel_index] < 128 {
                        byte |= 1 << (3 - col);
                    }
                }
                char_data[row] = byte;
            }

            characters[char_index] = Character(char_data);
        }

        Ok(Font { characters })
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let config = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_little_endian();

        config.serialize(self).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        let config = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_little_endian();

        Ok(config
            .deserialize(data)
            .expect("Unable to deserialize font from bytes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_from_png() {
        let font = Font::from_png(include_bytes!("font.png")).expect("Failed to load font");
        assert_eq!(font.characters.len(), 16);
    }

    #[test]
    fn test_invalid_png_dimensions() {
        // Invalid PNG data
        let invalid_data = vec![0u8; 100];
        let result = Font::from_png(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut font = Font::from_png(include_bytes!("font.png")).expect("Failed to load font");
        let bytes = font.to_bytes();
        let deserialized_font = Font::from_bytes(&bytes).expect("Failed to deserialize");
        assert_eq!(font.characters.len(), deserialized_font.characters.len());
    }

    #[test]
    fn test_character_data() {
        let font = Font::from_png(include_bytes!("font.png")).expect("Failed to load font");
        for character in &font.characters {
            for row in &character.0 {
                assert!(*row <= 0x0F);
            }
        }
    }

    #[test]
    fn test_font_display() {
        let mut font = Font::from_png(include_bytes!("font.png")).expect("Failed to load font");
        let data = font.to_bytes();
        for char in data.chunks(5) {
            for line in char {
                for i in (0..4).rev() {
                    let bit = (line >> i) & 1;
                    assert!(bit <= 1);
                }
            }
        }
    }
}
