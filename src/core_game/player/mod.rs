use crate::core::states::AppState;
use bevy::prelude::*;

use self::state::init_input;
use self::state::init_state;
// use iyes_loopless::prelude::*;

mod animate;
mod apply_player_state;
mod audio_test;
mod generate_randomness;
pub mod get_player_input;
mod input;
mod move_camera;
mod movement_and_collisions;
mod player_casts;
mod player_deal_damage;
pub mod player_structs;
pub mod reset_player_input;
mod screen_shake;
mod set_animation_state;
mod set_attack_state;
mod set_move_state;
pub mod setup_camera;
pub mod setup_player;
pub mod state;
mod switch_animation;
mod teleport_to_spawn;
mod time_divisions;
mod transfer_data;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_systems(
			(
				// setup_player::setup_player.in_schedule(OnEnter(AppState::Loaded)),
				setup_player::setup_player,
				apply_system_buffers,
				init_input,
				init_state,
			)
				.chain()
				.in_schedule(OnEnter(AppState::Loaded)),
		)
		.add_system(state::runnning.in_set(OnUpdate(AppState::Loaded)))
		.add_systems(
			(
				//setup_player,
				player_casts::player_casts,
				set_attack_state::set_attack_state,
				set_move_state::set_move_state,
				apply_player_state::apply_player_state,
				movement_and_collisions::movement_and_collisions,
				transfer_data::transfer_data,
				teleport_to_spawn::teleport_to_spawn,
				move_camera::move_camera,
				screen_shake::screen_shake,
				set_animation_state::set_animation_state,
				switch_animation::switch_animation,
				// time_divisions::time_divisions,
				// animate::animate,
				reset_player_input::reset_player_input,
				player_deal_damage::player_deal_damage,
			)
				.chain()
				.in_schedule(CoreSchedule::FixedUpdate),
		);
	}
}
