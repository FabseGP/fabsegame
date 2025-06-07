use bevy::{prelude::*, window::CursorOptions};

pub fn create_window() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			cursor_options: CursorOptions {
				visible: false,
				..Default::default()
			},
			title: "Space Invaders Remastered".to_owned(),
			..default()
		}),
		..default()
	}
}
