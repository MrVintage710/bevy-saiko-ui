use bevy::prelude::*;

#[derive(Asset, TypePath)]
pub struct SaikoFontAtlas {
    #[allow(dead_code)]
    texture : Handle<Image>,
    font_data : Vec<FontData>
}

pub struct FontData {}

#[derive(Default)]
struct SaikoFontAtlasLoader;

pub enum SaikoFontErrors {
    
}