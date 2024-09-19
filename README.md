# MMcP - Make Minecraft Portable
Having trouble making your Minecraft installation more portable? This tool will help you achieve that and more! Make Minecraft Portable, MMcP for short, is a simple program to manage Minecraft in a friendlier, more portable method, allowing for advance features such as custom installations (called `instance`s), custom locations and others. Originally, this project was made in Windows Batch script (sadly lost the original) as a pet project. With this rewrite, it is now aimed to become friendlier towards the larger masses.

## Features
Currently, the following features are available:
- [Complete]: Launch the (official) Minecraft Launcher (logins are managed by the Minecraft Launcher, not MMcP).
- [In-Progress]: Create, modify and remove `instance`s.
- [Planned]: Download and install community-created contents (resource packs, mods, shaders, etc.) via providers (Modrinth, CurseForge, etc.).
- [Planned]: Auto-launch Minecraft via MMcP.
- [Planned]: Export `instance`s into a compressed archive to be imported into other third-party launchers ([Prism Launcher](https://prismlauncher.org/), [Modrinth App](https://modrinth.com/app), [CurseForge App](https://www.curseforge.com/download/app), etc.).
- And many more! (Send ideas by creating a feature request, pull/merge request, etc.).

## Build Instructions
Prerequisites:
- Rust (`rustc` version 1.81.0 during build. Older version untested.)
- Git (Optional, but very helpful if you want to contribute code.)

---
Use the green `Code` button around the middle top-right, choose your appropriate download method.


Alternatively, with `git`, `clone` the project to a working directory, then move into it.
```
git clone https://github.com/NickGenghar/make-minecraft-portable.git
cd make-minecraft-portable
```
Once inside the directory, run `cargo build` to start building the project. Alternatively, use `cargo run` to build and run the project soon after.

## Usage
The program does not receives command-line input. Instead, it provides what's known as a _Terminal User Interface_, TUI for short. Using this TUI, the user then selects the options provided. As of now, certain interactions are not implemented, instead, it will be implemented in the near future.