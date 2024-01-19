## 0.9.0

- Update to bitflags 2.x and udev 0.8
- Don't require generated bindings for specific OS/architecture to build

## 0.8.3

- Fixed bitflags test for config entries (see https://github.com/Smithay/input.rs/pull/54, thanks @ids1024)
- Fixed use of struct\_name in Debug impls (see https://github.com/Smithay/input.rs/pull/53, thanks @ids1024)

## 0.8.2

- Added missing `GestureEndEvent` implementation for `GestureHoldEvent`

## 0.8.1

- Fix missing feature flags for libinput 1.21

## 0.8.0 (yanked)

- Use safe-io types
- Add libinput 1.21 support
- Add (untested) pre-generated FreeBSD bindings
- Upgrade udev dependency to 0.7
- Fix udev-device-unref memory corruption

## 0.7.0

- Add libinput 1.19 support

## 0.6.0

- Upgrade udev dependency to 0.6.0

## 0.5.0

- Upgrade udev dependency to 0.4.0
- Update input-sys to 1.15, allowing usage of newer libinput versions
  - The exposed libinput functions are controlled via features!
  - **Breaking:** To remain compatible with the oldest available libinput version disable the default feature set
  - The new default is to always expose all functions

## 0.4.0

- Upgrade input-sys to libinput 1.9.0
- Optionally integrate with `udev` crate
- Replace `libinput_interface` with `LibinputInterface` trait
- Remove broken `Userdata` api

## 0.3.0

- Use bitflags 1.0 to represent bitflags

## 0.2.1

- Bugfixes
  - id_vendor() was incorrectly returning the product id
  - config_accel_profiles() was returning inverted results ([Flat,Adaptive] for devices that support neither, [] for devices supporting both)


## 0.2.0

- Remove invalid `Clone` implementations

## 0.1.0

- Initial release
