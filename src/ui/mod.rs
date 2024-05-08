pub mod component;
pub mod node;
pub mod context;
pub mod position;
pub mod shaping;

use bevy::prelude::*;

use self::{
    component::{rect::RectComponent, SaikoComponentPlugin},
    node::SaikoNodePlugin,
};

pub struct SaikoUiPlugin;

impl Plugin for SaikoUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SaikoComponentPlugin::<RectComponent>::default())
            .add_plugins(SaikoNodePlugin)
            .register_type::<RectComponent>()
        ;
    }
}
