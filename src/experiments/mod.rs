use bevy::prelude::*;
use bevy_proto::prelude::*;

const PREFAB_BLASTER: &str = "blaster";

pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_models.run_if(prototypes_ready(["Cube", "small_box"]).and_then(run_once())),
        )
        .add_systems(
            Update,
            spawn_with_reload.run_if(prototypes_ready(["small_box", PREFAB_BLASTER])),
        );
    }
}

#[allow(dead_code)]
pub fn spawn_models(mut commands: ProtoCommands) {
    println!("spawning stuff");
    // commands.spawn("Sphere");
    // commands.spawn("Cube");
    // commands.spawn("Monkey");
    // commands.spawn("small_box");
    commands
        .spawn("small_box")
        .entity_commands()
        .insert(Transform::from_xyz(0.0, 10.0, 0.0));
}

pub fn spawn_with_reload(
    mut commands: ProtoCommands,
    keyboard_input: Res<Input<KeyCode>>,
    mut previous: Local<Option<Entity>>,
    mut proto_asset_events: EventReader<ProtoAssetEvent>,
    transform_query: Query<&Transform>,
) {
    let prefab = PREFAB_BLASTER;
    let position = Vec3::new(0.0, 5.0, 0.0);
    if previous.is_none() || keyboard_input.just_pressed(KeyCode::Space) {
        *previous = Some(
            commands
                .spawn(prefab)
                .entity_commands()
                .insert(Transform::from_translation(position))
                .id(),
        );
    }

    if previous.is_none() {
        return;
    };
    // Listen for changes:
    for proto_asset_event in proto_asset_events.iter() {
        match proto_asset_event {
            // Only trigger a re-insert of the prototype when modified and if IDs match
            ProtoAssetEvent::Modified { id, .. } if id == prefab => {
                let previous_entity = previous.unwrap();
                println!("previous {:?}", previous);

                if transform_query.get(previous_entity).is_err() {
                    continue;
                }
                let transform = match transform_query.get(previous_entity) {
                    Ok(t) => *t,
                    Err(_) => Transform::from_translation(position),
                };
                // commands
                //     .entity(previous.unwrap())
                //     .insert(prefab)
                //     .entity_commands()
                //     .insert(Sleeping {
                //         sleeping: false,
                //         ..default()
                //     })
                // .insert(Transform::from_matrix(transform.compute_matrix()));
                // ;
                commands
                    .entity(previous_entity)
                    .entity_commands()
                    .despawn_recursive();
                *previous = Some(
                    commands
                        .spawn(prefab)
                        .entity_commands()
                        .insert(Transform::from_matrix(transform.compute_matrix()))
                        .id(),
                );
            }
            _ => {}
        }

        // Note: We could also have checked using the helper method:
        // if proto_asset_event.is_modified("ReloadableSprite") { ... }
    }
}
