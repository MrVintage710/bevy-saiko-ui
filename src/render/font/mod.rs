//==============================================================================
//  this file contains the logic for loading a font in and creating SDF data
//  for the font. It uses a library for doing this called msdfgen. In the future,
//  there maybe a custom implementation of this, but for now, this is the best.
//==============================================================================

pub mod sdf;

use bevy::{prelude::*, render::RenderApp};
use ttf_parser::OutlineBuilder;
use self::sdf::SaikoFontSdfPlugin;

//==============================================================================
//             SaikoFontPlugin
//==============================================================================

pub struct SaikoFontPlugin;

impl Plugin for SaikoFontPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_plugins(SaikoFontSdfPlugin);
        
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
    }
}

//==============================================================================
//             GlyphBuilder
//==============================================================================

#[derive(Default)]
struct GlyphBuilder {
    lines : Vec<GlyphCurve>,
    current_pos : Vec2
}

impl GlyphBuilder {
    fn finalize(mut self) -> Vec<GlyphCurve> {
        
        
        // self
        //     .lines.iter_mut()
        //     .for_each(|outline| outline.normalize(&self.dims))
        // ;
        
        self.lines
    }
}

impl OutlineBuilder for GlyphBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        
        
        self.current_pos.x = x;
        self.current_pos.y = y;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let p0 = self.current_pos;
        let p2 = Vec2::new(x, y);
        let p1 = p2 * 0.5 - p0 * 0.5 + p0;
        self.lines.push(GlyphCurve{
            start : p0, 
            control : p1, 
            end : p2
        });
        self.move_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.lines.push(GlyphCurve {
            start : self.current_pos.clone(), 
            control : Vec2::new(x1, y1), 
            end : Vec2::new(x, y)
        });
        self.move_to(x, y);
        
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let q0 = self.current_pos;
        let q1 = Vec2::new(x1, y1);
        let q2 = Vec2::new(x2, y2);
        let q3 = Vec2::new(x, y);
        let quarter = 1.0 / 4.0;
        let three_quarters = 3.0 / 4.0;
        let p1 = -quarter * q0 + three_quarters * q1 + three_quarters * q2 - quarter * q3;
        
        self.lines.push(GlyphCurve {
            start : q0, 
            control : p1, 
            end : q3
        });
        self.move_to(x, y)
    }

    fn close(&mut self) {
        self.current_pos.x = 0.0;
        self.current_pos.y = 0.0;
    }
}

// #[derive(PartialEq, Debug, Clone)]
// enum GlyphOutline {
//     Line(Vec2, Vec2),
//     Curve(Vec2, Vec2, Vec2),
// }

#[derive(Debug, Clone, Copy)]
struct GlyphCurve {
    start : Vec2,
    control : Vec2,
    end : Vec2
}

// impl GlyphOutline {
//     fn normalize (&mut self, scale : &Vec2) {
//         match self {
//             GlyphOutline::Line(start, end) => {
//                 *start = *start / *scale;
//                 *end = *end / *scale;
//             },
//             GlyphOutline::Curve(start, control, end) => {
//                 *start = *start / *scale;
//                 *control = *control / *scale;
//                 *end = *end / *scale;
//             }
//         }
//     }
// }

//==============================================================================
//             SaikoCharacterSet
//==============================================================================

#[derive(Debug, Clone)]
pub struct SaikoCharacterSet {
    characters : Vec<char>
}

impl Default for SaikoCharacterSet {
    fn default() -> Self {
        SaikoCharacterSet::ascii()
    }
}

impl From<Vec<char>> for SaikoCharacterSet {
    fn from(characters: Vec<char>) -> Self {
        SaikoCharacterSet {
            characters
        }
    }
}

impl From<&str> for SaikoCharacterSet {
    fn from(characters: &str) -> Self {
        SaikoCharacterSet {
            characters : characters.chars().collect()
        }
    }
}

impl From<&[char]> for SaikoCharacterSet {
    fn from(characters: &[char]) -> Self {
        SaikoCharacterSet {
            characters : characters.to_vec()
        }
    }
}

