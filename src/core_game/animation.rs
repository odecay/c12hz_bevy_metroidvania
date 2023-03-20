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

//in future might want to fork benimator and add metadata to Frames struct
#[derive(Component, Deref)]
pub struct Animation(benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);

fn animate(
	time: Res<Time>,
	// animations: Res<Assets<Animation>>,
	mut query: Query<(
		&mut AnimationState,
		&mut TextureAtlasSprite,
		// &Handle<Animation>,
		&Animation,
	)>,
) {
	for (mut state, mut sprite, animation) in query.iter_mut() {
		state.update(animation, time.delta());
		sprite.index = state.frame_index();
	}
}

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
