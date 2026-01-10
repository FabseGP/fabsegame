use bevy::{prelude::*, window::WindowMode};

use crate::constants::WINDOW_TITLE;

pub fn create_window() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: WINDOW_TITLE.to_owned(),
			mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
			..default()
		}),
		..default()
	}
}
