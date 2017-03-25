# Nitro game engine [![Crates listing](https://img.shields.io/crates/v/nitro.svg)](https://crates.io/crates/nitro) [![Build Status](https://travis-ci.org/nitro-devs/nitro-game-engine.svg?branch=master)](https://travis-ci.org/nitro-devs/nitro-game-engine/branches) [![Build Status](https://ci.appveyor.com/api/projects/status/github/nitro-devs/nitro-game-engine?branch=master&svg=true)](https://ci.appveyor.com/project/Xaeroxe/nitro-game-engine?branch=master) [![Gitter Chat](https://badges.gitter.im/nitro-game-engine/nitro.svg)](https://gitter.im/nitro-game-engine/nitro)

Nitro is a game engine built in Rust for Windows, Mac OSX, and Linux systems.

The Nitro project aims to be a complete game development solution, including a scene editor, asset import,
and all the features we've come to expect from commercial offerings such as Unity or Unreal Engine 4.
We're not there yet.  This project is a Work In Progress.  You can make some basic games with it as it is today, for example a mario clone.  However in some areas the API endpoints are a bit scarce and some features such as multiplayer are missing entirely.

# Progress thus far
* Very basic asset management exists.
* A 2D rendering backend powered by SDL2 is present, 3D is a long ways off at this point.
* "Immediate mode" GUI system driven by Components.
* Audio playback, including features such as volume and pausing/resuming are present.
* The nphysics physics library has been integrated into Nitro and exposed to the end user.
* Rebindable keys and input axes exist and can be queried by Components.
* The ECS is in place, allowing users to create GameObjects and extend them by attaching Components.
* Components receive messages from the engine which allow the components to respond to events as they occur.

# Objectives for the near future
* Improve documentation.
* Add layer masking support.
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
* SDL2_mixer

If you're on Windows please refer to the
[nitro-example-project](https://github.com/nitro-devs/nitro-example-project)
You will need the dll folder, and make sure to add build.rs as
a build script.

If you are on Linux or OSX, you will need to install these dependencies.  Here's some commands to help:

If you have homebrew on OSX you can install these using:

```
brew update
brew install sdl2 sdl2_image
brew install sdl2_mixer --with-libvorbis
```

If you're on Ubuntu 14.04 or greater these commands will install the dependencies for you:

```
sudo apt-get -qq update
sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev
```

# Contributing

* We try and keep the issues list on this project up to date with any "requests for help" we might have.  If you don't know what to start working on you can look there.
* If you spot issues please submit them.
* Small improvements can be made via PR without prior communication.  You're welcome to make larger PRs without prior communication as well, but be aware they may be rejected wholesale if they don't align with the project goals.  To avoid this, open up an issue for discussing the changes before you put too much work into them.
