use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_planets)
        .run();
}

#[derive(Component)]
struct Planet {
    orbit_speed: f32,
    rotation_speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(40.0, 40.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 環境光を追加
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // 太陽（自己発光するマテリアル）
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(2.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            emissive: Color::rgb(1.0, 0.9, 0.5) * 2.0, // 太陽の発光
            ..default()
        }),
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    });

    let mut rng = rand::thread_rng();
    let num_stars = 1000;
    
    for _ in 0..num_stars {
        let distance = rng.gen_range(5.0..50.0);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        let y = rng.gen_range(-5.0..5.0);
        
        let size = rng.gen_range(0.1..0.3);
        let orbit_speed = rng.gen_range(0.1..0.5);
        let rotation_speed = rng.gen_range(0.5..2.0);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(size)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.384, 0.533, 0.471),
                    emissive: Color::WHITE * 0.5,
                    ..default()
                }),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            Planet {
                orbit_speed,
                rotation_speed,
            },
        ));
    }

    // メインライト（太陽光）
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    // 補助ライト（全体的な明るさを調整）
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: false,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, 20.0, 10.0),
        ..default()
    });
}

fn rotate_planets(mut query: Query<(&mut Transform, &Planet)>, time: Res<Time>) {
    for (mut transform, planet) in query.iter_mut() {
        // 公転
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(planet.orbit_speed * time.delta_seconds()),
        );
        // 自転
        transform.rotate_local_y(planet.rotation_speed * time.delta_seconds());
    }
}

