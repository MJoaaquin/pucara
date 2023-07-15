use bevy::prelude::*;

#[derive(Resource)]
pub struct Health {
    pub value: u32,
}

impl Default for Health {
    fn default() -> Self {
        Health { value: 100 }
    }
}

#[derive(Resource)]
pub struct Points {
    pub value: u32,
}

impl Default for Points {
    fn default() -> Self {
        Points { value: 0 }
    }
}
