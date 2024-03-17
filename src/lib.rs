mod render;

use bevy::prelude::*;

pub struct SaikoUiPlugin;

impl Plugin for SaikoUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(render::SaikoRenderPlugin);
    }
}
