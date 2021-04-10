use crate::{
    ffi::{linux::free, vk, xcb},
    gfx::InstanceTable,
    input::{Event, Key},
};
use std::{
    ffi::{c_void, CStr},
    ptr::{null, null_mut, NonNull},
};

fn register_for_wm_delete(connection: *mut xcb::Connection, window_id: u32) -> Option<u32> {
    // NOTE: Don't need NULL-terminated strings here
    let protocol = "WM_PROTOCOLS";
    let delete = "WM_DELETE_WINDOW";

    unsafe {
        let protocol_cookie =
            xcb::intern_atom(connection, 1, protocol.len() as u16, protocol.as_ptr());
        let protocol_reply = xcb::intern_atom_reply(connection, protocol_cookie, null_mut());
        if protocol_reply == null_mut() {
            return None;
        }

        let delete_cookie = xcb::intern_atom(connection, 0, delete.len() as u16, delete.as_ptr());
        let delete_reply = xcb::intern_atom_reply(connection, delete_cookie, null_mut());
        if delete_reply == null_mut() {
            free(protocol_reply as *mut c_void);
            return None;
        }

        xcb::change_property(
            connection,
            0,
            window_id,
            (*protocol_reply).atom,
            4,
            32,
            1,
            &(*delete_reply).atom as *const u32 as *const c_void,
        );

        let delete_atom = (*delete_reply).atom;
        free(protocol_reply as *mut c_void);
        free(delete_reply as *mut c_void);

        return Some(delete_atom);
    }
}

pub struct Window {
    atom_delete: u32,
    connection: *mut xcb::Connection,
    event: *mut xcb::GenericEvent,
    pub exiting: bool,
    window_id: u32,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window {
    pub fn create_surface(
        &self,
        table: &InstanceTable,
        instance: *mut vk::Instance,
    ) -> Option<NonNull<vk::SurfaceKHR>> {
        let info = vk::XcbSurfaceCreateInfoKHR {
            stype: vk::StructureType::XCBSurfaceCreateInfoKHR,
            next: null(),
            flags: 0,
            connection: self.connection,
            window_id: self.window_id,
        };
        let mut surface = null_mut();
        let result = (table.create_xcb_surface_khr)(instance, &info, null(), &mut surface);
        if result != vk::Result::Success {
            return None;
        }

        return NonNull::new(surface);
    }

    pub fn dimensions_inner(&self) -> (u32, u32) {
        unsafe {
            let cookie = xcb::get_geometry(self.connection, self.window_id);
            let reply = xcb::get_geometry_reply(self.connection, cookie, null_mut());

            let dims = ((*reply).width as u32, (*reply).height as u32);
            free(reply as *mut c_void);

            return dims;
        }
    }

    pub fn new(instance_name: &str, class_name: &str, width: u16, height: u16) -> Self {
        let (connection, window_id, screen) = unsafe {
            let c = xcb::connect(null(), null_mut());
            let w = xcb::generate_id(c);
            let s = xcb::setup_roots_iterator(xcb::get_setup(c)).data;

            (c, w, s)
        };

        let mask = 2048;
        let values = [xcb::EventMask::KeyPress as u32
            | xcb::EventMask::KeyRelease as u32
            | xcb::EventMask::ButtonPress as u32
            | xcb::EventMask::ButtonRelease as u32
            | xcb::EventMask::PointerMotion as u32
            | xcb::EventMask::StructureNotify as u32
            | xcb::EventMask::FocusChange as u32];
        unsafe {
            xcb::create_window(
                connection,
                0,
                window_id,
                (*screen).root_window,
                0,
                0,
                width,
                height,
                0,
                1,
                (*screen).root_visual,
                mask,
                values.as_ptr() as *mut c_void,
            )
        };

        let atom_delete = register_for_wm_delete(connection, window_id)
            .expect("Failed to register for WM_DELETE event!");
        let window = Window {
            atom_delete,
            connection,
            event: null_mut(),
            exiting: false,
            window_id,
        };
        window.set_title(instance_name, class_name);

        unsafe {
            xcb::map_window(connection, window_id);
            xcb::flush(connection);
        }
        return window;
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event = unsafe { xcb::poll_for_event(self.connection) };
        if self.event == null_mut() {
            return None;
        }

        let event_type = unsafe { (*self.event).response_type & !0x80 };
        let event = match event_type {
            // Keyboard Event
            2 | 3 => {
                let key_press = self.event as *mut xcb::KeyPressEvent;
                let (response_type, key_code) =
                    unsafe { ((*key_press).response_type, (*key_press).key_code) };

                let key = match key_code {
                    36 => Key::Enter,
                    113 => Key::ArrowLeft,
                    114 => Key::ArrowRight,
                    _ => {
                        println!("Unknown key code {}", key_code);
                        Key::Unknown
                    }
                };

                match response_type {
                    2 => Some(Event::KeyPress(key)),
                    3 => Some(Event::KeyRelease(key)),
                    _ => None,
                }
            }
            xcb::FOCUS_IN => {
                std::process::Command::new("xset")
                    .args(&["r", "rate", "10", "25"])
                    .spawn()
                    .unwrap();
                Some(Event::WindowFocused)
            }
            xcb::FOCUS_OUT => {
                Some(Event::WindowUnfocused)
            }
            xcb::CONFIGURE_NOTIFY => {
                let config_event = self.event as *mut xcb::ConfigureNotifyEvent;
                unsafe {
                    let width = (*config_event).width;
                    let height = (*config_event).height;
                    Some(Event::WindowResize(width, height))
                }
            }
            // Client Event
            33 => {
                // If the WM_DELETE_WINDOW atom was sent, set `exiting` to true.
                let client_message = self.event as *mut xcb::ClientMessageEvent;
                let atom_value = unsafe { (*client_message).data.data32[0] };
                if atom_value == self.atom_delete {
                    self.exiting = true;
                }

                None
            }
            _ => None,
        };

        unsafe {
            free(self.event as *mut c_void);
        }
        return event;
    }

    pub fn set_title(&self, instance_name: &str, class_name: &str) {
        use std::iter::once;

        // Concatenate instance_name with a NUL-byte
        let term_in = instance_name.bytes().chain(once(0));
        // Concatenate class_name with a NUL-byte
        let term_cn = class_name.bytes().chain(once(0));
        // Concatenate the NUL-terminated instance and class names into one byte slice
        let title: Box<[u8]> = term_in.chain(term_cn).collect();

        unsafe {
            xcb::icccm::set_wm_class(
                self.connection,
                self.window_id,
                title.len() as u32,
                title.as_ptr(),
            );
        }
    }

    pub fn surface_extension_name() -> &'static CStr {
        return CStr::from_bytes_with_nul(b"VK_KHR_xcb_surface\0").unwrap();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            xcb::destroy_window(self.connection, self.window_id);
            xcb::disconnect(self.connection);
        }
    }
}
