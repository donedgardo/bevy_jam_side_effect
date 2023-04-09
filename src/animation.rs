use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity, Query, Res, TextureAtlasSprite, Time};
use benimator::FrameRate;

pub fn add_blinking_animation(commands: &mut Commands, ship: Entity) {
    let animation = Animation(benimator::Animation::from_indices(
        0..=1,
        FrameRate::from_fps(7.0),
    ));
    commands.entity(ship).insert(animation);
}

pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut state, mut texture, animation) in query.iter_mut() {
        state.update(animation, time.delta());
        texture.index = state.frame_index();
    }
}

#[derive(Component, Deref)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(benimator::State);
