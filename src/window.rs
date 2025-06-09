use bevy::{prelude::*, window::CursorOptions};

use crate::constants::WINDOW_TITLE;

pub fn create_window() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			cursor_options: CursorOptions {
				visible: false,
				..Default::default()
			},
			title: WINDOW_TITLE.to_owned(),
			..default()
		}),
		..default()
	}
}
