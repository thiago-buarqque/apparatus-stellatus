use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

use crate::{plugins::scenario_plugin::ScenarioPlugin, reader::parser::Warehouse};

mod model;
mod plugins;
mod reader;

fn main() {
    let warehouse = Warehouse::from_json_file("./src/resources/sample.json");
    //App::new()
    //    .add_plugins((
    //        DefaultPlugins,
    //        FpsOverlayPlugin {
    //            config: FpsOverlayConfig {
    //                text_config: TextFont {
    //                    // Here we define size of our overlay
    //                    font_size: 42.0,
    //                    // If we want, we can use a custom font
    //                    font: default(),
    //                    // We could also disable font smoothing,
    //                    font_smoothing: FontSmoothing::default(),
    //                    ..default()
    //                },
    //                // We can also change color of the overlay
    //                text_color: Color::srgb(0.0, 1.0, 0.0),
    //                // We can also set the refresh interval for the FPS counter
    //                refresh_interval: core::time::Duration::from_millis(250),
    //                enabled: true,
    //            },
    //        },
    //    ))
    //    .add_plugins(ScenarioPlugin)
    //    .run();
}
