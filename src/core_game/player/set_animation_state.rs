use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{Collider, InteractionGroups, QueryFilter, RapierContext},
    rapier::prelude::Group,
};


use crate::core_game::player::player_structs::PlayerState;
use crate::core_game::player::player_structs::PlayerMoveState;
use crate::core_game::player::player_structs::PlayerAnimationState;
use crate::core_game::player::player_structs::PlayerDirectionState;
use crate::core_game::player::player_structs::PlayerStateVariables;
use crate::core_game::player::player_structs::Player;
<<<<<<< Updated upstream:src/core_game/player/set_animation_state.rs

use crate::core_game::player::player_structs::WallKick;
=======
use crate::core_game::player::player_structs::PlayerAttackState;
use crate::core_game::player::player_structs::WallKick;
use crate::core_game::player::player_structs::PlayerInput;
>>>>>>> Stashed changes:src/player/set_animation_state.rs



// this function generates player states
// the idea here is that player states are only dependant on: 
// 1) key presses,
// 2) basic collision checks,
// 3) basic variables (PlayerStateVariables) 
//  - those variables are only modified by the state machine and nothing else.
// this results in a clean 'signal flow', 
// where the outputs of the state machine (like velocity) 
// never feed back into its input.


pub fn set_animation_state (
    mut query: Query<(&mut PlayerState, &mut PlayerStateVariables, &Collider, &Transform, &WallKick, &PlayerInput), With<Player>>,
    rapier_context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
) {


    let mut attacking = false;

<<<<<<< Updated upstream:src/core_game/player/set_animation_state.rs

    for (mut state, mut var, collider, transform, _wall) in query.iter_mut() {
=======
    for (mut state, mut var, collider, transform, wall, input) in query.iter_mut() {
>>>>>>> Stashed changes:src/player/set_animation_state.rs




        if state.new.3 != PlayerAttackState::None {
            attacking = true;
        }



        // RESET PARAMETERS WHEN STATE CHANGES

        if state.new.0 != state.old.0 {
            var.runidle_counter = 0;
            var.idlewhirl_counter = 0;
            var.whirlidle_counter = 0;
        }

        let mut frame_count = 0;
        let frame_duration = 5;



        //RUN > IDLE TRANSITION ANIMATION STATE
        if state.old.0 == PlayerMoveState::Run && state.new.0 == PlayerMoveState::Idle && !attacking {
            frame_count = 5;
            var.runidle_counter = (frame_count * frame_duration) + 1;
        }

        if var.runidle_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RunIdle;
            var.runidle_counter -= 1;

        }

        //IDLE > WHIRLWIND STATE - currently something is not working > stuck in transition animation when trying to use when not running
        if state.old.0 == PlayerMoveState::Idle && state.new.0 == PlayerMoveState::Whirlwind && !attacking {
            frame_count = 2;
            var.idlewhirl_counter = (frame_count * frame_duration) + 1;
        }
        if var.idlewhirl_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::IdleWhirlwind;
            var.idlewhirl_counter -= 1;
 
        }

        //WHIRLWIND > IDLE STATE
        if state.old.0 == PlayerMoveState::Whirlwind && state.new.0 == PlayerMoveState::Idle && !attacking {
            frame_count = 2;
            var.whirlidle_counter = (frame_count * frame_duration) + 1;
        }
        if var.whirlidle_counter > 0 {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::WhirlwindIdle;
            var.whirlidle_counter -= 1;
            
        }

        // FALL > IDLE STATE

        if state.old.0 == PlayerMoveState::Fall && state.new.0 == PlayerMoveState::Run && !attacking {
            //frame_count = 1;
            //var.fallidle_counter = (frame_count * frame_duration) + 1;
        }
        if var.fallidle_counter > 0 {
            //state.old.2 = state.new.2;
            //state.new.2 = PlayerAnimationState::FallIdle;
            //var.fallidle_counter -= 1;
            
        }


        let mut currently_transitioning = var.runidle_counter != 0 || var.idlewhirl_counter != 0 || var.whirlidle_counter != 0 || var.fallidle_counter != 0;






        // IDLE ANIMATION STATE
        if currently_transitioning == false && !attacking {
            if state.new.0 == PlayerMoveState::Idle {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Idle;
            }
        }


        // RUN ANIMATION STATE
        if currently_transitioning == false && !attacking {
            if state.new.0 == PlayerMoveState::Run {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Run;
            }
        }


        // JUMP ANIMATION STATE
        if currently_transitioning == false && !attacking {
            if state.new.0 == PlayerMoveState::Jump {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Jump;
            }
        }


        // FALL ANIMATION STATE
        if currently_transitioning == false && !attacking {
            if state.new.0 == PlayerMoveState::Fall {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Fall;
            }
        }


        // WALL SLIDE ANIMATION STATE
        if currently_transitioning == false && !attacking {
            if state.new.0 == PlayerMoveState::WallSlide {
                let mut side = 0.0;

                if state.new.1 == PlayerDirectionState::Left {
                    side = -1.0;
                }

                if state.new.1 == PlayerDirectionState::Right {
                    side = 1.0;
                }


                let hit_ws_up = rapier_context.cast_shape(
                    Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(0.0, 18.0),
                    0.0,
                    Vec2::new(side, 0.0),
                    collider,
                    1.0,
                    QueryFilter::default()
                    .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                );

                let hit_ws_down = rapier_context.cast_shape(
                    Vec2::new(transform.translation.x, transform.translation.y) + Vec2::new(0.0, -18.0),
                    0.0,
                    Vec2::new(side, 0.0),
                    collider,
                    1.0,
                    QueryFilter::default()
                    .groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
                );

                if hit_ws_up.is_some() && hit_ws_down.is_some() {
                    state.old.2 = state.new.2;
                    state.new.2 = PlayerAnimationState::WallSlide;
                }

                if hit_ws_up.is_none() || hit_ws_down.is_none() {
                    state.old.2 = state.new.2;
                    state.new.2 = PlayerAnimationState::Fall;
                }

                
            }
        }

        // BASIC SWORD ATTACK ANIMATION STATE
        if state.new.3 == PlayerAttackState::MeleeBasicSword {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::MeleeBasicSword;
            currently_transitioning = false
        }

        // BASIC HAMMER ATTACK ANIMATION STATE
        if state.new.3 == PlayerAttackState::MeleeBasicHammer {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::MeleeBasicHammer;
            currently_transitioning = false
        }

        // BASIC BOW FORWARD ATTACK ANIMATION STATE
        if state.new.3 == PlayerAttackState::RangedBasicBowForward {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RangedBasicBowForward;
            currently_transitioning = false
        }

         // BASIC BOW UP ATTACK ANIMATION STATE
         if state.new.3 == PlayerAttackState::RangedBasicBowUp {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RangedBasicBowUp;
            currently_transitioning = false
        }

        // BASIC GUNS FORWARD ATTACK ANIMATION STATE
        if state.new.3 == PlayerAttackState::RangedBasicGunsForward {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RangedBasicGunsForward;
            currently_transitioning = false
        }

        // BASIC GUNS UP ATTACK ANIMATION STATE
        if state.new.3 == PlayerAttackState::RangedBasicGunsUp {
            state.old.2 = state.new.2;
            state.new.2 = PlayerAnimationState::RangedBasicGunsUp;
            currently_transitioning = false
        }


        // WHIRLWIND ANIMATION STATE - currently something is not working > stuck in transition animation when trying to use when not running
        if currently_transitioning == false  {
            if state.new.3 == PlayerAttackState::WhirlwindHammer {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::WhirlwindHammer;
            }
        }

        // WHIRLWIND ANIMATION STATE - currently something is not working > stuck in transition animation when trying to use when not running
        if currently_transitioning == false  {
            if state.new.3 == PlayerAttackState::WhirlwindSword {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::WhirlwindSword;
            }
        }

        // DASH ANIMATION STATE
        if currently_transitioning == false {
            if state.new.3 == PlayerAttackState::DashForward {
                state.old.2 = state.new.2;
                state.new.2 = PlayerAnimationState::Run;
            }
        }
    }
}