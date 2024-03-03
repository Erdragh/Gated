// Dirty workaround while bevy's UI system is still fucky: https://github.com/bevyengine/bevy/issues/3570

use bevy::{
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, ResMut, Resource},
    },
    math::Vec3Swizzles,
    transform::components::GlobalTransform,
    ui::{Interaction, Node},
    window::{PrimaryWindow, Window},
};

#[derive(Resource)]
pub struct IsPointerCaptured(pub bool);

#[derive(Component)]
pub struct NoPointerCapture;

pub fn setup_pointer_capture(mut commands: Commands) {
    commands.insert_resource(IsPointerCaptured(false));
}

pub fn is_pointer_captured_system(
    window: Query<&Window, With<PrimaryWindow>>,
    mut is_pointer_captured: ResMut<IsPointerCaptured>,
    node_query: Query<(&Node, &GlobalTransform, &Interaction), Without<NoPointerCapture>>,
) {
    is_pointer_captured.0 = window
        .get_single()
        .ok()
        .and_then(|window| window.cursor_position())
        .map(|pointer_position| {
            node_query
                .iter()
                .filter(|(_, _, interaction)| matches!(interaction, Interaction::Hovered | Interaction::Pressed))
                .any(|(node, transform, ..)| {
                    let node_position = transform.translation().xy();
                    let half_size = 0.5 * node.size();
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    (min.x..max.x).contains(&pointer_position.x)
                        && (min.y..max.y).contains(&pointer_position.y)
                })
        })
        .unwrap_or(false);
}
