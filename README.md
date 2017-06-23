# Nitro game engine [![Crates listing](https://img.shields.io/crates/v/nitro.svg)](https://crates.io/crates/nitro) [![Build Status](https://travis-ci.org/nitro-devs/nitro-game-engine.svg?branch=master)](https://travis-ci.org/nitro-devs/nitro-game-engine/branches) [![Build Status](https://ci.appveyor.com/api/projects/status/github/nitro-devs/nitro-game-engine?branch=master&svg=true)](https://ci.appveyor.com/project/Xaeroxe/nitro-game-engine?branch=master) [![Gitter Chat](https://badges.gitter.im/nitro-game-engine/nitro.svg)](https://gitter.im/nitro-game-engine/nitro)

UPDATE: It is unlikely further work will be done in this repository, as I've decided to work on the [Amethyst](https://www.amethyst.rs/) project alternatively.  After evaluating the two code bases and philosophies I've determined that Nitro and Amethyst have the same goals, I hope any fans of this project will consider the Amethyst project as a sufficient alternative.  If anyone wants to take over the nitro repositories I'll be happy to do the transfer.

[Documentation](https://nitro-devs.github.io/nitro-game-engine/nitro/)

Nitro is a game engine built in Rust for Windows, Mac OSX, and Linux systems.

This project is ~~a Work In Progress~~ abandoned.  You can make some basic games with it as it is today, for example a mario clone.  However in some areas the API endpoints are a bit scarce and some features such as multiplayer are missing entirely.

# Features
* Very basic asset management exists.
* A 2D rendering backend powered by SDL2 is present, 3D is a long ways off at this point.
* "Immediate mode" GUI system driven by Components.
* Audio playback, including features such as volume and pausing/resuming are present.
* The nphysics physics library has been integrated into Nitro and exposed to the end user.
* Rebindable keys and input axes exist and can be queried by Components.
* The ECS is in place, allowing users to create GameObjects and extend them by attaching Components.
* Components receive messages from the engine which allow the components to respond to events as they occur.

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

* This repository has been abandoned.  If you'd like to take over further development on it (or use the nitro name itself for another project) please contact @Xaeroxe.
