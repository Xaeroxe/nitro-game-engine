# Nitro game engine [![Build Status](https://travis-ci.org/Xaeroxe/nitro_game_engine.svg?branch=master)](https://travis-ci.org/Xaeroxe/nitro_game_engine)

Nitro is a game engine built in Rust for Windows, Mac OSX, and Linux systems.

The Nitro project aims to be a complete game development solution, complete with a scene editor, asset import,
and all the features we've come to expect from commercial offerings such as Unity or Unreal Engine 4.
We're not there yet.  This project is very much so a Work In Progress.  You could create a game with Nitro as it exists today
but you would be better served by several other engines as of this moment.

# Progress thus far
* Very basic asset management exists.
* A 2D rendering backend powered by SDL2 is present, 3D is a long ways off at this point.
* Audio playback, including features such as volume and pausing/resuming are present.
* The nphysics physics library has been integrated into Nitro and exposed to the end user.
* Rebindable keys and input axes exist and can be queried by Components.
* The ECS is in place, allowing users to create GameObjects and extend them by attaching Components.
* Components receive messages from the engine which allow the components to respond to events as they occur.

# Objectives for the near future
* Improve documentation.
* Make "cargo build" work out of the box without requiring the user to setup our C dependencies.
* Implement an "Immediate mode" GUI system driven by Components.
* Facilitate easy setup of multiplayer games.
* Improve asset import (Current asset system is just based on the filesystem.)
* Create a scene editor
* Implement a GUI editor that allows users to visually design a GUI WYSIWYG style.

# Objectives for the distant future
* Extend the engine to work in 3D
* Implement importing of existing 3D animations
* "Locomotion" type system to allow animations to respond to the game environment.
* Implement a lighting system that can compete with State of the Art commercial engines.
* Anything beyond this is too far out to plan at this time.

# Building the engine
Nitro has three major C based dependencies that will need to be setup on your dev machine:
* SDL2
* SDL2_image
* Alsa (on Linux systems only)

Alsa is configured to import via pkg_config if available however you will need to setup the SDL2 and SDL2_image libraries
and the linkage for them in your project.  The rust-SDL2 project has some great tutorials for how to do that here:
https://github.com/AngryLawyer/rust-sdl2/blob/daa27e6d3596f62ff930bc8e1dc70f150ee41f92/README.md

# What's this rodio folder?
Nitro depends on a custom version of the rodio project which was necessary in order to get the DjIdle Component message
working.  This is not the cleanest solution so the audio backend may be overhauled at some point but for now there are other
features I'm more eager to get to.
