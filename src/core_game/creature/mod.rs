use bevy::prelude::*;

// use iyes_loopless::prelude::*;

pub mod apply_creature_state;
pub mod creature_get_damage;
pub mod creature_movement;
pub mod set_creature_state;

pub mod animate_creature;
pub mod creature_casts;
pub mod creature_death;
pub mod creature_reset_color;
pub mod creature_structs;
pub mod creature_switch_animation;
pub mod creature_time_divisions;
pub mod periodic_spawn;
pub mod setup_creature;
pub mod transfer_data_creature;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		// app.add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_casts::creature_casts.label("c_casts"),
		// )
		app.add_systems(
			(
				creature_casts::creature_casts,
				set_creature_state::set_creature_state,
				apply_creature_state::apply_creature_state,
				creature_movement::creature_movement,
				//maybe need to add more ordering constraints
				creature_reset_color::creature_reset_color,
				creature_get_damage::creature_get_damage,
				//maybe need to add more ordering constraints
				transfer_data_creature::transfer_data_creature,
				creature_switch_animation::creature_switch_animation,
				creature_time_divisions::creature_time_divisions,
				animate_creature::animate_creature,
				creature_death::creature_death,
			)
				.chain()
				.in_schedule(CoreSchedule::FixedUpdate),
		);
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	set_creature_state::set_creature_state
		// 		.label("set_c_state")
		// 		.after("c_casts"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	apply_creature_state::apply_creature_state
		// 		.label("apply_c_state")
		// 		.after("set_c_state"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_movement::creature_movement
		// 		.label("move")
		// 		.after("apply_c_state"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_reset_color::creature_reset_color.before("get_damage"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_get_damage::creature_get_damage
		// 		.label("get_damage")
		// 		.after("deal_damage")
		// 		.after("move"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	transfer_data_creature::transfer_data_creature.after("move"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_switch_animation::creature_switch_animation
		// 		.after("move")
		// 		.label("c_switch_anim"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_time_divisions::creature_time_divisions
		// 		.label("c_time")
		// 		.after("c_switch_anim"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	animate_creature::animate_creature
		// 		.label("c_animate")
		// 		.after("c_time"),
		// )
		// .add_fixed_timestep_system(
		// 	"my_fixed",
		// 	0,
		// 	creature_death::creature_death
		// 		.after("get_damage")
		// 		.after("c_animate"),
		// );
	}
}
