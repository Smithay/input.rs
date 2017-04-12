

use {AsRaw, Device, Event, FromRaw, Userdata, ffi};

use libc;
use std::{mem, ptr};
use std::ffi::CString;
use std::io::{Error as IoError, Result as IoResult};
use std::iter::Iterator;
use std::os::unix::io::RawFd;

/// libinput does not open file descriptors to devices directly,
/// instead `open_restricted` and `close_restricted` are called for
/// each path that must be opened.
///
/// Implementations are of course permitted to just use `open` and
/// `close` respectively, but doing so would require root permissions
/// to open devices. This interface provides an api agnostic way to
/// use ConsoleKit or similar endpoints to open devices without
/// direct priviledge escalation.
pub type LibinputInterface = ffi::libinput_interface;

ffi_ref_struct!(
/// Libinput context
///
/// Contexts can be used to track input devices and receive events from them.
/// You can use either `new_from_udev` to create a context tracking all devices on a specific seat,
/// or use `new_from_path` to track input devices manually.
///
/// Either way you then have to use `dispatch()` and `next()` (provided by the `Iterator` trait) to
/// receive events.
struct Libinput, ffi::libinput, ffi::libinput_ref, ffi::libinput_unref, ffi::libinput_get_user_data, ffi::libinput_set_user_data);

impl Iterator for Libinput {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = unsafe { ffi::libinput_get_event(self.as_raw_mut()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Event::from_raw(ptr)) }
        }
    }
}

