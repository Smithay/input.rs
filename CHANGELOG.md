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
