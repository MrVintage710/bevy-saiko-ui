pub mod node;
pub mod position;
pub mod component;

use bevy::prelude::*;

use self::{component::{rect::RectComponent, SaikoComponentPlugin}, node::SaikoNode};

pub struct SaikoUiPlugin;

impl Plugin for SaikoUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SaikoComponentPlugin::<RectComponent>::default())
        
            .register_type::<SaikoNode>()
        ;
    }
}