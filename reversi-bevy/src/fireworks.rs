use bevy::prelude::*;
use rand::random;

#[derive(Component)]
pub struct Firework {
    pub timer: Timer,
}

#[derive(Component)]
pub struct FireworkParticle {
    pub velocity: Vec2,
    pub timer: Timer,
}

pub struct FireworkPlugin;

impl Plugin for FireworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_fireworks, update_firework_particles).run_if(in_state(crate::GameState::GameOverScreen)),
        );
    }
}

fn update_fireworks(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Firework)>,
) {
    // Note: On utilise crate::GameOverRoot pour marquer les entités à nettoyer plus tard
    // Il faut s'assurer que GameOverRoot est public dans main.rs ou accessible.
    
    for (entity, mut firework) in &mut query {
        firework.timer.tick(time.delta());
        if firework.timer.just_finished() {
            // Créer une explosion de particules simples
            let x = (random::<f32>() - 0.5) * 400.0;
            let y = (random::<f32>() - 0.5) * 400.0;
            let color = Color::hsv(random::<f32>() * 360.0, 1.0, 1.0);

            for _ in 0..20 {
                let velocity = Vec2::new(
                    (random::<f32>() - 0.5) * 300.0,
                    (random::<f32>() - 0.5) * 300.0 + 100.0,
                );
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(Vec2::splat(5.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 20.0),
                    crate::GameOverRoot,
                    FireworkParticle {
                        velocity,
                        timer: Timer::from_seconds(1.0, TimerMode::Once),
                    },
                ));
            }
            commands.entity(entity).despawn();
        }
    }
}

fn update_firework_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FireworkParticle)>,
) {
    for (entity, mut transform, mut particle) in &mut query {
        particle.timer.tick(time.delta());
        if particle.timer.just_finished() {
            commands.entity(entity).despawn();
        } else {
            let delta = time.delta_secs();
            transform.translation.x += particle.velocity.x * delta;
            transform.translation.y += particle.velocity.y * delta;
            particle.velocity.y -= 100.0 * delta; // Gravité
        }
    }
}
