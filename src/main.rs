mod my_plugin;

use crate::my_plugin::HelloPlugin;
use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}