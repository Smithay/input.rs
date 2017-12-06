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
