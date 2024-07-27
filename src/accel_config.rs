//! Pointer acceleration
use crate::ffi;
use crate::AccelProfile;
use crate::DeviceConfigError;
use crate::DeviceConfigResult;
use input_sys::libinput_config_accel;

/// A handle for configuration pointer acceleration.
///
/// @warning Unlike other structs pointer acceleration configuration is
/// considered transient and <b>not</b> refcounted. Calling
/// libinput_config_accel_destroy() <b>will</b> destroy the configuration.
///
/// To configure pointer acceleration, first create a config of a desired
/// acceleration profile with libinput_config_accel_create(), then
/// configure the profile-specific acceleration properties.
///
/// In this version of libinput, this pointer acceleration configuration
/// only provides configuration for @ref LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM.
///
/// For @ref LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM use
/// @ref libinput_config_accel_set_points.
///
/// Once set up, apply the configuration to a device using
/// libinput_device_config_accel_apply(). Once applied,
/// destroy it with libinput_config_accel_destroy().
///
/// @since 1.23
#[cfg(feature = "libinput_1_26")]
pub struct AccelConfig(*mut libinput_config_accel);

#[cfg(feature = "libinput_1_26")]
impl AccelConfig {
    /// Create an acceleration configuration of a given profile.
    ///
    /// Note that in this version of libinput, only the
    /// @ref LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM profile provides configuration
    /// options. All other acceleration profiles, when applied, will merely switch
    /// the profile and reset any profile-specific options to the default values.
    ///
    /// @param profile The profile of the newly created acceleration configuration.
    ///
    /// @return The newly created acceleration configuration or NULL on error.
    ///
    /// @warning Unlike other structs pointer acceleration configuration is
    /// considered transient and <b>not</b> refcounted. Calling
    /// libinput_config_accel_destroy() <b>will</b> destroy the configuration.
    ///
    /// @see libinput_config_accel
    /// @since 1.23
    pub fn new(profile: AccelProfile) -> Self {
        let profile = match profile {
            AccelProfile::Flat => {
                ffi::libinput_config_accel_profile_LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT
            }
            AccelProfile::Adaptive => {
                ffi::libinput_config_accel_profile_LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE
            }
            AccelProfile::Custom => {
                ffi::libinput_config_accel_profile_LIBINPUT_CONFIG_ACCEL_PROFILE_CUSTOM
            }
        };

        unsafe { Self(ffi::libinput_config_accel_create(profile)) }
    }

    /// Set the points for custom acceleration
    ///
    /// # Safety
    ///
    /// The accel config could become dereferenced
    #[cfg(feature = "libinput_1_26")]
    pub unsafe fn set_points(
        &self,
        accel_type: AccelType,
        step: f64,
        npoints: usize,
        points: Vec<f64>,
    ) -> DeviceConfigResult {
        let accel_type = match accel_type {
            AccelType::Fallback => {
                input_sys::libinput_config_accel_type_LIBINPUT_ACCEL_TYPE_FALLBACK
            }
            AccelType::Motion => input_sys::libinput_config_accel_type_LIBINPUT_ACCEL_TYPE_MOTION,
            AccelType::Scroll => input_sys::libinput_config_accel_type_LIBINPUT_ACCEL_TYPE_SCROLL,
        };
        match unsafe {
            ffi::libinput_config_accel_set_points(
                self.0,
                accel_type,
                step,
                npoints,
                points.clone().as_mut_ptr(),
            )
        } {
            ffi::libinput_config_status_LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status_LIBINPUT_CONFIG_STATUS_UNSUPPORTED => {
                Err(DeviceConfigError::Unsupported)
            }
            ffi::libinput_config_status_LIBINPUT_CONFIG_STATUS_INVALID => {
                Err(DeviceConfigError::Invalid)
            }
            _ => panic!("libinput returned invalid 'libinput_config_status'"),
        }
    }

    /// Get the pointer for the acceleration config
    pub fn ptr(&self) -> *mut libinput_config_accel {
        self.0
    }
}

/// Acceleration types are categories of movement by a device that may have
/// specific acceleration functions applied. A device always supports the
/// @ref LIBINPUT_ACCEL_TYPE_MOTION type (for regular pointer motion). Other
/// types (e.g. scrolling) may be added in the future.
///
/// The special type @ref LIBINPUT_ACCEL_TYPE_FALLBACK specifies the acceleration
/// function to be moved for any movement produced by the device that does not
/// have a specific acceleration type defined.
///
/// Use to specify the acceleration function type in
/// @ref libinput_config_accel_set_points
///
/// Each device implements a subset of those types, see a list of supported
/// devices for each movement type definition.
///
/// @see LIBINPUT_ACCEL_ARG_TYPE
/// @since 1.23
pub enum AccelType {
    /// The default acceleration type used as a fallback when other
    /// acceleration types are not provided.
    Fallback,
    /// Acceleration type for regular pointer movement. This
    /// type is always supported.
    Motion,
    /// Acceleration type for scroll movement.
    /// This type is supported by mouse and touchpad.
    Scroll,
}

// /// Creates a new acceleration config
// #[cfg(feature = "libinput_1_26")]
// pub fn config_accel_create(profile: AccelProfile) -> *mut libinput_config_accel {}
