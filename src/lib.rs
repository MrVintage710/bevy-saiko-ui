pub mod common;
pub mod render;
pub mod debug;
pub mod ui;

use bevy::prelude::*;

pub struct SaikoUiPlugin;

impl Plugin for SaikoUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(render::SaikoRenderPlugin);
        app.add_plugins(common::SaikoUiCommonPlugin);
        app.add_plugins(ui::SaikoUiPlugin);
        app.add_plugins(debug::SaikoDebuggerPlugin);
    }
}
