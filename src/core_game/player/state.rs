//player state
//maybe move up into core_game and use for enemy also
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::core_game::{animation::Animation, loading::PlayerAssets};

use super::player_structs::Player;

// struct PlayerState;
//
// impl Plugin for PlayerState {
// 	fn build(
// 		&self,
// 		app: &mut App,
// 	) {
// 	    app.
// 	}
// }
//

//need two state machines, one for movement and one for attack
//need to have an entity for each, as child of player

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Running;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Jumping;

//~~~attack states~~~

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
struct Attacking;
//skill attacking with? or separate state for each skill?
//skill timers?
//

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
	Run,
	Jump,
}

fn jumping() {}

pub fn init_input(
	query: Query<(Entity, &Player)>,
	mut commands: Commands,
) {
	for (entity, _player) in query.iter() {
		commands
			.entity(entity)
			.insert(InputManagerBundle::<PlayerAction> {
				action_state: ActionState::default(),
				input_map: InputMap::new([(KeyCode::D, PlayerAction::Run)]),
			});
	}
}

pub fn init_state(
	query: Query<(Entity, &Player)>,
	mut commands: Commands,
	player_assets: Res<PlayerAssets>,
) {
	let run = player_assets.run.clone();

	for (entity, _player) in query.iter() {
		let movement_state_e = commands
			.entity(entity)
			.insert((
				StateMachine::default()
					.trans::<Idle>(PressedTrigger(PlayerAction::Run), Running)
					// .on_enter::<Running>(move |entity| {
					// 	entity.insert((
					// 		Animation(benimator::Animation::from_indices(
					// 			0..=6,
					// 			benimator::FrameRate::from_fps(12.0),
					// 		)),
					// 		SpriteSheetBundle {
					// 			texture_atlas: run,
					// 			sprite: TextureAtlasSprite::new(0),
					// 			..default()
					// 		},
					// 	));
					// })
					.set_trans_logging(true),
				Idle,
			))
			.id();
		println!("Inserted StateMachine: {:?}", movement_state_e);
	}
}

//should b in player animation state machine

pub fn runnning(
	// mut query: Query<(&mut Animation, &mut Handle<TextureAtlas>, Added<Running>)>,
	mut query: Query<(Entity, &Children, Added<Running>)>,
	mut animation_query: Query<(&mut Animation, &mut Handle<TextureAtlas>)>,
	assets: Res<PlayerAssets>,
) {
	for (entity, children) in query.iter() {
		//need a way to identify children
		//handle to animation entity?
	}
	for (mut animation, mut sprite, _added) in animation_query.iter_mut() {
		animation.0 =
			benimator::Animation::from_indices(0..=6, benimator::FrameRate::from_fps(12.0));
		*sprite = assets.run.clone();
		println!("Running sprite changed");
	}
}
