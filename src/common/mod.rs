pub mod bounds;
pub mod value;
pub mod util;

use bevy::prelude::*;

//==============================================================================
//          Common Module
//==============================================================================

pub struct SaikoUiCommonPlugin;

impl Plugin for SaikoUiCommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MarkSaikoUiDirty>();
    }
}

//==============================================================================
//          Common Events
//==============================================================================

#[derive(Event)]
pub struct MarkSaikoUiDirty;