impl SaikoCharacterSet {
    pub fn ascii() -> SaikoCharacterSet {
        SaikoCharacterSet {
            characters : vec![
                '\u{00}', '\u{01}', '\u{02}', '\u{03}', '\u{04}', '\u{05}', '\u{06}', '\u{07}', '\u{08}', '\u{09}', '\u{0A}', '\u{0B}', '\u{0C}', '\u{0D}', '\u{0E}', '\u{0F}',
                '\u{10}', '\u{11}', '\u{12}', '\u{13}', '\u{14}', '\u{15}', '\u{16}', '\u{17}', '\u{18}', '\u{19}', '\u{1A}', '\u{1B}', '\u{1C}', '\u{1D}', '\u{1E}', '\u{1F}',
                '\u{20}', '\u{21}', '\u{22}', '\u{23}', '\u{24}', '\u{25}', '\u{26}', '\u{27}', '\u{28}', '\u{29}', '\u{2A}', '\u{2B}', '\u{2C}', '\u{2D}', '\u{2E}', '\u{2F}',
                '\u{30}', '\u{31}', '\u{32}', '\u{33}', '\u{34}', '\u{35}', '\u{36}', '\u{37}', '\u{38}', '\u{39}', '\u{3A}', '\u{3B}', '\u{3C}', '\u{3D}', '\u{3E}', '\u{3F}',
                '\u{40}', '\u{41}', '\u{42}', '\u{43}', '\u{44}', '\u{45}', '\u{46}', '\u{47}', '\u{48}', '\u{49}', '\u{4A}', '\u{4B}', '\u{4C}', '\u{4D}', '\u{4E}', '\u{4F}',
                '\u{50}', '\u{51}', '\u{52}', '\u{53}', '\u{54}', '\u{55}', '\u{56}', '\u{57}', '\u{58}', '\u{59}', '\u{5A}', '\u{5B}', '\u{5C}', '\u{5D}', '\u{5E}', '\u{5F}',
                '\u{60}', '\u{61}', '\u{62}', '\u{63}', '\u{64}', '\u{65}', '\u{66}', '\u{67}', '\u{68}', '\u{69}', '\u{6A}', '\u{6B}', '\u{6C}', '\u{6D}', '\u{6E}', '\u{6F}',
                '\u{70}', '\u{71}', '\u{72}', '\u{73}', '\u{74}', '\u{75}', '\u{76}', '\u{77}', '\u{78}', '\u{79}', '\u{7A}', '\u{7B}', '\u{7C}', '\u{7D}', '\u{7E}', '\u{7F}'
            ]
        }
    }
    
    pub fn japanese_hiragana() -> SaikoCharacterSet {
        SaikoCharacterSet {
            characters : ('\u{3040}'..'\u{309f}').into_iter().collect()
        }
    }
    
