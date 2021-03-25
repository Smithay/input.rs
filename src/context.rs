use libc;
use std::ffi::{CStr, CString};
use std::io::{Error as IoError, Result as IoResult};
use std::iter::Iterator;
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::rc::Rc;
#[cfg(feature = "udev")]
use udev::ffi as udev;
use {ffi, AsRaw, Device, Event, FromRaw};

/// libinput does not open file descriptors to devices directly,
/// instead `open_restricted` and `close_restricted` are called for
/// each path that must be opened.
///
/// Implementations are of course permitted to just use `open` and
/// `close` respectively, but doing so would require root permissions
/// to open devices. This interface provides an api agnostic way to
/// use ConsoleKit or similar endpoints to open devices without
/// direct priviledge escalation.
pub trait LibinputInterface {
    /// Open the device at the given path with the flags provided and
    /// return the fd.
    ///
    /// ## Paramater
    /// - `path` - The device path to open
    /// - `flags` Flags as defined by open(2)
    ///
    /// ## Returns
    /// The file descriptor, or a negative errno on failure.
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32>;

    /// Close the file descriptor.
    ///
    /// ## Parameter
    /// `fd` - The file descriptor to close
    fn close_restricted(&mut self, fd: RawFd);
}

unsafe extern "C" fn open_restricted<I: LibinputInterface + 'static>(
    path: *const libc::c_char,
    flags: libc::c_int,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    use std::borrow::Cow;

    if let Some(ref mut interface) = (user_data as *mut I).as_mut() {
        let path_str = CStr::from_ptr(path).to_string_lossy();
        let res = match path_str {
            Cow::Borrowed(string) => interface.open_restricted(Path::new(string), flags),
            Cow::Owned(string) => interface.open_restricted(Path::new(&string), flags),
        };
        match res {
            Ok(fd) => fd,
            Err(errno) => {
                if errno > 0 {
                    -errno
                } else {
                    errno
                }
            }
        }
    } else {
        -1
    }
}

unsafe extern "C" fn close_restricted<I: LibinputInterface + 'static>(
    fd: libc::c_int,
    user_data: *mut libc::c_void,
) {
    if let Some(ref mut interface) = (user_data as *mut I).as_mut() {
        interface.close_restricted(fd)
    }
}

/// Libinput context
///
/// Contexts can be used to track input devices and receive events from them.
/// You can use either `new_from_udev` to create a context tracking all devices on a specific seat,
/// or use `new_from_path` to track input devices manually.
///
/// Either way you then have to use `dispatch()` and `next()` (provided by the `Iterator` trait) to
/// receive events.
pub struct Libinput {
    ffi: *mut ffi::libinput,
    _interface: Option<Rc<Box<dyn LibinputInterface + 'static>>>,
}

impl ::std::fmt::Debug for Libinput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Libinput @{:p}", self.as_raw())
    }
}

impl AsRaw<ffi::libinput> for Libinput {
    fn as_raw(&self) -> *const ffi::libinput {
        self.ffi as *const _
    }
}

impl Clone for Libinput {
    fn clone(&self) -> Self {
        Libinput {
            ffi: unsafe { ffi::libinput_ref(self.as_raw_mut()) },
            _interface: self._interface.clone(),
        }
    }
}

impl Drop for Libinput {
    fn drop(&mut self) {
        unsafe {
            ffi::libinput_unref(self.ffi);
        }
    }
}

impl PartialEq for Libinput {
    fn eq(&self, other: &Self) -> bool {
        self.as_raw() == other.as_raw()
    }
}

impl Eq for Libinput {}

impl ::std::hash::Hash for Libinput {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self.as_raw().hash(state);
    }
}

impl Iterator for Libinput {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = unsafe { ffi::libinput_get_event(self.as_raw_mut()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Event::from_raw(ptr, self)) }
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
    /// # Safety
    ///
    /// This function is unsafe, because there is no way to verify that `udev_context` is indeed a valid udev context or even points to valid memory.
    #[cfg(feature = "udev")]
    pub fn new_with_udev<I: LibinputInterface + 'static>(interface: I) -> Libinput {
        let mut boxed_userdata = Box::new(interface);
        let boxed_interface = Box::new(ffi::libinput_interface {
            open_restricted: Some(open_restricted::<I>),
            close_restricted: Some(close_restricted::<I>),
        });

        let context = unsafe {
            let udev = udev::udev_new();
            let libinput = ffi::libinput_udev_create_context(
                &*boxed_interface as *const _,
                &mut *boxed_userdata as *mut I as *mut libc::c_void,
                udev as *mut input_sys::udev,
            );
            udev::udev_unref(udev);
            Libinput {
                ffi: libinput,
                _interface: Some(Rc::new(
                    boxed_userdata as Box<dyn LibinputInterface + 'static>,
                )),
            }
        };

        mem::forget(boxed_interface);

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
    pub fn new_from_path<I: 'static + LibinputInterface>(interface: I) -> Libinput {
        let mut boxed_userdata = Box::new(interface);
        let boxed_interface = Box::new(ffi::libinput_interface {
            open_restricted: Some(open_restricted::<I>),
            close_restricted: Some(close_restricted::<I>),
        });

        let context = Libinput {
            ffi: unsafe {
                ffi::libinput_path_create_context(
                    &*boxed_interface as *const _,
                    &mut *boxed_userdata as *mut I as *mut libc::c_void,
                )
            },
            _interface: Some(Rc::new(
                boxed_userdata as Box<dyn LibinputInterface + 'static>,
            )),
        };

        mem::forget(boxed_interface);

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
                Some(Device::from_raw(ptr, self))
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
    #[cfg(feature = "udev")]
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
    /// `nix::poll`:
    ///
    /// ```no_run
    /// use nix::poll;
    ///
    /// let pollfd = poll::PollFd::new(context.as_raw_fd(), poll:POLLIN);
    /// while poll::poll(&mut [pollfd], -1).is_ok() {
    ///     context.dispatch().unwrap();
    ///     for event in context {
    ///         // do some processing...
    ///     }
    /// }
    /// ```
    ///
    /// For more complex operations you may wish to use other approches
    /// as event loops e.g. in the `wayland-server` or the `tokio`
    /// crates to wait for data to become available on this file
    /// descriptor.
    ///
    /// # Safety
    ///
    /// See [`AsRawFd`]
    #[deprecated(since = "0.4.1", note = "Use the provided AsRawFd implementation")]
    pub unsafe fn fd(&self) -> RawFd {
        ffi::libinput_get_fd(self.as_raw_mut())
    }

    /// Create a new instance of this type from a raw pointer.
    ///
    /// ## Warning
    ///
    /// If you make use of [`Userdata`](./trait.Userdata.html) make sure you use the correct types
    /// to allow receiving the set userdata. When dealing with raw pointers initialized by other
    /// libraries this must be done extra carefully to select a correct representation.
    ///
    /// If unsure using `()` is always a safe option..
    ///
    /// # Safety
    ///
    /// If the pointer is pointing to a different struct, invalid memory or `NULL` the returned
    /// struct may panic on use or cause other undefined behavior.
    pub unsafe fn from_raw(ffi: *mut ffi::libinput) -> Self {
        Libinput {
            ffi: ffi::libinput_ref(ffi),
            _interface: None,
        }
    }
}

impl AsRawFd for Libinput {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::libinput_get_fd(self.as_raw_mut()) }
    }
}
