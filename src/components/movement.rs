use bevy::prelude::*;

/// Applies player movement to an entity
pub struct Movement {
    /// Normalized direction
    pub direction: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub rotation_yaw: Quat,
    pub rotation_pitch: Quat,
    pub speed: f32,
    pub acceleration: f32,
    pub wants_to_jump: bool,
    pub sensitivity: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            direction: Default::default(),
            pitch: 90f32,
            yaw: Default::default(),
            roll: Default::default(),
            rotation_yaw: Default::default(),
            rotation_pitch: Default::default(),
            speed: 8f32,
            acceleration: 100f32,
            wants_to_jump: false,
            sensitivity: 0.05,
        }
    }
}

impl Movement {
    #[inline(always)]
    pub fn facing(&self) -> Quat {
        self.rotation_yaw
    }

    /// Set the movement's direction normalizing
    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction.normalize_or_zero();
    }

    /// Set direction without normalizing
    pub fn set_direction_unchecked(&mut self, direction: Vec3) {
        self.direction = direction;
    }

    /// Get a reference to the movement's direction.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
