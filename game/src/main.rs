use std::sync::OnceLock;

use avian3d::prelude::*;
use avian3d::prelude::{DistanceJoint, GravityScale, RigidBody};
use bevy::prelude::*;
use bevy::picking::backend::ray::RayMap;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_channel_trigger::ChannelSender;

use wasm_bindgen::prelude::*;

static EVENT_BRIDGE: OnceLock<Option<ChannelSender<WebEvent>>> = OnceLock::new();

fn set_sender(sender: ChannelSender<WebEvent>) {
    while EVENT_BRIDGE.set(Some(sender.clone())).is_err() {}
}

#[wasm_bindgen]
pub fn send_event(event: &str) {
    let Some(sender) = EVENT_BRIDGE.get().map(Option::as_ref).flatten() else {
        return bevy::log::error!("`WebPlugin` not installed correctly (no sender found)");
    };
    match event {
        "light" => sender.send(WebEvent::SetLightMode),
        "dark" => sender.send(WebEvent::SetDarkMode),
        _ => { warn!("Bad event: {event}") }
    }
}

fn main() {
    let window_descriptor = Window {
        title: "Mouse".to_string(),
        canvas: Some("#game".to_string()),
        fit_canvas_to_parent: true,
        ..default()
    };
    let window_plugin = WindowPlugin {
        primary_window: Some(window_descriptor),
        ..default()
    };

    let mut app = App::new();

    let app = app
        .add_plugins((DefaultPlugins.set(window_plugin), PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, update_goal_system)
        .add_observer(on_light_mode)
        .add_observer(on_web_event)
        .add_observer(on_dark_mode);

    use bevy_channel_trigger::ChannelTriggerApp;
    let sender = app.add_channel_trigger::<WebEvent>();
    set_sender(sender); 

    app.run();
}

#[derive(Event, Clone)]
enum WebEvent {
    SetLightMode,
    SetDarkMode,
}

#[derive(Event)]
struct SetLightMode;

#[derive(Event)]
struct SetDarkMode;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct ColorMode;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        ColorMode,
    ));
    // cube
    let cube = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Collider::cuboid(1., 1., 1.),
            RigidBody::Dynamic,
            GravityScale(0.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
            LockedAxes::ROTATION_LOCKED,
        ))
        .id();
    // goal sphere
    let goal = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
            RigidBody::Static,
            //Collider::cuboid(1., 1., 1.),
            Transform::from_xyz(-1.0, 0.0, 0.0),
            Goal,
        ))
        .id();
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(
        DistanceJoint::new(goal, cube)
            .with_compliance(1.0 / 50.0)
            .with_linear_velocity_damping(10.0), //.with_local_anchor_2(0.5 * Vector::ONE)
        //.with_rest_length(1.5)
        //.with_compliance(1.0 / 400.0),
    );
}

fn update_goal(
    mut query_goal: Query<&mut Transform, With<Goal>>,
    ray_map: Res<RayMap>,
) -> Option<()> {
    if ray_map.iter().count() != 1 {
        return None;
    };

    let (_, ray) = ray_map.iter().next()?;
    let ground_dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::default())?;
    let ground = ray.get_point(ground_dist);

    for mut transform in &mut query_goal {
        transform.translation = ground;
    }

    Some(())
}

fn update_goal_system(query_goal: Query<&mut Transform, With<Goal>>, ray_map: Res<RayMap>) {
    update_goal(query_goal, ray_map);
}

fn on_web_event(
    trigger: Trigger<WebEvent>,
    material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<ColorMode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let color = match trigger.event() {
        WebEvent::SetLightMode => Color::WHITE,
        WebEvent::SetDarkMode => Color::BLACK,
    };

    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = color; 
        }
    }
}

fn on_dark_mode(
    _trigger: Trigger<SetDarkMode>,
    material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<ColorMode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = Color::BLACK; 
        }
    }
}

fn on_light_mode(
    _trigger: Trigger<SetLightMode>,
    material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<ColorMode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = Color::WHITE; 
        }
    }
}
