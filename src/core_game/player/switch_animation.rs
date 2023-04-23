use bevy::prelude::*;

use crate::core_game::player::player_structs::MyPlayerAnimations;
use crate::core_game::player::player_structs::PlayerAnimationState::*;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerStateBuffer;
use crate::core_game::player::player_structs::Vel;

use crate::core_game::player::player_structs::Player;
use crate::core_game::player::player_structs::PlayerGraphics;

pub fn switch_animation(
	anims: Option<Res<MyPlayerAnimations>>,
	player_query: Query<Entity, With<PlayerGraphics>>,
	vel_query: Query<(&PlayerStateBuffer, &Vel), With<Player>>,
	mut commands: Commands,
	mut float_counter: Local<u32>,
	mut clock: Local<bool>,
) {
	if let Some(anims) = anims {
		for (state, velocity) in vel_query.iter() {
			if state.new.animation == Fall && velocity.y == 0.0 {
				*float_counter += 1;
			}
			if state.new.animation != Fall {
				*float_counter = 0;
			}
			for e in player_query.iter() {
				let same_attack: bool = state.new.attack != state.old.attack;
				if (same_attack && state.new.animation.is_attacking()) {
					*clock = !*clock;
				}
				let no_direction: bool = state.new.direction == PlayerDirectionState::None;

				let cloned = match state.new.animation {
					Run 						=> anims.run.clone(),
					Idle 						=> anims.idle.clone(),
					Jump if no_direction 		=> anims.jump.clone(),
					Jump 						=> anims.jumpd.clone(),
					Fall if velocity.y < 0.0 	=> anims.fall.clone(),
					WallSlide 					=> anims.slide.clone(),
					WhirlwindHammer 			=> anims.whirl.clone(),
					RunIdle 					=> anims.runidle.clone(),
				    IdleWhirlwind 				=> anims.idlewhirl.clone(),
				    WhirlwindIdle 				=> anims.whirlidle.clone(),
					FallIdle 					=> anims.fallidle.clone(),
					MeleeBasicSword if *clock 	=> anims.mbs1.clone(),
					MeleeBasicSword 			=> anims.mbs2.clone(),
					MeleeBasicHammer if *clock 	=> anims.mbh1.clone(),
					MeleeBasicHammer 			=> anims.mbh2.clone(),
					RangedBasicGunsForward 		=> anims.rbgf1.clone(),
					RangedBasicGunsUp if *clock => anims.rbgu1.clone(),
					RangedBasicBowForward 		=> anims.rbbf.clone(),
					RangedBasicBowUp 			=> anims.rbbu.clone(),
					Fall => {
						println!("no fall state with velocity y: {:?}", velocity.y);
						anims.fall.clone()
					},
					x => {
						println!("No current arm for the state: {:?}", x);
						anims.idle.clone()
					}

				};
				commands.entity(e).insert(cloned);
			}
		}
	}
}
