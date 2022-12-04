use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody};
use bevy_ecs_ldtk::prelude::*;

use crate::core_game::creature::creature_structs::Creature;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::MyCreatureAnimations;
use crate::core_game::creature::creature_structs::AnimationParams;
use crate::core_game::creature::creature_structs::Vel;
use crate::core_game::creature::creature_structs::MoveSpeed;
use crate::core_game::creature::creature_structs::CreatureMoveState;
use crate::core_game::creature::creature_structs::CreatureDirectionState;
use crate::core_game::creature::creature_structs::CreatureAnimationState;
use crate::core_game::creature::creature_structs::CreatureState;
use crate::core_game::creature::creature_structs::CreatureStateVariables;
use crate::core_game::creature::creature_structs::CreatureStats;
use crate::core_game::creature::creature_structs::TimeDivisions;

use crate::core_game::creature::creature_structs::CreatureSpawnPoint;

use crate::core_game::creature::creature_structs::CreatureGraphicsEntity;
use crate::core_game::creature::creature_structs::CreatureUsefulVariables;


pub fn periodic_spawn(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut query_spawnpoint: Query<&mut CreatureSpawnPoint>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    query_creature: Query<Entity, With<Creature>>,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
) {

    

        
        


        for mut spawnpoint in query_spawnpoint.iter_mut() {
            if let Some(e_creature) = spawnpoint.current {
                if query_creature.get(e_creature).is_err() {
                    spawnpoint.current = None;
                    
                }
            }
            if spawnpoint.current.is_none() {
                
                spawnpoint.timer.tick(time.delta());
                if spawnpoint.timer.finished() {

                    spawnpoint.timer.reset();
                    
                    
                    let e_creature = commands.spawn()
                        .insert_bundle(TransformBundle {
                            local: spawnpoint.position,
                            ..Default::default()
                        })
                        .insert(Vel {
                            x: 0.0,
                            y: 0.0,
                            dir: 0.0,
                        })
                        .insert(MoveSpeed {
                            x: 0.125,
                            y: 0.0,
                        })
                        .insert(Creature)
                        .insert(CreatureState {
                            old: (CreatureMoveState::Idle, CreatureDirectionState::None, CreatureAnimationState::Idle),
                            new: (CreatureMoveState::Idle, CreatureDirectionState::None, CreatureAnimationState::Idle),
                        })
                        .insert(CreatureStateVariables {
                            chase_direction: 1.0,
                            patrol_timer: 0,
                            idle_timer: 0,
                            reset_velocity: true,
                            attack_range_offset: 0.0,
                        })
                        .insert(TimeDivisions {
                            two: 0,
                            three: 0,
                            four: 0,
                            five: 0,
                            six: 0,
                            seven: 0,
                            eight: 0,
                            nine: 0,
                            ten: 0,
                            eleven: 0,
                            twelve: 0,
                            thirteen: 0,
                            fourteen: 0,
                            fifteen: 0,
                            reset: false,
                        })
                        .insert(CreatureUsefulVariables {
                            chase_delay: 0,
                            attack_delay: 0,
                        })
                        .insert(CreatureStats{
                            life: 192.0,
                        })
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(Collider::cuboid(9.0, 5.0))
                        .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2))
                        .id();




                    

                    
                    spawnpoint.current = Some(e_creature);
                    if spawnpoint.current.is_some() {
                    }
                    // SET UP GRAPHICS ENTITY

                    let perfect_transitions = true;
                    let texture_handle_idle = asset_server.load("animations/creature/IcePaukIdle.png");
                    let texture_atlas_idle = TextureAtlas::from_grid(texture_handle_idle, Vec2::new(90.0, 90.0), 17, 1);
                    let texture_atlas_handle_idle = texture_atlases.add(texture_atlas_idle);
                    let texture_handle_walkf = asset_server.load("animations/creature/IcePaukWalk.png");
                    let texture_atlas_walkf = TextureAtlas::from_grid(texture_handle_walkf, Vec2::new(90.0, 90.0), 10, 1);
                    let texture_atlas_handle_walkf = texture_atlases.add(texture_atlas_walkf);
                    let texture_handle_atk = asset_server.load("animations/creature/IcePaukOffensiveAttack.png");
                    let texture_atlas_atk = TextureAtlas::from_grid(texture_handle_atk, Vec2::new(90.0, 90.0), 11, 1);
                    let texture_atlas_handle_atk = texture_atlases.add(texture_atlas_atk);

                    commands.insert_resource(MyCreatureAnimations {
                        idle: AnimationParams {
                            atlas: texture_atlas_handle_idle.clone(),
                            start: 0,
                            restart: 0,
                            end: 17,
                            perfect_transitions: true,
                        },
                        walkf: AnimationParams {
                            atlas: texture_atlas_handle_walkf.clone(),
                            start: 0,
                            restart: 0,
                            end: 10,
                            perfect_transitions: true,
                        },
                        atk: AnimationParams {
                            atlas: texture_atlas_handle_atk.clone(),
                            start: 0,
                            restart: 0,
                            end: 11,
                            perfect_transitions: true,
                        }
                    });


                    // spawn the entity
                    let e_graphics = commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle_idle.clone(),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                            visibility: Visibility { is_visible: true },
                            ..Default::default()
                        })
                        .insert(CreatureGraphics)
                        .insert(AnimationParams {
                            atlas: texture_atlas_handle_idle.clone(),
                            start: 0,
                            restart: 0,
                            end: 0,
                            perfect_transitions,
                        })
                        .insert(TimeDivisions {
                            two: 0,
                            three: 0,
                            four: 0,
                            five: 0,
                            six: 0,
                            seven: 0,
                            eight: 0,
                            nine: 0,
                            ten: 0,
                            eleven: 0,
                            twelve: 0,
                            thirteen: 0,
                            fourteen: 0,
                            fifteen: 0,
                            reset: false,
                        })
                        .id();

                    commands.entity(e_creature)
                        .insert(CreatureGraphicsEntity(e_graphics));
                }
            }

        
        }
    }