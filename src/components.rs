use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ojas {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct TargetPosition(pub Vec3);

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct TargetDummy;

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub original_color: Color,
}

#[derive(Component)]
pub struct EclipseDebuff {
    pub timer: Timer,
    pub original_color: Color,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum MeridianHouse {
    Mars,
    Venus,
    Jupiter,
    PlayerHouse, // Your faction
}

#[derive(Component)]
pub enum Personality {
    Arrogant, // Will always attack rivals
    Timid,    // Will always run from rivals
}

#[derive(Component)]
pub struct CameraShake {
    pub timer: Timer,
    pub intensity: f32,
}

#[derive(Component)]
pub struct AIController {
    pub target: Option<Vec3>, // Where the AI currently wants to walk
    pub cooldown: Timer,
}

// Dynamic camera orbit controls component
#[derive(Component)]
pub struct GameCamera {
    pub yaw: f32,      // Orbit angle around the vertical Y axis
    pub pitch: f32,    // Vertical angle (tilt) looking down
    pub distance: f32, // Distance from the player target
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillCategory {
    SunAligned,
    MoonAligned,
    AscendantAligned,
}

#[derive(Component)]
pub struct ResonantGlow;

#[derive(Component)]
pub struct CelestialWheelCanvas {
    pub image_handle: Handle<Image>,
    pub sun_flash: f32, // remaining animation duration (max 0.3s)
    pub sun_flash_sign: Option<ZodiacSign>,
    pub sun_flash_is_resonant: bool,
    pub moon_flash: f32,
    pub moon_flash_sign: Option<ZodiacSign>,
    pub moon_flash_is_resonant: bool,
    pub ascendant_flash: f32,
    pub ascendant_flash_sign: Option<ZodiacSign>,
    pub ascendant_flash_is_resonant: bool,
}

// Projectile Component for our Fractals
#[derive(Component)]
#[allow(dead_code)]
pub struct FractalProjectile {
    pub direction: Vec3,
    pub speed: f32,
    pub damage: f32,
    pub lifespan: Timer, // To despawn it if it misses and flies off the map
    pub native_sign: ZodiacSign,
    pub category: SkillCategory,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct TessellationStrike {
    pub native_sign: ZodiacSign,
    pub category: SkillCategory,
    pub lifespan: Timer,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct EclipseField {
    pub native_sign: ZodiacSign,
    pub category: SkillCategory,
    pub lifespan: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChakraType {
    Root,         // Stability (Increases base Health)
    Sacral,       // Flow (Increases movement speed)
    SolarPlexus,  // Will (Increases maximum Ojas)
    Heart,        // Connection (Unlocks passive aura regeneration)
    Throat,       // Expression (Increases Fractal projectile damage)
    ThirdEye,     // Vision (Improves visual overlay range)
    Crown,        // Ascension (Prepares user for Kundalini awakening)
}

#[derive(Clone, Copy)]
pub struct Chakra {
    pub chakra_type: ChakraType,
    pub unblocked: bool,
    pub cost_to_unblock: f32,
}

#[derive(Component)]
pub struct ChakraTree {
    pub chakras: [Chakra; 7],
}

#[derive(Component)]
pub struct Myelin {
    pub structural_density: f32,
    pub max_density: f32,
    pub throttled: bool,
}

#[derive(Component)]
pub struct MyelinTextUi;

#[derive(Component)]
pub struct MyelinFillBarUi;

#[derive(Component)]
pub struct OjasFillMeterUi;

#[derive(Component)]
pub struct ExhaustionStatusTextUi;

#[derive(Component)]
pub struct ChakraUiMarker {
    pub index: usize,
}

#[derive(Component)]
pub struct NeuralFriction {
    pub ida_friction: f32,
    pub pingala_friction: f32,
    pub decay_constant: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZodiacSign {
    Aries, Taurus, Gemini, Cancer, Leo, Virgo, 
    Libra, Scorpio, Sagittarius, Capricorn, Aquarius, Pisces
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum House {
    First, Second, Third, Fourth, Fifth, Sixth,
    Seventh, Eighth, Ninth, Tenth, Eleventh, Twelfth
}

#[derive(Component, Debug, Clone, Copy)]
pub struct BirthData {
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CelestialProfile {
    pub sun_sign: ZodiacSign,
    pub moon_sign: ZodiacSign,
    pub ascendant_sign: ZodiacSign,
    pub house_placements: [House; 12],
    pub is_cusp: bool,
}

#[derive(Resource)]
pub struct DevModeState {
    pub enabled: bool,
    pub god_mode: bool,
    pub dummy_spawn_active: bool,
    pub tessellation_override: ZodiacSign,
    pub fractal_override: ZodiacSign,
    pub eclipse_override: ZodiacSign,
}

impl Default for DevModeState {
    fn default() -> Self {
        Self {
            enabled: false,
            god_mode: false,
            dummy_spawn_active: false,
            tessellation_override: ZodiacSign::Libra,
            fractal_override: ZodiacSign::Scorpio,
            eclipse_override: ZodiacSign::Taurus,
        }
    }
}

#[derive(Component)]
pub struct ActiveInputFocus;

#[derive(Resource)]
pub struct MovementBinds {
    pub up: KeyCode,
    pub left: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode,
}

impl Default for MovementBinds {
    fn default() -> Self {
        Self {
            up: KeyCode::KeyW,
            left: KeyCode::KeyA,
            down: KeyCode::KeyS,
            right: KeyCode::KeyD,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NadiChannel {
    Ida,
    Pingala,
    Sushumna,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ActiveChannelState {
    pub current: NadiChannel,
}

impl Default for ActiveChannelState {
    fn default() -> Self {
        Self {
            current: NadiChannel::Sushumna,
        }
    }
}

#[derive(Component)]
pub struct IdaNode;

#[derive(Component)]
pub struct PingalaNode;

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ActiveCastState {
    pub active_channel: Option<NadiChannel>,
    pub casting_skill: Option<SkillCategory>,
    pub timer: f32,
    pub lockout_tessellation: f32,
    pub lockout_fractal: f32,
    pub lockout_eclipse: f32,
    pub flash_amber_tessellation: f32,
    pub flash_amber_fractal: f32,
    pub flash_amber_eclipse: f32,
    pub ida_pulse: f32,
    pub pingala_pulse: f32,
}

#[derive(Component)]
pub struct IdaKeybindUi;

#[derive(Component)]
pub struct PingalaKeybindUi;

#[derive(Component)]
pub struct HotbarCardUi {
    pub category: SkillCategory,
}

#[derive(Component)]
pub struct HotbarProgressBarUi {
    pub category: SkillCategory,
}

#[derive(Component)]
pub struct HotbarFrictionOverlayUi {
    pub category: SkillCategory,
}

#[derive(Component)]
pub struct HotbarAmberFlashOverlayUi {
    pub category: SkillCategory,
}
