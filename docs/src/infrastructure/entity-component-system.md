# Entity-Component-System (ECS)

ECS is a pattern for splitting data and behavior based on different use cases.
Alternative patterns include object-oriented and actor model.

**World**:
The world is a wrapper that ties the entities, components, and systems together.

**Entity**:
In its simplest form entities are just `id`s.

**Component**:
Components are pure data structures that contain no logic or behavior.
They always refer to entities.

**Systems**:
Systems loop over sets of components, and can read and write to them.

**Resources**:
Global data structures.

## Simplified example

**Resources**

- Map
- Inputs

**Components**

- Position(x, y, z)
- PlayerMovement
- NpcMovement
- RandomLaughing(cooldownTime)

Components that does not contain any data on their own are often
called "tag components" and is used to limit the scope when looping over components.
`PlayerMovement` and `NpcMovement` are both tag components in this simplified example.

**Entities with components**

Player

- Position
- PlayerMovement

Everyday regular rock

- Position
- RandomLaughing

Normal NPC

- Position
- NpcMovement

Other NPC

- Position
- NpcMovement
- RandomLaughing

**Systems**

Some systems are very generic and some are very spesific.
Here are some simplified ones for the entities and components we have defined.

PlayerMovementSystem

1. Read inputs (for example `WASD`)
1. Loop over the components: `Position` and `PlayerMovement`
1. Update the `Position` data based on the user inputs

NpcMovementSystem

1. Loop over the components: `Position` and `NpcMovement`
1. Get a random action (for example move forward or turn)
1. Update the `Position` data based on that action

LaughSystem

1. Loop over the components: `RandomLaughing` and `Position`
1. Check the `Map` resource
   - If the position does not match an odd numbered map tile,
     skip to the next element
   - Otherwise, continue with the next step
1. Check the cooldown time on the component
   - If zero, play laugh sound track and reset timer
   - If not zero, decrease the timer by a tick

**Summary**

In this simplified example every system is interested in entities
with the `Position` component.
However, the reason they want to know the position differ.
The `PlayerMovementSystem` and `NpcMovementSystem` both updates the component
but has different ways to update it.
The `LaughSystem` on the other hand only reads the the `Position` data.
It only want the information to know if the entity should play a laugh sound.

The ECS approach makes it very easy to extend a game with new features
as new components and systems can be added or changed without having to change the existing ones.
It can also improve readability and maintenance as systems can easily be
scoped to do "one thing" (separation of concern).
