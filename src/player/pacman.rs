use crate::common::Position;
use bevy::prelude::*;
use bevy::sprite::entity::SpriteComponents;

#[derive(Clone, Debug)]
pub struct Pacman {
    health: u8,
    size: Size,
    position: Position,
    pub speed: f32,
}

impl Pacman {
    pub const SIZE: f32 = 30.0;
    pub const MAX_LIVES: u8 = 3;
    pub const MOVEMENT_VELOCITY: f32 = 200.0;
    const MAIN_COLOR: [f32; 4] = [1.0, 0.5, 1.0, 1.0];
}

impl Pacman {
    /// Attach pacman to bevy command chain with rendered sprite sheet.
    pub fn attach(commands: &mut Commands, materials: ResMut<Assets<ColorMaterial>>) {
        let pacman = Pacman::default();

        commands.spawn(pacman.render(materials)).with(pacman);
    }

    pub fn damage(&mut self) {
        self.health -= 1;

        if self.health == 0 {
            println!("Game over!");
        } else {
            println!("Lives: {}", self.health);
        }
    }

    pub fn render(&self, mut materials: ResMut<Assets<ColorMaterial>>) -> SpriteComponents {
        SpriteComponents {
            sprite: Sprite::new(Vec2::new(self.size.width, self.size.height)),
            transform: Transform::from_translation(Vec3::new(0.0, -self.size.height, 1.0)),
            material: materials.add(Color::from(Self::MAIN_COLOR).into()),
            ..Default::default()
        }
    }
}

impl Default for Pacman {
    fn default() -> Self {
        Pacman {
            health: Pacman::MAX_LIVES,
            size: Size::new(Self::SIZE, Self::SIZE),
            position: Position::init(0, 0),
            speed: Self::MOVEMENT_VELOCITY,
        }
    }
}
