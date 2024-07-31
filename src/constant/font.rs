pub const MAIN_FONT_ASSET_PATH: &str = "fonts/Kenney Space.ttf";

pub enum Size {
    Large,
    Medium,
    Small,
    Px(f32),
}

impl From<Size> for f32 {
    fn from(value: Size) -> Self {
        return match value {
            Size::Large => 60.0,
            Size::Medium => 40.0,
            Size::Small => 30.0,
            Size::Px(px) => px,
        };
    }
}

pub struct Font {
    asset: String,
    size: Size,
}

impl Default for Font {
    fn default() -> Self {
        return Self {
            asset: MAIN_FONT_ASSET_PATH.into(),
            size: Size::Medium,
        };
    }
}

impl Font {
    pub fn new(size: Size) -> Self {
        return Self {
            size,
            ..Default::default()
        };
    }
}
