use bevy::prelude::*;

pub fn create_window() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: "Space Invaders Remastered".to_owned(),
			..default()
		}),
		..default()
	}
}
