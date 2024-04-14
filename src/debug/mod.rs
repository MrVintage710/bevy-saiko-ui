mod bounds_debugger;

use bevy::prelude::*;

//==============================================================================
//             SaikoDebuggerPlugin
//==============================================================================

pub struct SaikoDebuggerPlugin;

impl Plugin for SaikoDebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bounds_debugger::SaikoBoundsDebuggerPlugin);
        app.init_resource::<SaikoDebuggerState>();
    }
}

//==============================================================================
//             SaikoDebuggerState
//==============================================================================

#[derive(Resource)]
pub struct SaikoDebuggerState {
    pub show_bounds: bool
}

impl Default for SaikoDebuggerState {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self {
                show_bounds: true
            }
        } else {
            Self {
                show_bounds: false
            }
        }
    }
}

//==============================================================================
//             SaikoDebuggerState
//==============================================================================

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SaikoDebuggerGroup;

pub fn init_debug_group(
    mut config_store: ResMut<GizmoConfigStore>
) {
    let config = config_store.config_mut::<SaikoDebuggerGroup>().0;
}