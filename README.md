<p align="center">
    <img src="https://github.com/o7Moon/Crustacean/raw/main/Crustacean.png">
</p>

---

Crustacean is a simple and easy to use mod installer for Crab Game. 

# Usage
## Installation
 - Grab the build from [Releases](https://github.com/o7Moon/Crustacean/releases), you'll want the `.exe` for Windows and the extensionless file for Linux.
 - Run it. If you are on Windows there may be a "Windows protected your PC" popup. Unfortunately there isn't much I can do to remove this, so you'll have to click "More Info" and then "Run anyway". You will only have to do this once.
 - If your game installation is in steam, you shouldn't need to change the install path. Just select any of the official mods that you want to install by default and hit "Install". This will install both BepInEx (the modloader) and the mods you select. You can install any future mods easily by clicking that mod's install link.
 - If you want to move the executable to a different folder, run it again after moving it and then close it. The custom uri handler for quick mod installation relies on knowing the location of the exe, so you need to re-run the installer to update that path.

## For Mod Developers:
 - if you want to create an install link for your mod, the format looks like this: `crustacean://installMod/link/to/your/mod.dll`.
 - unfortunately github hates fun and wont let you embed these links in markdown files. You can however have a link that redirects to the crustacean url, with something like tinyurl.

# FAQ:
### I tried using this on linux, but mods still dont seem to run.
You need to run in proton and enable the wine dll override for BepInEx: (https://docs.bepinex.dev/articles/advanced/proton_wine.html). In the future the installer will probably do this for you.

### Why "Crustacean"?
Crab Game + Rust = _

### Why Rust?
Mostly for the crab pun, but other reasons include:
 - easy cross-compiling
 - the `egui` Immediate-Mode gui library
 - it was a good project for starting out with rust

### Why does this use a custom `system_uri` rather than the official crate?
The official crate is 5 years old and no longer compiles. There is a fork with updated deps which does successfully compile to linux, but it still wouldn't compile to windows, so I made a fork of my own and fixed the breaking changes.