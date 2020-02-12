use crate::helpers::asset_helpers::{load_color_texture, load_material, load_shape_mesh};
use amethyst::core::components::Transform;
use amethyst::core::math::Vector3;
use amethyst::ecs::world::Entity;
use amethyst::prelude::*;
use amethyst::renderer::camera::Camera;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::shape::Shape;
use amethyst::window::ScreenDimensions;

pub fn spawn_camera(world: &mut World) -> Entity {
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

pub fn spawn_plane(world: &mut World) {
    let mesh = load_shape_mesh(world, Shape::Plane(None));
    let material = {
        let green_color = LinSrgba::new(0.2, 1.0, 0.2, 1.0);
        let albedo = load_color_texture(world, green_color);
        load_material(world, albedo)
    };

    let transform = {
        let mut transform = Transform::default();
        transform.append_rotation_x_axis(-90.0f32.to_radians());
        transform.set_scale(Vector3::new(10.0, 10.0, 10.0));
        transform
    };

    world
        .create_entity()
        .with(transform)
        .with(material)
        .with(mesh)
        .build();
}

pub fn spawn_cube(world: &mut World) {
    let mesh = load_shape_mesh(world, Shape::Cube);
    let material = {
        let green_color = LinSrgba::new(1.0, 0.2, 0.2, 1.0);
        let albedo = load_color_texture(world, green_color);
        load_material(world, albedo)
    };

    let transform = {
        let mut transform = Transform::default();
        transform.set_translation_xyz(4.0, 0.5, 0.0);
        transform
    };

    world
        .create_entity()
        .with(transform)
        .with(material)
        .with(mesh)
        .build();
}

pub fn spawn_sphere(world: &mut World) {
    let mesh = load_shape_mesh(world, Shape::Sphere(10, 10));
    let material = {
        let green_color = LinSrgba::new(0.2, 0.2, 1.0, 1.0);
        let albedo = load_color_texture(world, green_color);
        load_material(world, albedo)
    };

    let transform = {
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.5, 4.0);
        transform
    };

    world
        .create_entity()
        .with(transform)
        .with(material)
        .with(mesh)
        .build();
}

pub fn spawn_cone(world: &mut World) {
    let mesh = load_shape_mesh(world, Shape::Cone(10));
    let material = {
        let green_color = LinSrgba::new(0.5, 0.5, 0.5, 1.0);
        let albedo = load_color_texture(world, green_color);
        load_material(world, albedo)
    };

    let transform = {
        let mut transform = Transform::default();
        transform.set_translation_xyz(-4.0, 1.0, 0.0);
        transform.prepend_rotation_x_axis(-90f32.to_radians());
        transform
    };

    world
        .create_entity()
        .with(transform)
        .with(material)
        .with(mesh)
        .build();
}
