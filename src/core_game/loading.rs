use crate::core::states::AppState;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_loading_state(
			LoadingState::new(AppState::Loading).continue_to_state(AppState::Loaded),
		)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			AppState::Loading,
			"player/audio.assets.ron",
		)
		.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
			AppState::Loading,
			"player/animations.assets.ron",
		)
		.add_collection_to_loading_state::<_, PlayerAssets>(AppState::Loading)
		.add_system(check_loaded.in_schedule(OnEnter(AppState::Loaded)));
		// .add_system(check_loaded.run_if(in_state(AppState::Loaded)));
	}
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
	//audio
	#[asset(key = "ice_impact", collection(typed, mapped))]
	pub ice_impact: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "hammer_impact", collection(typed, mapped))]
	pub hammer_impact: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "bass_hit", collection(typed, mapped))]
	pub bass_hit: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "footstep", collection(typed, mapped))]
	pub footstep: HashMap<String, Handle<AudioSource>>,

	//animations
	#[asset(key = "run")]
	pub run: Handle<TextureAtlas>,
	#[asset(key = "idle")]
	pub idle: Handle<TextureAtlas>,
	//add attack animations
}

fn check_loaded(
	assets: Res<PlayerAssets>,
	sounds: Res<Assets<AudioSource>>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
) {
	for (key, handle) in assets.ice_impact.iter() {
		// if let Some(sound) = sounds.get(handle) {
		let load_state = asset_server.get_load_state(handle);
		println!("{}: {:?}", key, load_state);
		// println!("{}:", key);
		// }
	}
}
