//! This is a very slightly adapted copy of `generate_custom_mesh.rs` from the bevy examples.

use bevy::{
    asset::{AssetMetaCheck, RenderAssetUsages, UnapprovedPathMode},
    input::ButtonInput,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use dioxus::prelude::{GlobalSignal, ReadableExt, Signal};

// Event for resetting rotation from Dioxus
#[derive(Message, Clone)]
pub struct ResetRotationEvent;

// Global signal for reset rotation commands
static RESET_ROTATION_SIGNAL: GlobalSignal<bool> = Signal::global(|| false);

pub fn send_reset_rotation() {
    *RESET_ROTATION_SIGNAL.write() = true;
}

// Define a "marker" component to mark the custom mesh. Marker components are often used in Bevy for
// filtering entities in queries with `With`, they're usually not queried directly since they don't
// contain information within them.
#[derive(Component)]
struct CustomUV;

pub async fn run_bevy_app() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy-canvas".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // Where Bevy looks for assets, relative to your project root:
                    // file_path: "assets".into(),
                    // processed_file_path: "assets".into(),
                    // How strict to be about paths outside that root:
                    meta_check: AssetMetaCheck::Never,
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                }),
        )
        .add_message::<ResetRotationEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
        .add_systems(Update, handle_reset_signal)
        .add_systems(Update, reset_rotation_handler)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Import the custom texture. For some reason, if I invoke dioxus's `asset!()` macro inline
    // here, I get an error about how manganis isn't available.
    let texture = crate::ARRAY_TEXTURE.to_string();
    let texture = texture.trim_start_matches("/assets/").to_string();
    info!(?texture, "trying to load asset");
    let custom_texture_handle: Handle<Image> = asset_server.load(texture);
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            ..default()
        })),
        CustomUV,
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_and_light_transform));

    // Light up the scene.
    commands.spawn((PointLight::default(), camera_and_light_transform));
}

// System to receive input from the user,
// check out examples/input/ for more examples about user input.
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mesh_query: Query<&Mesh3d, With<CustomUV>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<CustomUV>>,
    mut reset_events: MessageWriter<ResetRotationEvent>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mesh_handle = mesh_query.single().expect("Query not successful");
        let mesh = meshes.get_mut(mesh_handle).unwrap();
        toggle_texture(mesh);
    }
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_secs() / 1.2);
        }
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        reset_events.write(ResetRotationEvent);
    }
}

// System to handle reset commands from Dioxus global signal
fn handle_reset_signal(mut reset_events: MessageWriter<ResetRotationEvent>) {
    // Check if the global signal has been triggered. This is reading from a Dioxus global signal on
    // every iteration of the game loop which maybe is not a great way to do things? I don't know.
    if *RESET_ROTATION_SIGNAL.read() {
        *RESET_ROTATION_SIGNAL.write() = false; // Reset the signal
        reset_events.write(ResetRotationEvent);
    }
}

// System to handle reset rotation events
fn reset_rotation_handler(
    mut events: MessageReader<ResetRotationEvent>,
    mut query: Query<&mut Transform, With<CustomUV>>,
) {
    for _ in events.read() {
        for mut transform in &mut query {
            transform.look_to(Vec3::NEG_Z, Vec3::Y);
        }
    }
}

#[rustfmt::skip]
fn create_cube_mesh() -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    //
    // The first two defined triangles look like this (marked with the vertex indices,
    // and the axis), when looking down at the top (+y) of the cube:
    //   -Z
    //   ^
    // 0---1
    // |  /|
    // | / | -> +X
    // |/  |
    // 3---2
    //
    // The right face's (+x) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 10--11
    // |  /|
    // | / | -> -Z
    // |/  |
    // 9---8
    //
    // The back face's (+z) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 17--18
    // |\  |
    // | \ | -> +X
    // |  \|
    // 16--19
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}

// Function that changes the UV mapping of the mesh, to apply the other texture.
fn toggle_texture(mesh_to_change: &mut Mesh) {
    // Get a mutable reference to the values of the UV attribute, so we can iterate over it.
    let uv_attribute = mesh_to_change.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    // The format of the UV coordinates should be Float32x2.
    let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
        panic!("Unexpected vertex format, expected Float32x2.");
    };

    // Iterate over the UV coordinates, and change them as we want.
    for uv_coord in uv_attribute.iter_mut() {
        // If the UV coordinate points to the upper, "dirt+grass" part of the texture...
        if (uv_coord[1] + 0.5) < 1.0 {
            // ... point to the equivalent lower, "sand+water" part instead,
            uv_coord[1] += 0.5;
        } else {
            // else, point back to the upper, "dirt+grass" part.
            uv_coord[1] -= 0.5;
        }
    }
}
