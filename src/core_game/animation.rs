use bevy::prelude::*;

// use bevy::time::Timer;
// use iyes_loopless::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_systems((animate, animation_flip));
	}
}

// #[derive(Component, Clone)]
// AnimationMeta
// pub struct AnimationParams {
// 	pub atlas: Handle<TextureAtlas>,
// 	pub start: usize,
// 	pub restart: usize,
// 	pub end: usize,
// 	pub looping: bool,
// 	pub perfect_transitions: bool,
// }

#[derive(Component, Clone)]
pub enum Facing {
	Left,
	Right,
}

impl From<Facing> for f32 {
	fn from(facing: Facing) -> Self {
		match facing {
			Facing::Left => -1.0,
			Facing::Right => 1.0,
		}
	}
}

#[derive(Component, Deref)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

fn animate(
	time: Res<Time>,
	mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
	for (mut state, mut sprite, animation) in query.iter_mut() {
		state.update(animation, time.delta());
		sprite.index = state.frame_index();
	}
}

//animation state hydration?

// fn switch_animation_on_state_transition(mut query: Query<(&mut AnimationState, &mut Animation)>) {}
//
//want to attach animation metadata struct to each entity as component,
//check those to run animations, load the metadata from yaml?

fn animation_flip(mut query: Query<(&mut TextureAtlasSprite, &Facing)>) {
	for (mut sprite, facing) in query.iter_mut() {
		match facing {
			Facing::Left => sprite.flip_x = false,
			Facing::Right => sprite.flip_x = true,
		}
	}
}

// pub struct AnimationTimer(Timer);

// pub struct FrameMeta {}
