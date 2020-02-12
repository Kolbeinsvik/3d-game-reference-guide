use amethyst::core::components::Transform;
use amethyst::core::math::Vector3;
use amethyst::ecs::world::Entity;
use amethyst::prelude::*;
use amethyst::prelude::*;
use amethyst::renderer::camera::Camera;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::shape::Shape;
use amethyst::window::ScreenDimensions;

pub fn spawn_camera_first_person(world: &mut World) -> Entity {
    let transform = {
        let mut transform = Transform::default();
        transform.prepend_translation_y(1.0);
        transform.prepend_translation_z(8.0);
        transform.prepend_translation_x(8.0);
        transform.prepend_rotation_y_axis(45.0f32.to_radians());
        transform
    };

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    return world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}

pub fn spawn_camera_third_person(world: &mut World) -> Entity {
    let transform = {
        let mut transform = Transform::default();
        transform.prepend_translation_y(10.0);
        transform.prepend_translation_z(10.0);
        transform.prepend_translation_x(10.0);
        transform.append_rotation_x_axis(-45.0f32.to_radians());
        transform.prepend_rotation_y_axis(45.0f32.to_radians());
        transform
    };

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    return world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}

pub fn spawn_camera_isometric(world: &mut World) -> Entity {
    let transform = {
        let mut transform = Transform::default();
        transform.prepend_translation_y(10.0);
        transform.prepend_translation_z(10.0);
        transform.prepend_translation_x(10.0);
        transform.append_rotation_x_axis(-45.0f32.to_radians());
        transform.prepend_rotation_y_axis(45.0f32.to_radians());
        transform
    };

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    return world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}

pub fn spawn_camera_top_down(world: &mut World) -> Entity {
    let transform = {
        let mut transform = Transform::default();
        transform.prepend_translation_y(10.0);
        transform.prepend_translation_z(10.0);
        transform.prepend_translation_x(10.0);
        transform.append_rotation_x_axis(-45.0f32.to_radians());
        transform.prepend_rotation_y_axis(45.0f32.to_radians());
        transform
    };

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    return world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}

pub fn spawn_camera_side_scroller(world: &mut World) -> Entity {
    let transform = {
        let mut transform = Transform::default();
        transform.prepend_translation_y(10.0);
        transform.prepend_translation_z(10.0);
        transform.prepend_translation_x(10.0);
        transform.append_rotation_x_axis(-45.0f32.to_radians());
        transform.prepend_rotation_y_axis(45.0f32.to_radians());
        transform
    };

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    return world
        .create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}
