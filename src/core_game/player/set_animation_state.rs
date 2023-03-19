use bevy::prelude::*;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerAnimationState;
use crate::core_game::player::player_structs::PlayerAttackState;
use crate::core_game::player::player_structs::PlayerCasts;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerStateBuffer;
use crate::core_game::player::player_structs::PlayerStateVariables;

// this function generates player states
// the idea here is that player states are only dependant on:
// 1) key presses,
// 2) basic collision checks,
// 3) basic variables (PlayerStateVariables)
//  - those variables are only modified by the state machine and nothing else.
// this results in a clean 'signal flow',
// where the outputs of the state machine (like velocity)
// never feed back into its input.

pub fn set_animation_state(
	mut query: Query<
		(
			&mut PlayerStateBuffer,
			&mut PlayerStateVariables,
			&PlayerCasts,
			&Transform,
		),
		With<Player>,
	>
) {
	let mut attacking = false;

	for (mut state, mut var, cast, transform) in query.iter_mut() {
		if state.new.attack != PlayerAttackState::None {
			attacking = true;
		}

		// RESET PARAMETERS WHEN STATE CHANGES

		if state.new.movement != state.old.movement {
			var.runidle_counter = 0;
			var.idlewhirl_counter = 0;
			var.whirlidle_counter = 0;
		}

		let mut frame_count = 0;
		let frame_duration = 5;

		//RUN > IDLE TRANSITION ANIMATION STATE
		if state.old.movement == PlayerMoveState::Run
			&& state.new.movement == PlayerMoveState::Idle
			&& !attacking
		{
			frame_count = 5;
			var.runidle_counter = (frame_count * frame_duration) + 1;
		}

		if var.runidle_counter > 0 {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::RunIdle;
			var.runidle_counter -= 1;
		}

		//IDLE > WHIRLWIND STATE - currently something is not working > stuck in transition animation when trying to use when not running
		if state.old.movement == PlayerMoveState::Idle
			&& state.new.movement == PlayerMoveState::Whirlwind
			&& !attacking
		{
			frame_count = 2;
			var.idlewhirl_counter = (frame_count * frame_duration) + 1;
		}
		if var.idlewhirl_counter > 0 {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::IdleWhirlwind;
			var.idlewhirl_counter -= 1;
		}

		//WHIRLWIND > IDLE STATE
		if state.old.movement == PlayerMoveState::Whirlwind
			&& state.new.movement == PlayerMoveState::Idle
			&& !attacking
		{
			frame_count = 2;
			var.whirlidle_counter = (frame_count * frame_duration) + 1;
		}
		if var.whirlidle_counter > 0 {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::WhirlwindIdle;
			var.whirlidle_counter -= 1;
		}

		// FALL > IDLE STATE

		if state.old.movement == PlayerMoveState::Fall
			&& state.new.movement == PlayerMoveState::Run
			&& !attacking
		{
			//frame_count = 1;
			//var.fallidle_counter = (frame_count * frame_duration) + 1;
		}
		if var.fallidle_counter > 0 {
			//state.old.2 = state.new.2;
			//state.new.2 = PlayerAnimationState::FallIdle;
			//var.fallidle_counter -= 1;
		}

		let mut currently_transitioning = var.runidle_counter != 0
			|| var.idlewhirl_counter != 0
			|| var.whirlidle_counter != 0
			|| var.fallidle_counter != 0;

		// IDLE ANIMATION STATE
		if currently_transitioning == false && !attacking {
			if state.new.movement == PlayerMoveState::Idle {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::Idle;
			}
		}

		// RUN ANIMATION STATE
		if currently_transitioning == false && !attacking {
			if state.new.movement == PlayerMoveState::Run {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::Run;
			}
		}

		// JUMP ANIMATION STATE
		if currently_transitioning == false && !attacking {
			if state.new.movement == PlayerMoveState::Jump {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::Jump;
			}
		}

		// FALL ANIMATION STATE
		if currently_transitioning == false && !attacking {
			if state.new.movement == PlayerMoveState::Fall {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::Fall;
			}
		}

		// WALL SLIDE ANIMATION STATE
		if currently_transitioning == false && !attacking {
			if state.new.movement == PlayerMoveState::WallSlide {
				if cast.wallslide_anim_up && cast.wallslide_anim_down {
					state.old.animation = state.new.animation;
					state.new.animation = PlayerAnimationState::WallSlide;
				}

				if !cast.wallslide_anim_up || !cast.wallslide_anim_down {
					state.old.animation = state.new.animation;
					state.new.animation = PlayerAnimationState::Fall;
				}
			}
		}

		// BASIC SWORD ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::MeleeBasicSword {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::MeleeBasicSword;
			currently_transitioning = false
		}

		// BASIC HAMMER ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::MeleeBasicHammer {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::MeleeBasicHammer;
			currently_transitioning = false
		}

		// BASIC BOW FORWARD ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::RangedBasicBowForward {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::RangedBasicBowForward;
			currently_transitioning = false
		}

		// BASIC BOW UP ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::RangedBasicBowUp {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::RangedBasicBowUp;
			currently_transitioning = false
		}

		// BASIC GUNS FORWARD ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::RangedBasicGunsForward {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::RangedBasicGunsForward;
			currently_transitioning = false
		}

		// BASIC GUNS UP ATTACK ANIMATION STATE
		if state.new.attack == PlayerAttackState::RangedBasicGunsUp {
			state.old.animation = state.new.animation;
			state.new.animation = PlayerAnimationState::RangedBasicGunsUp;
			currently_transitioning = false
		}

		// WHIRLWIND ANIMATION STATE - currently something is not working > stuck in transition animation when trying to use when not running
		if currently_transitioning == false {
			if state.new.attack == PlayerAttackState::WhirlwindHammer {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::WhirlwindHammer;
			}
		}

		// WHIRLWIND ANIMATION STATE - currently something is not working > stuck in transition animation when trying to use when not running
		if currently_transitioning == false {
			if state.new.attack == PlayerAttackState::WhirlwindSword {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::WhirlwindSword;
			}
		}

		// DASH ANIMATION STATE
		if currently_transitioning == false {
			if state.new.attack == PlayerAttackState::DashForward {
				state.old.animation = state.new.animation;
				state.new.animation = PlayerAnimationState::Run;
			}
		}
	}
}
