use std::println;

//player state
//maybe move up into core_game and use for enemy also
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::{
	core::states::AppState,
	core_game::{
		animation::{Animation, Facing},
		loading::PlayerAssets,
	},
};

use super::{
	player_structs::{Player, PlayerAnimation, PlayerAttack, PlayerMovement},
	setup_player::AnimationSibling,
	Sets,
};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_event::<MoveStateTransition>().add_systems(
			(init_input, init_state)
				.chain()
				.after(Sets::Setup)
				.in_schedule(OnEnter(AppState::Loaded)),
		);
	}
}

//need two state machines, one for movement and one for attack
//need to have an entity for each, as child of player

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct RunningRight;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct RunningLeft;

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Jumping;

//~~~attack states~~~

#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Attacking;
//skill attacking with? or separate state for each skill?
//skill timers?
//

pub struct MoveStateTransition;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerMoveAction {
	Idle,
	MoveRight,
	MoveLeft,
	Jump,
	Dodge,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAttackAction {
	Idle,
	Skill1,
	Skill2,
	Skill3,
	Skill4,
}

pub fn init_input(
	// query: Query<(Entity, &Player)>,
	movement_query: Query<(Entity, &Parent, With<PlayerMovement>)>,
	attack_query: Query<(Entity, &Parent, With<PlayerAttack>)>,
	mut commands: Commands,
) {
	//bind movement keys
	for (entity, _, _) in movement_query.iter() {
		commands
			.entity(entity)
			.insert(InputManagerBundle::<PlayerMoveAction> {
				action_state: ActionState::default(),
				input_map: InputMap::new([
					(KeyCode::W, PlayerMoveAction::Jump),
					(KeyCode::A, PlayerMoveAction::MoveLeft),
					(KeyCode::S, PlayerMoveAction::Dodge),
					(KeyCode::D, PlayerMoveAction::MoveRight),
				]),
			});
	}
	//bind attack keys
	for (entity, _, _) in attack_query.iter() {
		commands
			.entity(entity)
			.insert(InputManagerBundle::<PlayerAttackAction> {
				action_state: ActionState::default(),
				input_map: InputMap::new([
					(KeyCode::H, PlayerAttackAction::Skill1),
					(KeyCode::J, PlayerAttackAction::Skill2),
					(KeyCode::K, PlayerAttackAction::Skill3),
					(KeyCode::L, PlayerAttackAction::Skill4),
				]),
			});
	}
}

//for input need a "cancelable" state/component that is operated on by animation system, checks
//when animation is in recovery frames.
//annnd need a way to determine if player is in state that can accept new inputs??
//player state machine for attack, movement
//
//can_act?
//cancelable?
//can input condition? incapacitated? stuns and freeze/whatnot add incapacitated componenet,
//checked in condition.
//movement walk, idle, jump/airborne, dashing,
//counter for how many jumps?
//
//determine if can attack?
//attack, attacking, ??

pub struct StateTransitionEvent(Entity);

pub fn init_state(
	query: Query<(Entity, With<Player>)>,
	movement_query: Query<(Entity, &Parent, With<PlayerMovement>)>,
	attack_query: Query<(Entity, &Parent, With<PlayerAttack>)>,
	// animation_query: Query<(Entity, &Parent, With<PlayerAnimation>)>,
	mut commands: Commands,
	// player_assets: Res<PlayerAssets>,
	// mut world: World,
) {
	//okay we want one level of abstraction, animation transitions on their own entity, switching
	//handle for texture atlas directly
	for (entity, _, _) in movement_query.iter() {
		commands.entity(entity).insert((
			StateMachine::default()
				.trans::<Idle>(
					JustPressedTrigger(PlayerMoveAction::MoveRight),
					RunningRight,
				)
				.trans::<Idle>(JustPressedTrigger(PlayerMoveAction::MoveLeft), RunningLeft)
				.trans::<RunningRight>(JustReleasedTrigger(PlayerMoveAction::MoveRight), Idle)
				.trans::<RunningRight>(JustPressedTrigger(PlayerMoveAction::MoveLeft), RunningLeft)
				.trans::<RunningLeft>(JustReleasedTrigger(PlayerMoveAction::MoveLeft), Idle)
				.trans::<RunningLeft>(
					JustPressedTrigger(PlayerMoveAction::MoveRight),
					RunningRight,
				)
				.set_trans_logging(true),
			Idle,
		));
	}
	for (entity, _, _) in attack_query.iter() {
		commands.entity(entity).insert((
			StateMachine::default()
				.trans::<Idle>(JustPressedTrigger(PlayerAttackAction::Skill1), Attacking)
				.set_trans_logging(true),
			Idle,
		));
	}
}

pub fn idle(
	query: Query<(
		Entity,
		&AnimationSibling,
		(With<PlayerMovement>, Added<Idle>),
	)>,
	mut animation_query: Query<(
		&mut Animation,
		&mut Handle<TextureAtlas>,
		&Parent,
		With<PlayerAnimation>,
	)>,
	assets: Res<PlayerAssets>,
) {
	for (entity, animation_entity, _) in query.iter() {
		if let Ok((mut animation, mut sprite, parent, _)) =
			animation_query.get_mut(**animation_entity)
		{
			animation.0 =
				benimator::Animation::from_indices(0..=0, benimator::FrameRate::from_fps(12.0));
			*sprite = assets.idle.clone();
		}
	}
}

pub fn running(
	//for some reason added here acting same as with
	query: Query<(
		Entity,
		&AnimationSibling,
		AnyOf<(&RunningRight, &RunningLeft)>,
		With<PlayerMovement>,
	)>,
	mut animation_query: Query<(
		&mut Animation,
		&mut Handle<TextureAtlas>,
		&mut Facing,
		With<PlayerAnimation>,
	)>,
	assets: Res<PlayerAssets>,
) {
	for (entity, animation_entity, running, _) in query.iter() {
		if let Ok((mut animation, mut sprite, mut facing, _)) =
			animation_query.get_mut(**animation_entity)
		{
			animation.0 =
				benimator::Animation::from_indices(0..=6, benimator::FrameRate::from_fps(12.0));
			*sprite = assets.run.clone();
			match running {
				(Some(RunningRight), None) => *facing = Facing::Right,
				(None, Some(RunningLeft)) => *facing = Facing::Left,
				_ => (),
			}
		}
	}
}

pub fn attacking(
	query: Query<(
		Entity,
		&AnimationSibling,
		(With<PlayerAttack>, With<Attacking>),
	)>
) {
}
