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
