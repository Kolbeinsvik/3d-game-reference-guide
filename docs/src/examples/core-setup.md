# Core setup example

This will set up the minimal things required to run a 3d game with Amethyst and Rust.

- Creating a project
- Adding amethyst dependencies
- Rendering a basic 3D world with
  - A flat plane
  - A couple of 3D shapes on that plane
  - A camera with fixed position and angle looking down on the objects
- Basic ui with an fps counter
- Generating prefabs

![Core setup example](/images/examples/core-setup-hero.png)

The full code example can be found in the `code-examples/core-setup-example/` folder.
At the end the file structure will look like the following.

```
core-setup-example/
├─ assets/font/sqaure.ttf
├─ generated/assets/prefabs/ui/ui.ron
├─ src/
│  ├─ bin/prefabs.rs
│  ├─ entities/mod.rs
│  ├─ helpers/
│  │  ├─ mod.rs
│  │  └─ asset_helpers.rs
│  ├─ prefabs/
│  │  ├─ ui/
│  │  │  └─ build_ui_prefabs.rs
│  │  └─ mod.rs
│  ├─ states/mod.rs
│  ├─ systems/
│  │  ├─ mod.rs
│  │  └─ fps_counter_ui_system.rs
│  └─ lib.rs
│  └─ main.rs
├─ target/
├─ Cargo.lock
└─ Cargo.toml
```

## Create project

Create (or modify) the `Cargo.toml` file and add the `Amethyst` dependencies.
You can use Cargo to create the project with the initial files.

```bash
cargo new core-setup-example
```

Cargo.toml

```toml
[package]
name = "core-setup-example"
version = "0.1.0"
authors = ["Debuglines"]
edition = "2018"

[dependencies.amethyst]
version = "0.13.2"
default-features = false
features = ["vulkan"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
ron = "0.5.1"
```

We turn `default-features` off for now to improve compile times.

## Main game function

We set up the game config in the `main.rs` file. 

We use a few of the included bundles for the core setup. 

- TransformBundle
- RenderingBundle, with plugins
  - RenderToWindow
  - RenderFlat3D
  - RenderSkybox
  - RenderUi
- InputBundle::<StringBindings>
- UiBundle::<StringBindings>
- FpsCounterBundle

We also need our own system to update the fps counter on the ui.

- FpsCounterUiSystem


## A simple game state

We only set up a single state for the core setup, 
and we only need to implement the `SimpleState`. 
It does not need anything more complex 
as there are only a few things we want it to handle. 

- Spawn the plane and the objects on it
- Spawn a camera
- Set up the ui
- Handle quitting if the `esc` button is pressed

Only the quit action is handled directly in the state,
while the rest is delegated to separate functions. 

## Entities / spawners

We have a few spawners and they follow the same general pattern.

1. Initialize components
1. Create entity with the builder from the world object
1. Add the components
1. Build the entity

Pseudo code

```rust
pub fn spawn_object(world: &mut World) {
    let a = A::new();
    let b = B::new();
    
    world
        .create_entity()
        .with(a)
        .with(b)
        .build();
}
```

For the object we wan to spawn we need to add three main components.

- Transform
- Mesh
- Material

The `Transform` sets the position and rotation of the entity in the game world.

The `Mesh` sets the shape of the entity. 
We use the built in shape for a plane, cube, sphere and cone. 

The `Material` sets how the mesh is displayed.
We use a built in material with a built in texture. 

The `Mesh`, the `Material` and the `Texture` are assets. 
Amethyst provides loaders that will load assets based on data you send in
and we use some very simple ones. 
We also set up a couple of helper functions for our use case, 
which is located in the `asset_helpers.rs` file. 


## UI and fps counter

The fps counter works by combining the Amethyst built-in `FpsCounterBundle`
and our small `FpsCounterUiSystem`. 
Amethyst also provide example for setting this up. 
The bundle does all of the heavy lifting, 
while the system only update the visual part through the ui.
The only ui we have in this example is a simple ui label widget,
and these widgets are included with Amethyst. 

In this example we create the prefab data in the code, serialize it and save it to disk. 
The actual prefabs are then loaded with the `UiCreator`. 
The gist of this is fairly simple and fit a small function in the `prefabs/ui/build_ui_prefbs.rs` file.
The `prefabs` bin entry is responsible for building the ui files.  


## Prefabs

Prefabs in its simplicity is just text files (or binary)
in a format that Amethyst can load and handle as assets. 

The spawners, for example, could have been stored instead as prefabs
and loaded in as assets instead of creating them directly in the code. 
The benefit to doing this is that the compiler will help us 
with any errors we make. 

The main benefit with the prefab asset approach is that we can
make any number of changes to the content of the prefabs without having to 
recompile the codebase. 
This can speed up the development when making small changes to tweak the setup.
The other benefit is that its a lot easier to work with Amethyst ui code
when using prefabs than spawning it directly from the code. 

However, we do not want to lose the benefit the compiler gives us
so we set up a function that creates the prefab data in code and serialize that.
Instead of creating these prefabs every time we start the game
we add another bin (`src/bin/prefabs.rs`) that we can run separately. 
Wen using multiple bins like this we also need to create a `lib.rs` file
that gives us access to the other parts of the code we need. 

An example is available below for the `lib.rs` file. 

```rust
pub mod prefabs;
pub mod helpers;
```       