    // pub const EXTENDED_ASCII : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : vec![
    //         '\u{00}', '\u{01}', '\u{02}', '\u{03}', '\u{04}', '\u{05}', '\u{06}', '\u{07}', '\u{08}', '\u{09}', '\u{0A}', '\u{0B}', '\u{0C}', '\u{0D}', '\u{0E}', '\u{0F}',
    //         '\u{10}', '\u{11}', '\u{12}', '\u{13}', '\u{14}', '\u{15}', '\u{16}', '\u{17}', '\u{18}', '\u{19}', '\u{1A}', '\u{1B}', '\u{1C}', '\u{1D}', '\u{1E}', '\u{1F}',
    //         '\u{20}', '\u{21}', '\u{22}', '\u{23}', '\u{24}', '\u{25}', '\u{26}', '\u{27}', '\u{28}', '\u{29}', '\u{2A}', '\u{2B}', '\u{2C}', '\u{2D}', '\u{2E}', '\u{2F}',
    //         '\u{30}', '\u{31}', '\u{32}', '\u{33}', '\u{34}', '\u{35}', '\u{36}', '\u{37}', '\u{38}', '\u{39}', '\u{3A}', '\u{3B}', '\u{3C}', '\u{3D}', '\u{3E}', '\u{3F}',
    //         '\u{40}', '\u{41}', '\u{42}', '\u{43}', '\u{44}', '\u{45}', '\u{46}', '\u{47}', '\u{48}', '\u{49}', '\u{4A}', '\u{4B}', '\u{4C}', '\u{4D}', '\u{4E}', '\u{4F}',
    //         '\u{50}', '\u{51}', '\u{52}', '\u{53}', '\u{54}', '\u{55}', '\u{56}', '\u{57}', '\u{58}', '\u{59}', '\u{5A}', '\u{5B}', '\u{5C}', '\u{5D}', '\u{5E}', '\u{5F}',
    //         '\u{60}', '\u{61}', '\u{62}', '\u{63}', '\u{64}', '\u{65}', '\u{66}', '\u{67}', '\u{68}', '\u{69}', '\u{6A}', '\u{6B}', '\u{6C}', '\u{6D}', '\u{6E}', '\u{6F}',
    //         '\u{70}', '\u{71}', '\u{72}', '\u{73}', '\u{74}', '\u{75}', '\u{76}', '\u{77}', '\u{78}', '\u{79}', '\u{7A}', '\u{7B}', '\u{7C}', '\u{7D}', '\u{7E}', '\u{7F}',
    //         '\u{80}', '\u{81}', '\u{82}', '\u{83}', '\u{84}', '\u{85}', '\u{86}', '\u{87}', '\u{88}', '\u{89}', '\u{8A}', '\u{8B}', '\u{8C}', '\u{8D}', '\u{8E}', '\u{8F}',
    //         '\u{90}', '\u{91}', '\u{92}', '\u{93}', '\u{94}', '\u{95}', '\u{96}', '\u{97}', '\u{98}', '\u{99}', '\u{9A}', '\u{9B}', '\u{9C}', '\u{9D}', '\u{9E}', '\u{9F}',
    //         '\u{A0}', '\u{A1}', '\u{A2}', '\u{A3}', '\u{A4}', '\u{A5}', '\u{A6}', '\u{A7}', '\u{A8}', '\u{A9}', '\u{AA}', '\u{AB}', '\u{AC}', '\u{AD}', '\u{AE}', '\u{AF}',
    //         '\u{B0}', '\u{B1}', '\u{B2}', '\u{B3}', '\u{B4}', '\u{B5}', '\u{B6}', '\u{B7}', '\u{B8}', '\u{B9}', '\u{BA}', '\u{BB}', '\u{BC}', '\u{BD}', '\u{BE}', '\u{BF}',
    //         '\u{C0}', '\u{C1}', '\u{C2}', '\u{C3}', '\u{C4}', '\u{C5}', '\u{C6}', '\u{C7}', '\u{C8}', '\u{C9}', '\u{CA}', '\u{CB}', '\u{CC}', '\u{CD}', '\u{CE}', '\u{CF}',
    //         '\u{D0}', '\u{D1}', '\u{D2}', '\u{D3}', '\u{D4}', '\u{D5}', '\u{D6}', '\u{D7}', '\u{D8}', '\u{D9}', '\u{DA}', '\u{DB}', '\u{DC}', '\u{DD}', '\u{DE}', '\u{DF}',
    //         '\u{E0}', '\u{E1}', '\u{E2}', '\u{E3}', '\u{E4}', '\u{E5}', '\u{E6}', '\u{E7}', '\u{E8}', '\u{E9}', '\u{EA}', '\u{EB}', '\u{EC}', '\u{ED}', '\u{EE}', '\u{EF}',
    //         '\u{F0}', '\u{F1}', '\u{F2}', '\u{F3}', '\u{F4}', '\u{F5}', '\u{F6}', '\u{F7}', '\u{F8}', '\u{F9}', '\u{FA}', '\u{FB}', '\u{FC}', '\u{FD}', '\u{FE}', '\u{FF}',
    //     ]
    // };
    
    // pub const JAPANESE_HIRAGANA : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{3040}'..'\u{309f}').into_iter().collect()
    // };
    
    // pub const JAPANESE_KATAKANA : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{30a0}'..'\u{30ff}').into_iter().collect()
    // };
    
    // pub const JAPANESE_KANJI : SaikoCharacterSet = SaikoCharacterSet {
    //     characters : ('\u{4e00}'..'\u{9faf}').into_iter().collect()
    // };
    
    pub fn extend(&mut self, other : impl Into<SaikoCharacterSet>) {
        self.characters.extend(other.into().characters);
    }
}

impl IntoIterator for SaikoCharacterSet {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.characters.into_iter()
    }
}