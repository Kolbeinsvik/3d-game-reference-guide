use amethyst::assets::AssetLoaderSystemData;
use amethyst::assets::Handle;
use amethyst::prelude::World;
use amethyst::prelude::WorldExt;
use amethyst::renderer::mtl::Material;
use amethyst::renderer::mtl::MaterialDefaults;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::rendy::mesh::Normal;
use amethyst::renderer::rendy::mesh::Position;
use amethyst::renderer::rendy::mesh::Tangent;
use amethyst::renderer::rendy::mesh::TexCoord;
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::shape::Shape;
use amethyst::renderer::types::Mesh;
use amethyst::renderer::types::Texture;
use amethyst::utils::application_root_dir;
use std::path::PathBuf;

pub fn generated_assets_prefabs_path() -> PathBuf {
    return application_root_dir()
        .unwrap()
        .join("generated")
        .join("assets")
        .join("prefabs");
}

pub fn load_shape_mesh(world: &mut World, shape: Shape) -> Handle<Mesh> {
    return world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            shape
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });
}

pub fn load_material(world: &mut World, albedo: Handle<Texture>) -> Handle<Material> {
    let roughness = 1.0f32 * (3 as f32 / 4.0f32);
    let metallic = 1.0f32 * (3 as f32 / 4.0f32);

    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    return world.exec(
        |(material_loader, texture_loader): (
            AssetLoaderSystemData<'_, Material>,
            AssetLoaderSystemData<'_, Texture>,
        )| {
            let metallic_roughness = texture_loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
                (),
            );

            material_loader.load_from_data(
                Material {
                    albedo,
                    metallic_roughness,
                    ..mat_defaults.clone()
                },
                (),
            )
        },
    );
}

pub fn load_color_texture(world: &mut World, color: LinSrgba) -> Handle<Texture> {
    return world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(load_from_linear_rgba(color).into(), ())
    });
}
