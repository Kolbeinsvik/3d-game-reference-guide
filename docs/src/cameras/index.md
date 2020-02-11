# Different camera types for 3D games

There are different camera types used in 3D (or 2.5D) games. 

Camera grouping

- Locked camera angle
- Free moving camera angles

Named camera angles

- First-person camera angle
- Third-person camera angle
- Top-down camera angle
- Side-scrolling camera angle
- Isometric camera angle


How each do rotation, movement, move the entity they are attached to, 
rotation of the entity they are attached to. 

## First-person camera

The first-person camera is placed at the "head" position of the entity they are attached to. 
For people or creature objects this is easy to imagine.
Rotation and tilting are changed based on input, 
most often from the computer mouse or the right analogue stick on a gamepad,
and that updates the "head" entity. 
The camera angle is anchored and fixed to that entity and so will follow it, 
making it seem like the camera rotated or tilted. 
The tilt angle is often clamped so that the camera cannot be tilted to the top and behind itself. 
However, the rotation are most often not clamped, so that players can rotate back and forth without issues. 

**Vehicle first-person camera**

There is a subset of first-person cameras that are used for "vehicle" entities.
For example a camera inside the driver side of a car, the cockpit of an aircraft, or a gun turret. 
They act like regular first-person cameras that has their rotation clamped as well as their tilt. 
Though, they often have a separate key trigger that will allow the player to watch behind them.

**Hand-held items**

A big part of the first-person camera is that items are often shown permanently in the camera view. 
The most prominent example is probably weapons, which is a mainstay of the  "first-person shooter" genre.
However, it can apply to any item, and can have different properties. 
A lantern or torch, for example can be held in view and light up the environment around the player.
Many games also have an eating mechanic where the normal item in view is dropped out of it and 
a food item is brought up and "eaten" before the main item is brought back up in view. 

**Models in view**

Full body realism is not required when using first-person cameras. 
This means that many games only render part of the 3D models in the first-person view.
Often only the arms and potential item is rendered, with the occasional other feature,
such as a foot used as a melee mechanic.
This means that players can look directly down and not see the feet of their character. 

Some games also use a "bobbing" mechanic to make the movement feel more "natural", 
though some players experience motion sickness from it and so its often a feature that can be turned off. 
Another mechanic is to make an item in view follow tilt and rotations with a slight lag and then snap,
to make it feel less static. 
It can also improve the "identity" of characters as making this animation looser may
convey a more sloppy, while a more snappy animation may convey more control or skill.     


## Third-person camera

## Relevant Wikipedia pages about game cameras

- https://en.wikipedia.org/wiki/Virtual_camera_system
- https://en.wikipedia.org/wiki/First-person_(video_games)
