use bevy::asset::AssetPath;

#[derive(Default, Copy, Clone)]
pub enum FontSize {
    Large,
    #[default]
    Medium,
    Small,
    Px(f32),
}

impl From<FontSize> for f32 {
    fn from(value: FontSize) -> Self {
        return match value {
            FontSize::Large => 60.0,
            FontSize::Medium => 40.0,
            FontSize::Small => 30.0,
            FontSize::Px(px) => px,
        };
    }
}

#[derive(Default, Clone, Copy)]
pub enum Font {
    #[default]
    Main,
}

impl From<Font> for AssetPath<'static> {
    fn from(value: Font) -> Self {
        return match value {
            Font::Main => "fonts/Kenney Space.ttf".into(),
        };
    }
}

#[derive(Default, Copy, Clone)]
pub struct TextStyle {
    pub font: Font,
    pub font_size: FontSize,
}

impl TextStyle {
    pub const fn new(font_size: FontSize) -> Self {
        return Self {
            font_size,
            font: Font::Main,
        };
    }
}
