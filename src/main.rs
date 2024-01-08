//! This example demonstrates how to use the `Camera::viewport_to_world` method.

//use std::f32::consts::PI;

use bevy::prelude::*;
//use rand::Rng;

// Think we can do multiple startup steps and set multiple functions to be Update

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}


#[derive(Default)]
struct Avatar {
    entity: Option<Entity>,
    obj:    u8,
}

#[derive(Resource, Default)]
struct VStream {
    avatar: Avatar,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

#[derive(Default)]
struct Voxel {
    entity: Option<Entity>,
    x: f32,
    y: f32,
    z: f32,
}



fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), ground.up()) else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
}

// point -> 3d sphere and highlight it? 
// put walls up to create isometric view
// fix camera to only orbit using s curvles
// maybe make a tree for cel shading practice
// our goal is to have obj created for us, so
// develop a 3d space to stream 

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));

    // Our Grid Zero Position in a 2D Grid will be -5.0, -5.0 with ground at -0.5. Should be 
    //   intiailized at a position then stack these together easily for 3D shapes. Be able to scale down    //   voxels for higher detail items
    //
    

    // Extend commands so we can make this less clunky shit also so we can start doing 
    // things like changing their color n-stuff
    for column in 1..10 {
	println!("col {}", column);
        commands.spawn((
    	   PbrBundle {
    		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    		material: materials.add(Color::rgb_u8(124+(column as u8 *2), 144, 255).into()),
    		transform: Transform::from_xyz((column as f32 * 1.25)-6.0, 0.5, -6.0),
    		..default()
    	  },
	));
     for row in 1..10 {
	// All the 1s are being double spawned on, should start at 2 for row 
           println!("row {}", row);
           commands.spawn((
              PbrBundle {
           	mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
           	material: materials.add(Color::rgb_u8(124+(row as u8 *4), 144, 255).into()),
           	transform: Transform::from_xyz((row as f32 * 1.25)-6.0, 0.5, (column as f32 * 1.25)-6.0),
           	..default()
             },
           ));
	};
    }



   // cube
   //SpawnCube(10.0, 5.0, 10.0);

   // light
   commands.spawn(DirectionalLightBundle {
       transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
       ..default()
   });

  // camera
  commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(15.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
		..Default::default()
  });
}

// THEN DO User input on W-A-S-D 
//
// fn SpawnCube(
//     xpos: f32, 
//     ypos: f32,
//     zpos: f32,
//     mut material: ResMut<Assets<CustomMaterial>>,
// ) {
//     commands.spawn((
// 	 PbrBundle {
// 		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
// 		material: materials.add(Color::rgb_u8(124, 144, 255).into()),
// 		transform: Transform::from_xyz(xpos, ypos, zpos),
// 		..default()
// 		},
// 	));
// }
