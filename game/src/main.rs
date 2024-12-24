use std::hash::DefaultHasher;
use std::ops::Deref;
use std::sync::OnceLock;

use avian3d::prelude::*;
use avian3d::prelude::{DistanceJoint, GravityScale, RigidBody};
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css::{GOLD, PURPLE};
use bevy::prelude::*;
use bevy::picking::backend::ray::RayMap;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_channel_trigger::ChannelSender;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

static EVENT_BRIDGE: OnceLock<Option<ChannelSender<MotionEvent>>> = OnceLock::new();

fn set_sender(sender: ChannelSender<MotionEvent>) {
    while EVENT_BRIDGE.set(Some(sender.clone())).is_err() {}
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
extern "C" {
    #[wasm_bindgen(js_name = publish_event)]
    fn publish_event(event: &str);
}

#[cfg(not(target_arch="wasm32"))]
fn publish_event(event: &str) {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run("meadow the box")
}

#[cfg(target_arch = "wasm32")]
fn main() {
}

#[wasm_bindgen]
pub fn send_event(event: &str) {
    let Some(sender) = EVENT_BRIDGE.get().map(Option::as_ref).flatten() else {
        return bevy::log::error!("`WebPlugin` not installed correctly (no sender found)");
    };

    if let Some((username, event)) = event.split_once(":") {
        if let Ok(position) = serde_json::from_str::<ReportedPosition>(event) {
            let username = username.to_owned();
            sender.send( MotionEvent { username, position })
        }
    }
}

#[derive(Event, Clone)]
struct MotionEvent {
    username: String,
    position: ReportedPosition,
}

#[wasm_bindgen]
pub fn run(name: &str) {
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
        .insert_resource(MyName(name.to_owned()))
        .add_systems(Startup, setup)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, update_goal_system)
        .add_systems(Update, report_position)
        .add_observer(on_motion_event);

    use bevy_channel_trigger::ChannelTriggerApp;
    let sender = app.add_channel_trigger::<MotionEvent>();
    set_sender(sender); 

    app.run();
}

#[derive(Resource)]
struct MyName(String);

#[derive(Component)]
struct Goal(String);

#[derive(Component)]
struct Box(String);

#[derive(Component)]
struct ColorMode;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
    my_name: Res<MyName>,
) {
    // ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        ColorMode,
    ));

    let MyName(name) = my_name.into_inner();

    create_box(
        name.to_owned(),
        commands.reborrow(),
        meshes,
        materials,
        images,
    );

    // cube
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

    commands.insert_resource(ReportedPosition { x: 0.0, z: 0.0 })
}

fn create_box(
    name: String,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLight::default());

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
        ))
        .id();

    let box_color = string_to_color(&name);
    let inverted_box_color = invert_color(box_color);

    commands
        .spawn((
            Node {
                // Cover the whole image
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(box_color),
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(&name),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                TextColor(inverted_box_color.into()),
            ));
        });
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,

        ..default()
    });

    let cube = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(material_handle),
            Collider::cuboid(1., 1., 1.),
            RigidBody::Dynamic,
            GravityScale(0.0),
            Transform::from_rotation(Quat::from_rotation_y(90.0)),
            LockedAxes::ROTATION_LOCKED,
            Box(name.to_owned()),
        ))
        .id();
    // goal
    let goal = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(0.0, 0.5, 0.0),
            Goal(name.to_owned()),
        ))
        .id();

    commands.spawn(
        DistanceJoint::new(goal, cube)
            .with_compliance(1.0 / 50.0)
            .with_linear_velocity_damping(10.0)
        //.with_local_anchor_2(0.5 * Vector::ONE)
        //.with_rest_length(1.5)
        //.with_compliance(1.0 / 400.0),
    );


}

#[derive(Resource, Serialize, Deserialize, Clone)]
struct ReportedPosition {
    x: f32,
    z: f32
}


fn report_position(
    mut query_goal: Query<(&Transform, &Goal)>,
    mut reported_position: ResMut<ReportedPosition>,
    my_name: Res<MyName>,
) {
    let MyName(name) = my_name.into_inner();
    if let Some((goal, _)) = query_goal.iter_mut().find(|(_, goal)| {&goal.0 == name}) {
        if goal.translation.x != reported_position.x || goal.translation.z != reported_position.z {
            let new_position = ReportedPosition {
                x: goal.translation.x,
                z: goal.translation.z,
            };
            if let Ok(json) = serde_json::to_string(&new_position) {
                publish_event(&json);
            }
            reported_position.x = new_position.x;
            reported_position.z = new_position.z;
        }
    }
}

fn update_goal(
    mut query_goal: Query<(&mut Transform, &Goal)>,
    my_name: Res<MyName>,
    ray_map: Res<RayMap>,
) -> Option<()> {
    if ray_map.iter().count() != 1 {
        return None;
    };

    let MyName(name) = my_name.into_inner();
    if let Some((mut goal, _)) = query_goal.iter_mut().find(|(_, goal)| {&goal.0 == name}) {
    let (_, ray) = ray_map.iter().next()?;
    let ground_dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::default())?;
    let ground = ray.get_point(ground_dist);


        goal.translation = ground.with_y(0.5);
    }


    Some(())
}

fn update_goal_system(
    mut query_goal: Query<(&mut Transform, &Goal)>,
    my_name: Res<MyName>,
    ray_map: Res<RayMap>,
) {
    update_goal(
        query_goal,
        my_name,
        ray_map,
    );
}

fn on_motion_event(
    trigger: Trigger<MotionEvent>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
    my_name: Res<MyName>,
    mut query_goal: Query<(&mut Transform, &Goal)>,
) {
    let MotionEvent { username, position} = trigger.event();

    // The event is for me
    let MyName(my_name) = my_name.into_inner();
    if my_name == username {
        return
    }

    // The event is for a known box
    if let Some((mut goal, _)) = query_goal.iter_mut().find(|(_, goal)| {&goal.0 == username}) {
        goal.translation.x = position.x;
        goal.translation.z = position.z;
        return
    }

    // The event is for a new box
    create_box(username.clone(), commands, meshes, materials, images)
}

pub fn string_to_color(input: &str) -> Color {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    // Hash the input string
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Use parts of the hash to generate RGB values
    let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = (hash & 0xFF) as f32 / 255.0;

    // Return a Color with RGB values and full opacity
    Color::srgba(r, g, b, 1.0)
}

fn invert_color(color: Color) -> Color {
    let Srgba {red, green, blue, alpha} = color.to_srgba();
    Color::srgba(1.0- red, 1.0 - green, 1.0 - blue, alpha)
}
