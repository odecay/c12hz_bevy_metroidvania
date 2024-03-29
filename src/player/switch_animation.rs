use bevy::{prelude::*};


use crate::player::MyPlayerAnimations;
use crate::player::PlayerState;
use crate::player::PlayerAnimationState;
use crate::player::PlayerDirectionState;
use crate::player::Vel;
use crate::player::TimeDivisions;

use crate::player::Player;
use crate::player::PlayerGraphics;

pub fn switch_animation(
    anims: Option<Res<MyPlayerAnimations>>,
    player_query: Query<(Entity, &TimeDivisions), With<PlayerGraphics>>,
    vel_query: Query<(&PlayerState, &Vel), With<Player>>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut float_counter: Local<u32>,
    mut clock: Local<bool>,
) {
    if let Some(anims) = anims {
        for (state, velocity) in vel_query.iter() {
            if state.new.2 == PlayerAnimationState::Fall && velocity.y == 0.0 {
                *float_counter += 1;
            }
            if state.new.2 != PlayerAnimationState::Fall {
                *float_counter = 0;
            }
            for (e, time) in player_query.iter() {


                if state.new.2 == PlayerAnimationState::Run {
                    commands.entity(e).insert(anims.run.clone());
                }
                
                if state.new.2 == PlayerAnimationState::Idle {
                    commands.entity(e).insert(anims.idle.clone());
                }

                if state.new.2 == PlayerAnimationState::Jump {
                    if state.new.1 == PlayerDirectionState::None {
                        commands.entity(e).insert(anims.jump.clone());
                    }
                    else {
                        commands.entity(e).insert(anims.jumpd.clone());
                    }
                    
                }

                //technically player can sometimes still have upwards velocity in the Fall state
                //because ending the jump early will move to Fall state but the velocity is not immediately set to 0
                //but is instead cut 3 times to make it look more natural
                //that's why I'm also checking here if velocity is less than 0.0 to start the Fall animation.
                if state.new.2 == PlayerAnimationState::Fall && velocity.y < 0.0 {
                    commands.entity(e).insert(anims.fall.clone());
                }

                if state.new.2 == PlayerAnimationState::WallSlide {

                    commands.entity(e).insert(anims.slide.clone());
                }


                if state.new.2 == PlayerAnimationState::Whirlwind {
                    commands.entity(e).insert(anims.whirl.clone());
                }

                if state.new.2 == PlayerAnimationState::RunIdle {
                    commands.entity(e).insert(anims.runidle.clone());
                }

                if state.new.2 == PlayerAnimationState::IdleWhirlwind {
                    commands.entity(e).insert(anims.idlewhirl.clone());
                }

                if state.new.2 == PlayerAnimationState::WhirlwindIdle {
                    commands.entity(e).insert(anims.whirlidle.clone());
                }

                if state.new.2 == PlayerAnimationState::FallIdle {
                    commands.entity(e).insert(anims.fallidle.clone());
                }

                if state.new.2 == PlayerAnimationState::SwordHitBasic {
                    if keys.just_pressed(KeyCode::G) {
                        *clock = !*clock;
                    }

                    if *clock == true {
                        commands.entity(e).insert(anims.swdatkbsc1.clone());
                    }
                    else {
                        commands.entity(e).insert(anims.swdatkbsc2.clone());
                    }
                }

                if state.new.2 == PlayerAnimationState::HammerHitBasic {
                    if keys.just_pressed(KeyCode::B) {
                        *clock = !*clock;
                    }

                    if *clock == true {
                        commands.entity(e).insert(anims.hmratkbsc1.clone());
                    }
                    else {
                        commands.entity(e).insert(anims.hmratkbsc2.clone());
                    }
                }
            }
        }
    }
}
