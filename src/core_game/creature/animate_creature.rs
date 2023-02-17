use crate::core_game::creature::creature_structs::AnimationParams;
use crate::core_game::creature::creature_structs::CreatureGraphics;
use crate::core_game::creature::creature_structs::TimeDivisions;
use bevy::prelude::*;

pub fn animate_creature(
	mut q: Query<
		(
			&mut TextureAtlasSprite,
			&mut Handle<TextureAtlas>,
			&AnimationParams,
			&TimeDivisions,
		),
		With<CreatureGraphics>,
	>
) {
	for (mut sprite, mut handle, params, time) in q.iter_mut() {
		// are we still doing the same animation?
		// check if the handle on the entity matches the one in params

		// yes we are doing the same animation

		// check if it's time for the next frame

		// perfect_transitions means game wait till current frame finishes before moving to next animation

		if params.perfect_transitions == true {
			if time.five == 1 {
				if params.atlas == *handle {
					sprite.index += 1;
					if sprite.index >= params.end {
						if params.looping {
							sprite.index = params.restart;
						} else {
							sprite.index = params.end - 1;
						}
					}
				} else {
					// we need to change to a different animation
					*handle = params.atlas.clone();
					sprite.index = params.start;
				}
			}
		} else if params.perfect_transitions == false {
			if params.atlas == *handle {
				if time.five == 1 {
					sprite.index += 1;
					if sprite.index >= params.end {
						if params.looping {
							sprite.index = params.restart;
						} else {
							sprite.index = params.end - 1;
						}
					}
				}
			}
			if params.atlas != *handle {
				// we need to change to a different animation
				*handle = params.atlas.clone();
				sprite.index = params.start;
			}
		}
	}
}
