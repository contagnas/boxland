
use std::f32::consts::{FRAC_PI_2, PI};
use bevy::{
    color::palettes::css,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    math::{vec3, NormedVectorSpace},
    picking::backend::ray::RayMap,
    prelude::*,
};
const LASER_SPEED: f32 = 0.03;
const DELTA: f32 = 0.001;

fn accel_to_goal(starting_pos: Vec3, starting_velocity: Vec3, goal_pos: Vec3, a_max: f32) -> Vec3 {
    // Compute the relative position vector to the goal
    let r = goal_pos - starting_pos;
    let r_norm = r.length();

    // If we're already very close to the goal and nearly stopped, no acceleration is needed
    if r_norm < 1e-3 && starting_velocity.length() < 1e-3 {
        return Vec3::ZERO;
    }

    // Decompose velocity into components parallel and perpendicular to the direction of r
    let v = starting_velocity;
    let r_unit = r / r_norm; // Unit vector toward the goal
    let v_parallel = r_unit * v.dot(r_unit); // Parallel component of velocity
    let v_perp = v - v_parallel; // Perpendicular component of velocity

    // Phase 1: Cancel perpendicular velocity to stop orbiting
    if v_perp.length() > 1e-3 {
        // Acceleration to cancel perpendicular velocity
        let a_perp = -v_perp;
        // Clamp acceleration magnitude to a_max
        return a_perp.clamp_length_max(a_max);
    }

    // Phase 2: Move toward the goal, transitioning to deceleration when necessary
    let v_norm = v.length();
    let stopping_distance = v_norm * v_norm / (2.0 * a_max);

    if stopping_distance >= r_norm {
        // Decelerate to stop at the goal
        let a_decel = -r_unit * a_max;
        return a_decel;
    } else {
        // Accelerate toward the goal
        let a_accel = r_unit * a_max;
        return a_accel;
    }
}


#[derive(Component, Default)]
struct Kinematic {
    velocity: Vec3,
    acceleration: Vec3,
}

fn process_kinematics(
    mut objects: Query<(&mut Transform, &mut Kinematic)>,
    time: Res<Time>
) {
    for (mut t, mut k) in &mut objects {
        let new_v = k.velocity + (k.acceleration * time.delta_secs());
        k.velocity = new_v;

        t.translation += k.velocity * time.delta_secs();
    }
}

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Chaser;


fn update_goal(
    mut query_goal: Query<&mut Transform, With<Goal>>,
    ray_map: Res<RayMap>,
) -> Option<()> {

    let iter = ray_map.iter();
    if ray_map.iter().count() != 1 { return None };

    let (_, ray) = ray_map.iter().next()?;
    let ground_dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::default())?;
    let ground = ray.get_point(ground_dist);

    for mut transform in &mut query_goal {
        transform.translation = ground;
    }

    Some(())
}
fn update_goal_system(
    query_goal: Query<&mut Transform, With<Goal>>,
    ray_map: Res<RayMap>
) {
    update_goal(query_goal, ray_map);
}

fn chase(
    goal: Query<&Transform, With<Goal>>,
    mut chaser: Query<(&Transform, &mut Kinematic), With<Chaser>>,
    time: Res<Time>
) {
        println!("0");
    if let Some(goal) = goal.iter().next() {
        println!("1");
        for (chaser_t, mut chaser_k) in &mut chaser {
        println!("2");
            //let new_accel = accel_to_goal(chaser_t.translation, chaser_k.velocity, goal.translation, 10.0);
            //let new_accel = (((goal.translation - chaser_t.translation.with_y(0.0)) * 4.0)).with_y(0.0);

            println!("chaser velo: {}", chaser_k.velocity);
            chaser_k.acceleration = dbg!(compute_acceleration2(
                chaser_t.translation,
                chaser_k.velocity,
                goal.translation,
                10.0
            ));
        }
    };
}