impl Libinput {
    /// Create a new libinput context using a udev context.
    ///
    /// This context is inactive until `udev_assign_seat` is called.
    ///
    /// ## Arguments
    ///
    /// - interface - A `LibinputInterface` providing functions to open and close devices.
    /// - userdata - Optionally some userdata attached to the newly created context (see [`Userdata`](./trait.Userdata.html))
    /// - udev_context - Raw pointer to a valid udev context.
    ///
    /// ## Unsafety
    ///
    /// This function is unsafe, because there is no way to verify that `udev_context` is indeed a valid udev context or even points to valid memory.
    pub unsafe fn new_from_udev<T: 'static>(interface: LibinputInterface,
                                            userdata: Option<T>,
                                            udev_context: *mut libc::c_void)
                                            -> Libinput {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: {
                ffi::libinput_udev_create_context(&*boxed_interface as *const _,
                                                  match (*boxed_userdata).as_mut() {
                                                      Some(value) => value as *mut T as *mut libc::c_void,
                                                      None => ptr::null_mut(),
                                                  },
                                                  udev_context as *mut _)
            },
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    /// Create a new libinput context that requires the caller to manually add or remove devices.
    ///
    /// The returned context is active, but will not yield any events
    /// until at least one device is added.
    ///
    /// Devices can be added and removed by calling `path_add_device` and `path_remove_device` respectively.
    ///
    /// ## Arguments
    ///
    /// - interface - A `LibinputInterface` providing functions to open and close devices.
    /// - userdata - Optionally some userdata attached to the newly created context (see [`Userdata`](./trait.Userdata.html))
    ///
    pub fn new_from_path<T: 'static>(interface: LibinputInterface, userdata: Option<T>) -> Libinput {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: unsafe {
                ffi::libinput_path_create_context(&*boxed_interface as *const _,
                                                  match (*boxed_userdata).as_mut() {
                                                      Some(value) => value as *mut T as *mut libc::c_void,
                                                      None => ptr::null_mut(),
                                                  })
            },
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    /// Add a device to a libinput context initialized with
    /// `new_from_context`.
    ///
    /// If successful, the device will be added to the internal list
    /// and re-opened on `resume`. The device can be removed with
    /// `path_remove_device()`.
    ///
    /// If the device was successfully initialized, it is returned.
    ///
    /// ## Warning
    ///
    /// It is an application bug to call this function on a context
    /// initialized with `new_from_udev`.
    pub fn path_add_device(&mut self, path: &str) -> Option<Device> {
        let path = CString::new(path).expect("Device Path contained a null-byte");
        unsafe {
            let ptr = ffi::libinput_path_add_device(self.as_raw_mut(), path.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(Device::from_raw(ptr))
            }
        }
    }

    /// Remove a device from a libinput context initialized with
    /// `new_from_path` and added to such a context with
    /// `path_add_device`.
    ///
    /// Events already processed from this input device are kept in
    /// the queue, the `DeviceRemovedEvent` event marks the end of
    /// events for this device.
    ///
    /// ## Warning
    ///
    /// It is an application bug to call this function on a context
    /// initialized with `new_from_udev`.
    pub fn path_remove_device(&mut self, device: Device) {
        unsafe { ffi::libinput_path_remove_device(device.as_raw_mut()) }
    }

    /// Assign a seat to this libinput context.
    ///
    /// New devices or the removal of existing devices will appear as
    /// events during `dispatch`.
    ///
    /// `udev_assign_seat` succeeds even if no input devices are
    /// currently available on this seat, or if devices are available
    /// but fail to open in `LibinputInterface::open_restricted`.
    ///
    /// Devices that do not have the minimum capabilities to be
    /// recognized as pointer, keyboard or touch device are ignored. /// Such devices and those that failed to open ignored until the
    /// next call to `resume`.
    ///
    /// ## Warning
    ///
    /// This function may only be called once per context.
    pub fn udev_assign_seat(&mut self, seat_id: &str) -> Result<(), ()> {
        let id = CString::new(seat_id).expect("Seat Id contained a null-byte");
        unsafe {
            match ffi::libinput_udev_assign_seat(self.as_raw_mut(), id.as_ptr()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    ffi_func!(
    /// Suspend monitoring for new devices and close existing
    /// devices.
    ///
    /// This closes all open devices and terminates libinput but
    /// does keep the context valid to be resumed with `resume`.
    pub fn suspend, ffi::libinput_suspend, ());

    /// Resume a suspended libinput context.
    ///
    /// This re-enables device monitoring and adds existing devices.
    pub fn resume(&mut self) -> Result<(), ()> {
        unsafe {
            match ffi::libinput_resume(self.as_raw_mut()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    /// Main event dispatchment function.
    ///
    /// Reads events of the file descriptors and processes them
    /// internally. Use `next` or any other function provided by the
    /// `Iterator` trait to retrieve the events until `None` is
    /// returned.
    ///
    /// Dispatching does not necessarily queue libinput events. This
    /// function should be called immediately once data is available
    /// on the file descriptor returned by `fd`. libinput has a number
    /// of timing-sensitive features (e.g. tap-to-click), any delay in
    /// calling `dispatch` may prevent these features from working
    /// correctly.
    pub fn dispatch(&mut self) -> IoResult<()> {
        unsafe {
            match ffi::libinput_dispatch(self.as_raw_mut()) {
                0 => Ok(()),
                x if x < 0 => Err(IoError::from_raw_os_error(x)),
                _ => unreachable!(),
            }
        }
    }

    /// libinput keeps a single file descriptor for all events.
    ///
    /// Call into `dispatch` if any events become available on this fd.
    ///
    /// The most simple variant to check for available bytes is to use
    /// the `libc`:
    ///
    ///     loop {
    ///         let mut count = 0i32;
    ///         libc::ioctl(context.fd(), libc::FIONREAD, &mut count);
    ///         if (count > 0) {
    ///             context.dispatch().unwrap();
    ///             for event in context {
    ///                 // do some processing...
    ///             }
    ///         }
    ///     }
    ///
    /// For more complex operations you may wish to use other approches
    /// as event loops e.g. in the `wayland-server` or the `tokio`
    /// crates to wait for data to become available on this file
    /// descriptor.
    pub unsafe fn fd(&self) -> RawFd {
        ffi::libinput_get_fd(self.as_raw_mut())
    }
}
