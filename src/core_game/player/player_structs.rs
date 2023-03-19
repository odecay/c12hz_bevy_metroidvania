use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct PlayerGraphics;

#[derive(Component, Clone)]
pub struct Vel {
	pub x: f32,
	pub y: f32,
	pub dir: f32,
}

#[derive(Component, Clone)]
pub struct MoveSpeed {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct Grav {
	pub speed: f32,
	pub max_speed: f32,
	pub slide_speed: f32,
	pub strength: f32,
	pub counter: i32,
}

#[derive(Component, Clone)]
pub struct CameraVariables {
	pub new_ground_height: f32,
}

#[derive(Component, Clone)]
pub struct PlayerInput {
	pub pressing_jump: bool,
	pub just_pressed_jump: bool,
	pub pressing_left: bool,
	pub just_pressed_left: bool,
	pub pressing_right: bool,
	pub just_pressed_right: bool,
	pub pressing_dodge: bool,
	pub just_pressed_dodge: bool,
	pub pressing_skill1: bool,
	pub just_pressed_skill1: bool,
	pub pressing_skill2: bool,
	pub just_pressed_skill2: bool,
	pub pressing_skill3: bool,
	pub just_pressed_skill3: bool,
	pub pressing_skill4: bool,
	pub just_pressed_skill4: bool,
	pub pressing_up: bool,
	pub just_pressed_up: bool,
}

#[derive(Component, Clone)]
pub struct PlayerCasts {
	pub basic_up: bool,
	pub basic_down: bool,
	pub basic_left: bool,
	pub basic_right: bool,
	pub directional_x: bool,
	pub big_left: bool,
	pub big_right: bool,
	pub wallslide_anim_up: bool,
	pub wallslide_anim_down: bool,
	pub enemy_penetration: bool,
	pub nearby_enemies: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerMoveState {
	Idle,
	Run,
	Jump,
	Fall,
	WallSlide,
	Whirlwind,
	DashForward,
	DashDown45,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerDirectionState {
	Left,
	Right,
	None,
}

#[derive(Component, Clone)]
pub struct StealthMode {
	pub active: bool,
	pub duration: u32,
	pub counter: u32,
	pub speed_x: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAnimationState {
	Idle,
	Run,
	Jump,
	Fall,
	WallSlide,
	WhirlwindHammer,
	WhirlwindSword,
	RunIdle,
	IdleWhirlwind,
	WhirlwindIdle,
	FallIdle,
	MeleeBasicHammer,
	MeleeBasicSword,
	RangedBasicBowForward,
	RangedBasicBowUp,
	RangedBasicGunsForward,
	RangedBasicGunsUp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAttackState {
	MeleeBasicHammer,
	MeleeBasicSword,
	RangedBasicBowForward,
	RangedBasicBowUp,
	RangedBasicGunsForward,
	RangedBasicGunsUp,
	WhirlwindHammer,
	WhirlwindSword,
	DashForward,
	DashDown45,
	None,
}

#[derive(Debug)]
pub struct PlayerState {
	pub movement: PlayerMoveState,
	pub direction: PlayerDirectionState,
	pub animation: PlayerAnimationState,
	pub attack: PlayerAttackState,
}

impl Default for PlayerState {
	fn default() -> Self {
		Self {
			movement: PlayerMoveState::Idle,
			direction: PlayerDirectionState::Right,
			animation: PlayerAnimationState::Idle,
			attack: PlayerAttackState::None,
		}
	}
}

#[derive(Component, Debug)]
pub struct PlayerStateBuffer {
	//rename to current
	pub old: PlayerState,
	//rename to next
	pub new: PlayerState,
}

// various variables used for determining player state.
// for example a frame counter variable to determine how long a jump state can last
#[derive(Component)]
pub struct PlayerStateVariables {
	pub jump_frame_counter: u32,
	pub jumps_remaining: u32,
	pub runidle_counter: u32,
	pub idlewhirl_counter: u32,
	pub whirlidle_counter: u32,
	pub fallidle_counter: u32,
	pub walljump_counter: u32,
	pub dash_counter: u32,
	pub dash_cooldown: u32,
	pub dash_strike_counter: u32,
	pub dash_strike_cooldown: u32,
	pub actively_colliding: bool,
	pub penetrating_enemy: bool,
	pub sprite_flipped: bool,
}

#[derive(Component)]
pub struct WallKick {
	pub timer: u32,
	pub wall_direction: f32,
	pub full_wallslide: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum DamageKind {
	Simple,
	Whirlwind,
	DashStrike,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub enum PassiveSkills {
	Adrenaline,
	Endorphin,
	Dopamine,
	Tranquility,
	Focus,
	Trickery,
	Surprise,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub enum PlayerWeaponMelee {
	Hammer,
	Sword,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub enum PlayerWeaponRanged {
	Bow,
	Guns,
	None,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub struct PlayerWeapons {
	pub melee: PlayerWeaponMelee,
	pub ranged: PlayerWeaponRanged,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Inspectable)]
pub struct PlayerPassives {
	pub p1: PassiveSkills,
	pub p2: PassiveSkills,
	pub p3: PassiveSkills,
	pub p4: PassiveSkills,
	pub p5: PassiveSkills,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerDamage {
	pub dealt: bool,
	pub applied: bool,
	pub targets: Vec<Entity>,
	pub location: Vec3,
	pub kind: DamageKind,
	pub kind_mult: f32,
	pub weapon_dmg: f32,
	pub crit: bool,
	pub value: f32,
	pub direction: f32,
}

#[derive(Component, Clone)]
pub struct PlayerDamageStats {
	pub hammer_damage: f32,
	pub sword_damage: f32,
	pub bow_damage: f32,
	pub guns_damage: f32,
	pub simple_mult: f32,
	pub whirlwind_mult: f32,
	pub dashstrike_mult: f32,
}

#[derive(Component, Clone)]
pub struct SoundParams {
	pub handle: Handle<AudioSource>,
	pub volume: f32,
	pub looping: bool,
}

#[derive(Component, Clone)]
pub struct AnimationParams {
	pub atlas: Handle<TextureAtlas>,
	pub start: usize,
	pub restart: usize,
	pub end: usize,
	pub perfect_transitions: bool,
}

#[derive(Resource)]
pub struct MyPlayerSounds {
	pub iceimpct1: SoundParams,
	pub iceimpct2: SoundParams,
	pub iceimpct3: SoundParams,
	pub iceimpct4: SoundParams,
	pub hmrimpct1: SoundParams,
	pub hmrimpct2: SoundParams,
	pub hmrimpct3: SoundParams,
	pub hmrimpct4: SoundParams,
	pub hmrimpct5: SoundParams,
	pub basshit1: SoundParams,
	pub basshit2: SoundParams,
	pub step1: SoundParams,
	pub step2: SoundParams,
	pub step3: SoundParams,
	pub step4: SoundParams,
	pub step5: SoundParams,
	pub step6: SoundParams,
	pub step7: SoundParams,
	pub step8: SoundParams,
	pub step9: SoundParams,
	pub step10: SoundParams,
	pub step11: SoundParams,
	pub step12: SoundParams,
	pub step13: SoundParams,
}

// the jumpd and falld are for separate forward jump and fall animations that will be added soon
#[derive(Resource)]
pub struct MyPlayerAnimations {
	pub run: AnimationParams,
	pub idle: AnimationParams,
	pub jump: AnimationParams,
	pub fall: AnimationParams,
	pub jumpd: AnimationParams,
	pub falld: AnimationParams,
	pub slide: AnimationParams,
	pub whirl: AnimationParams,
	pub runidle: AnimationParams,
	pub idlewhirl: AnimationParams,
	pub whirlidle: AnimationParams,
	pub fallidle: AnimationParams,
	pub mbs1: AnimationParams,
	pub mbs2: AnimationParams,
	pub mbh1: AnimationParams,
	pub mbh2: AnimationParams,
	pub rbbf: AnimationParams,
	pub rbbu: AnimationParams,
	pub rbgf1: AnimationParams,
	pub rbgf2: AnimationParams,
	pub rbgu1: AnimationParams,
	pub rbgu2: AnimationParams,
}

#[derive(Component, Clone)]
pub struct TimeDivisions {
	pub two: u32,
	pub three: u32,
	pub four: u32,
	pub five: u32,
	pub six: u32,
	pub seven: u32,
	pub eight: u32,
	pub nine: u32,
	pub ten: u32,
	pub eleven: u32,
	pub twelve: u32,
	pub thirteen: u32,
	pub fourteen: u32,
	pub fifteen: u32,
	pub reset: bool,
}

#[derive(Component, Clone)]
pub struct RandomValues {
	pub one: u32,
	pub two: u32,
	pub three: u32,
	pub four: u32,
	pub five: u32,
	pub six: u32,
	pub seven: u32,
	pub eight: u32,
	pub nine: u32,
	pub ten: u32,
	pub eleven: u32,
	pub twelve: u32,
}

#[derive(Clone, Copy, Debug, Reflect, Inspectable, PartialEq)]
pub enum Ability {
	MeleeBasic,
	RangedBasic,
	Whirlwind,
	DashForward,
	DashDown45,
	Stealth,
	None,
}

#[derive(Component, Debug, Reflect, Inspectable)]
pub struct PlayerAbilities {
	pub ability1: Ability,
	pub ability2: Ability,
	pub ability3: Ability,
}