fn compute_acceleration2(p: Vec3, v: Vec3, pg: Vec3, a_max: f32) -> Vec3 {
    let direction = pg - p;


    let d_unit = direction.normalize_or_zero();

    // if we are already at the goal, just decelerate
    if direction.length() < DELTA || d_unit == Vec3::ZERO {
        return -v.normalize_or_zero() * a_max;
    }

    let v_par = d_unit * v.dot(d_unit);
    let v_perp = v - v_par;
    let v_perp_speed = v_perp.length();
    let v_perp_dir = v_perp.normalize_or_zero();

    // Just fight the perp velocity
    if v_perp_speed > 1.0 && v_perp_dir != Vec3::ZERO {
        return -v_perp_dir * a_max;
    }

    // We are going straight at (or away from) the goal, charge
    // let v_par_speed = v_par.length();
    // let v_par_dir = v_par.normalize_or_zero();
    let speed_squared = v.dot(v);
    let goal_distance = direction.length();

    if v.dot(direction) > 90.0 && speed_squared > 2.0 * a_max * goal_distance {
        // pump the brakes
        return -v.normalize_or_zero() * a_max;
    }

    // accelerate towards the goal
    return d_unit * a_max;

}

fn compute_acceleration(
    p: Vec3,       // Current position
    v: Vec3,       // Current velocity
    pg: Vec3,      // Goal position
    a_max: f32,            // Maximum acceleration
) -> Vec3 {
    println!("current position: {p}");
    println!("current velo: {v}");
    println!("goal position: {pg}");

    // Compute the direction to the goal
    let direction_to_goal = (&(pg - p)).normalize();

    dbg!(direction_to_goal);

    // Decompose velocity into parallel and perpendicular components
    let v_parallel = direction_to_goal * v.dot(direction_to_goal);
    let v_perpendicular = v - v_parallel;

    dbg!((v_parallel, v_perpendicular));

    // Compute the distance to the goal
    let distance_to_goal = (&(pg - p)).length();

    dbg!(distance_to_goal);
    let speed = v.length();
    dbg!(speed);
    if speed * speed >= 2.0 * a_max * distance_to_goal {
        // Deceleration needed.
        // Oppose velocity:
        let mut a = -v;
        
        // If there's a perpendicular component, also oppose it
        let v_perp_norm = v_perpendicular.length();
        if v_perp_norm > 1e-12 {
            // Add a small portion to remove perpendicular velocity
            a += -v_perpendicular; 
        }
        
        // Normalize and scale to max acceleration
        let a_norm = a.length();
        if a_norm > 1e-12 {
            a = a * (a_max / a_norm);
        } else {
            // In a rare case where v is zero or negligible, just don't accelerate
            a = Vec3::ZERO;
        }

        return a;
    }

    // Check if we should start decelerating
    if speed * speed >= 2.0 * a_max * distance_to_goal {
        dbg!("decell!!!");
        // Deceleration phase: Oppose current velocity
        return -v.normalize() * a_max;
    }

    // Acceleration phase: Combine goal-directed acceleration and perpendicular cancellation
    let mut acceleration = -v_perpendicular.normalize_or_zero() * v_perpendicular.length();
    acceleration += direction_to_goal * a_max;

    // Ensure the magnitude of the acceleration does not exceed a_max
    if acceleration.length() > a_max {
        acceleration = acceleration.normalize() * a_max;
    }

    acceleration
}
    

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
    // cube
    let mut kine = Kinematic::default();
    kine.velocity = Vec3::X;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).insert((Chaser, kine));
    // goal sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(-1.0, 0.0, 0.0),
    )).insert(Goal).observe(on_click_print_hello);
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
}

fn on_click_print_hello(click: Trigger<Pointer<Click>>) {
    println!("{} was clicked!", click.entity());
}

fn on_move_print_move(mv: Trigger<Pointer<Move>>) {
    let pos = mv.hit.position;
    //println!("hit at {pos:?}");
}
use bevy::window::PrimaryWindow;

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor_options.visible = false;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, cursor_grab))
        .add_systems(Update, update_goal_system)
        .add_systems(Update, process_kinematics)
        .add_systems(Update, chase)
        .add_plugins(MeshPickingPlugin)
        .run();
}
