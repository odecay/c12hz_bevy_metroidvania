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
			"player/player.assets.ron",
		)
		.add_collection_to_loading_state::<_, PlayerAssets>(AppState::Loading)
		.add_system(check_loaded.in_schedule(OnEnter(AppState::Loaded)));
		// .add_system(check_loaded.run_if(in_state(AppState::Loaded)));
	}
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
	// A folder loaded to typed asset handles mapped with their file names (not supported on the web)
	// #[asset(path = "player/sound", collection(typed, mapped))]
	// audio_assets: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "ice_impact", collection(typed, mapped))]
	ice_impact: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "hammer_impact", collection(typed, mapped))]
	hammer_impact: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "bass_hit", collection(typed, mapped))]
	bass_hit: HashMap<String, Handle<AudioSource>>,
	#[asset(key = "footstep", collection(typed, mapped))]
	footstep: HashMap<String, Handle<AudioSource>>,
}

fn check_loaded(
	assets: Res<PlayerAssets>,
	sounds: Res<Assets<AudioSource>>,
	asset_server: Res<AssetServer>,
) {
	for (key, handle) in assets.ice_impact.iter() {
		// if let Some(sound) = sounds.get(handle) {
		let load_state = asset_server.get_load_state(handle);
		println!("{}: {:?}", key, load_state);
		// println!("{}:", key);
		// }
	}

	// let ice_impact_1: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce1.ogg");
	// let ice_impact_2: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce2.ogg");
	// let ice_impact_3: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce3.ogg");
	// let ice_impact_4: Handle<AudioSource> = asset_server.load("sound/HammerImpactIce4.ogg");
	//
	// let hammer_impact_1: Handle<AudioSource> = asset_server.load("sound/HammerImpact1.ogg");
	// let hammer_impact_2: Handle<AudioSource> = asset_server.load("sound/HammerImpact2.ogg");
	// let hammer_impact_3: Handle<AudioSource> = asset_server.load("sound/HammerImpact3.ogg");
	// let hammer_impact_4: Handle<AudioSource> = asset_server.load("sound/HammerImpact4.ogg");
	// let hammer_impact_5: Handle<AudioSource> = asset_server.load("sound/HammerImpact5.ogg");
	//
	// let bass_hit_1: Handle<AudioSource> = asset_server.load("sound/ImpactBass1.ogg");
	// let bass_hit_2: Handle<AudioSource> = asset_server.load("sound/ImpactBass2.ogg");
	//
	// let footstep1: Handle<AudioSource> = asset_server.load("sound/Footstep1.ogg");
	// let footstep2: Handle<AudioSource> = asset_server.load("sound/Footstep2.ogg");
	// let footstep3: Handle<AudioSource> = asset_server.load("sound/Footstep3.ogg");
	// let footstep4: Handle<AudioSource> = asset_server.load("sound/Footstep4.ogg");
	// let footstep5: Handle<AudioSource> = asset_server.load("sound/Footstep5.ogg");
	// let footstep6: Handle<AudioSource> = asset_server.load("sound/Footstep6.ogg");
	// let footstep7: Handle<AudioSource> = asset_server.load("sound/Footstep7.ogg");
	// let footstep8: Handle<AudioSource> = asset_server.load("sound/Footstep8.ogg");
	// let footstep9: Handle<AudioSource> = asset_server.load("sound/Footstep9.ogg");
	// let footstep10: Handle<AudioSource> = asset_server.load("sound/Footstep10.ogg");
	// let footstep11: Handle<AudioSource> = asset_server.load("sound/Footstep11.ogg");
	// let footstep12: Handle<AudioSource> = asset_server.load("sound/Footstep12.ogg");
	// let footstep13: Handle<AudioSource> = asset_server.load("sound/Footstep13.ogg");
	//
	// // PRELOAD ANIMATIONS
	// let texture_handle = asset_server.load("animations/newrun4c6.png");
	// let texture_atlas =
	// 	TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 1, None, None);
	// let texture_atlas_handle = texture_atlases.add(texture_atlas);
	//
	// let texture_handle_idle = asset_server.load("animations/PlayerFinal.png");
	// let texture_atlas_idle =
	// 	TextureAtlas::from_grid(texture_handle_idle, Vec2::new(16.0, 16.0), 1, 1, None, None);
	// let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
	//
	// let texture_handle_jump = asset_server.load("animations/jumpUp.png");
	// let texture_atlas_jump =
	// 	TextureAtlas::from_grid(texture_handle_jump, Vec2::new(16.0, 16.0), 3, 1, None, None);
	// let texture_atlas_handle_jump = texture_atlases.add(texture_atlas_jump);
	//
	// let texture_handle_fall = asset_server.load("animations/fallnew.png");
	// let texture_atlas_fall =
	// 	TextureAtlas::from_grid(texture_handle_fall, Vec2::new(16.0, 16.0), 3, 1, None, None);
	// let texture_atlas_handle_fall = texture_atlases.add(texture_atlas_fall);
	//
	// let texture_handle_jumpd = asset_server.load("animations/jumpnew2.png");
	// let texture_atlas_jumpd = TextureAtlas::from_grid(
	// 	texture_handle_jumpd,
	// 	Vec2::new(16.0, 16.0),
	// 	3,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_jumpd = texture_atlases.add(texture_atlas_jumpd);
	//
	// let texture_handle_falld = asset_server.load("animations/fall_directional.png");
	// let texture_atlas_falld = TextureAtlas::from_grid(
	// 	texture_handle_falld,
	// 	Vec2::new(36.0, 24.0),
	// 	4,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_falld = texture_atlases.add(texture_atlas_falld);
	//
	// let texture_handle_slide = asset_server.load("animations/PlayerFinal.png");
	// let texture_atlas_slide = TextureAtlas::from_grid(
	// 	texture_handle_slide,
	// 	Vec2::new(16.0, 16.0),
	// 	1,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_slide = texture_atlases.add(texture_atlas_slide);
	//
	// let texture_handle_whirl = asset_server.load("animations/whirlwind.png");
	// let texture_atlas_whirl = TextureAtlas::from_grid(
	// 	texture_handle_whirl,
	// 	Vec2::new(52.0, 24.0),
	// 	4,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_whirl = texture_atlases.add(texture_atlas_whirl);
	//
	// let texture_handle_runidle = asset_server.load("animations/PlayerFinal.png");
	// let texture_atlas_runidle = TextureAtlas::from_grid(
	// 	texture_handle_runidle,
	// 	Vec2::new(16.0, 16.0),
	// 	1,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_runidle = texture_atlases.add(texture_atlas_runidle);
	//
	// let texture_handle_idlewhirl = asset_server.load("animations/IdleWhirl.png");
	// let texture_atlas_idlewhirl = TextureAtlas::from_grid(
	// 	texture_handle_idlewhirl,
	// 	Vec2::new(52.0, 24.0),
	// 	2,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_idlewhirl = texture_atlases.add(texture_atlas_idlewhirl);
	//
	// let texture_handle_whirlidle = asset_server.load("animations/WhirlIdle.png");
	// let texture_atlas_whirlidle = TextureAtlas::from_grid(
	// 	texture_handle_whirlidle,
	// 	Vec2::new(52.0, 29.0),
	// 	2,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_whirlidle = texture_atlases.add(texture_atlas_whirlidle);
	//
	// let texture_handle_fallidle = asset_server.load("animations/fallidlenew.png");
	// let texture_atlas_fallidle = TextureAtlas::from_grid(
	// 	texture_handle_fallidle,
	// 	Vec2::new(16.0, 16.0),
	// 	2,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_fallidle = texture_atlases.add(texture_atlas_fallidle);
	//
	// let texture_handle_mbs1 = asset_server.load("animations/swordAttack1.png");
	// let texture_atlas_mbs1 =
	// 	TextureAtlas::from_grid(texture_handle_mbs1, Vec2::new(64.0, 24.0), 4, 1, None, None);
	// let texture_atlas_handle_mbs1 = texture_atlases.add(texture_atlas_mbs1);
	//
	// let texture_handle_mbs2 = asset_server.load("animations/swordAttack2.png");
	// let texture_atlas_mbs2 =
	// 	TextureAtlas::from_grid(texture_handle_mbs2, Vec2::new(64.0, 24.0), 4, 1, None, None);
	// let texture_atlas_handle_mbs2 = texture_atlases.add(texture_atlas_mbs2);
	//
	// let texture_handle_mbh1 = asset_server.load("animations/hammerAttack1.png");
	// let texture_atlas_mbh1 =
	// 	TextureAtlas::from_grid(texture_handle_mbh1, Vec2::new(64.0, 24.0), 6, 1, None, None);
	// let texture_atlas_handle_mbh1 = texture_atlases.add(texture_atlas_mbh1);
	//
	// let texture_handle_mbh2 = asset_server.load("animations/hammerAttack2.png");
	// let texture_atlas_mbh2 =
	// 	TextureAtlas::from_grid(texture_handle_mbh2, Vec2::new(64.0, 24.0), 6, 1, None, None);
	// let texture_atlas_handle_mbh2 = texture_atlases.add(texture_atlas_mbh2);
	//
	// let texture_handle_rbbf = asset_server.load("animations/rbbf.png");
	// let texture_atlas_rbbf =
	// 	TextureAtlas::from_grid(texture_handle_rbbf, Vec2::new(32.0, 32.0), 4, 1, None, None);
	// let texture_atlas_handle_rbbf = texture_atlases.add(texture_atlas_rbbf);
	//
	// let texture_handle_rbbu = asset_server.load("animations/rbbu.png");
	// let texture_atlas_rbbu =
	// 	TextureAtlas::from_grid(texture_handle_rbbu, Vec2::new(32.0, 32.0), 4, 1, None, None);
	// let texture_atlas_handle_rbbu = texture_atlases.add(texture_atlas_rbbu);
	//
	// let texture_handle_rbgf1 = asset_server.load("animations/rbgf1.png");
	// let texture_atlas_rbgf1 = TextureAtlas::from_grid(
	// 	texture_handle_rbgf1,
	// 	Vec2::new(48.0, 32.0),
	// 	3,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_rbgf1 = texture_atlases.add(texture_atlas_rbgf1);
	//
	// let texture_handle_rbgf2 = asset_server.load("animations/rbgf2.png");
	// let texture_atlas_rbgf2 = TextureAtlas::from_grid(
	// 	texture_handle_rbgf2,
	// 	Vec2::new(48.0, 32.0),
	// 	3,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_rbgf2 = texture_atlases.add(texture_atlas_rbgf2);
	//
	// let texture_handle_rbgu1 = asset_server.load("animations/rbgu1.png");
	// let texture_atlas_rbgu1 = TextureAtlas::from_grid(
	// 	texture_handle_rbgu1,
	// 	Vec2::new(48.0, 32.0),
	// 	3,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_rbgu1 = texture_atlases.add(texture_atlas_rbgu1);
	//
	// let texture_handle_rbgu2 = asset_server.load("animations/rbgu2.png");
	// let texture_atlas_rbgu2 = TextureAtlas::from_grid(
	// 	texture_handle_rbgu2,
	// 	Vec2::new(48.0, 32.0),
	// 	3,
	// 	1,
	// 	None,
	// 	None,
	// );
	// let texture_atlas_handle_rbgu2 = texture_atlases.add(texture_atlas_rbgu2);
}
