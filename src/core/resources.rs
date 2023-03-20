use bevy::prelude::*;

// use iyes_loopless::prelude::*;

use crate::core::states::AppState;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_startup_system(create_resources);
	}
}

pub fn create_resources(
	mut state: ResMut<NextState<AppState>>,
	_asset_server: Res<AssetServer>,
) {
	state.set(AppState::Loaded);
}
