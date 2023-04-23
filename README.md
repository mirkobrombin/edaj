# **THIS PROJECT HAS BEEN MIGRATED TO GITLAB**
Please make any PRs here, rather than to the GitHub: https://git.getcryst.al/crystal

# REPOSITORIES HERE ARE **OUTDATED**, GO TO THE LINK  BELOW
# ---- > https://git.getcryst.al/crystal < ----

<p align="center">
  <a href="https://github.com/crystal-linux/edaj/">
    <img src="https://getcryst.al/site/assets/other/logo.png" alt="Logo" width="150" height="150">
  </a>
</p>
<h2 align="center">edaJ</h2>
<p align="center">
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License">
    <a href="https://github/crystal-linux/edaj"><img alt="GitHub isses" src="https://img.shields.io/github/issues-raw/crystal-linux/edaj"></a>
    <a href="https://github/crystal-linux/edaj"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/edaj"></a><br>
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
    <a href="https://github.com/axtloss"><img src="https://img.shields.io/badge/Maintainer-@axtloss-brightgreen" alt=The maintainer of this repository" href="https://github.com/axtloss"></a>
    <a href="https://fosstodon.org/@crystal_linux"><img alt="Mastodon Follow" src="https://img.shields.io/mastodon/follow/108618426259408142?domain=https%3A%2F%2Ffosstodon.org">
    <a href="https://twitter.com/crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux"></a>
</p>

<p align="center">Edaj is an installer backend for crystal linux.</p>

## Backend usage

### autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
edaj partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
edaj partition auto /dev/nvmen0
```

### install base packages
```sh
edaj install-base
```

### install bootloader
```sh
# install as efi with esp being /boot/efi
edaj bootloader grub-efi /boot/efi

# install as legacy on /dev/sda
edaj bootloader grub-legacy /dev/sda
```

### generate fstab
```sh
edaj genfstab
```

### configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
edaj locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### configure network settings
```sh
# set the hostname to getcryst.al with ipv6 disabled
edaj networking getcryst.al 

# set the hostname to getcryst.al with ipv6 enabled
edaj networking getcryst.al --ipv6
```

### setup zramd
```sh
# install and enable zramd
edaj zramd
```

### configure users
```sh
# make a new user called nonRootHaver, without sudo, easytohack as the password and bash as the default shell
edaj users new-user nonRootHaver easytohack bash

# make a user called rootHaver, with sudo, omgsosuperhardtohack as the password and fish as the default shell
edaj users new-user rootHaver omgsuperhardtohack fish --hasroot
```

### set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
edaj users root-password muchSecurity,veryHardToHack
```

### install a desktop environment
```sh
# install onyx
edaj desktops onyx

# install gnome
edaj desktops gnome
```

### setup timeshift
```sh
edaj setup-timeshift
```

### setup flatpak
```sh
edaj flatpak
```

### debug logging

debug messages:
```sh
edaj -v
```

traces:
```sh
edaj -vv
```

## How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`

## Non-secret Secret
echo "EDAJ_UWU=true" >> ~/.zshrc <br>
echo "EDAJ_UWU=true" >> ~/.bashrc <br>
set -Ux EDAJ_UWU true <br>
<br>
if you want to have your log and crash output be “cute”
