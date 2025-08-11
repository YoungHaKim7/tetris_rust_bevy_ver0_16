#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
struct MyComponent {
    value: f32,
}

#[derive(Resource)]
struct MyResource {
    value: f32,
}

fn hello_world(query: Query<&MyComponent>, resource: Res<MyResource>) {
    let component = query.iter().next().unwrap();
    let _ = component.value;
    let _ = resource.value;
    let _ = resource.into_inner().value;
}

fn spawn_component(mut commands: Commands) {
    commands.spawn(MyComponent { value: 10.0 });
}

#[test]
fn simple_ecs_test() {
    let mut app = App::new();
    app.insert_resource(MyResource { value: 5.0 })
        .add_systems(Startup, spawn_component)
        .add_systems(Update, hello_world);

    // Drive a couple of update frames instead of .run()
    app.finish();
    app.cleanup();
    app.update();
}
