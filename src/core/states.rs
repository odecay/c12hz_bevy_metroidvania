use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
	#[default]
	Loading,
	Loaded,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
	#[default]
	MainMenu,
	InGame,
}
