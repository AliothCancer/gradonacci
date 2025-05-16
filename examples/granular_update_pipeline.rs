use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    // Rest of the Bevy setup
    app.configure_sets(
        Update,
        (
            AppSystems::GameplayLogic,
            AppSystems::Despawn,
            AppSystems::UpdateUi,
        )
            .chain(),
    );

    app.add_systems(
        Update,
        despawn_when_surpass_lower_bound.in_set(AppSystems::Despawn),
    );
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Handle gameplay logic.
    GameplayLogic,
    /// Despawn entities.
    Despawn,
    /// Update UI nodes.
    UpdateUi,
}

fn despawn_when_surpass_lower_bound(
    //event: Trigger<SurpassedBoundaries>,
    mut commands: Commands,
    q: Query<(Entity, &Transform)>,
) {
    //let entity = event.target();
    //let transform = transform.get(entity).unwrap();
    q.iter().for_each(|(entity, transform)| {
        if transform.translation.y < -150.0 {
            commands.entity(entity).despawn();
        }
    });
}
