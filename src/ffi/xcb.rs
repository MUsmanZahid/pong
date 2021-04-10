use std::ffi::c_void;

// Constants
pub const ENTER_NOTIFY: u8 = 7;
pub const LEAVE_NOTIFY: u8 = 8;
pub const FOCUS_IN: u8 = 9;
pub const FOCUS_OUT: u8 = 10;
pub const CONFIGURE_NOTIFY: u8 = 22;

// Type aliases
pub type KeyCode = u8;
pub type KeyReleaseEvent = KeyPressEvent;
pub type TimeStamp = u32;

// Enums
#[repr(C)]
pub enum ConfigWindow {
    X = 1,
    Y = 2,
    Width = 4,
    Height = 8,
    BorderWidth = 16,
    Sibling = 32,
    StackMode = 64,
}

#[repr(C)]
pub enum EventMask {
    NoEvent = 0,
    KeyPress = 1,
    KeyRelease = 2,
    ButtonPress = 4,
    ButtonRelease = 8,
    EnterWindow = 16,
    LeaveWindow = 32,
    PointerMotion = 64,
    StructureNotify = 131_072,
    FocusChange = 2_097_152,
}

// Opaque Structures
define_handle!(Connection);

// Visible structures
#[repr(C)]
pub union ClientMessageData {
    pub data8: [u8; 20],
    pub data16: [u16; 10],
    pub data32: [u32; 5],
}

#[repr(C)]
pub struct ConfigureNotifyEvent {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: u32,
    pub window_id: u32,
    pub above_sibling: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

#[repr(C)]
pub struct ClientMessageEvent {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window_id: u32,
    pub atom_type: u32,
    pub data: ClientMessageData,
}

#[repr(C)]
pub struct GenericError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

#[repr(C)]
pub struct GetGeometryCookie {
    pub sequence: u32,
}

#[repr(C)]
pub struct GetGeometryReply {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root_window_id: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2],
}

#[repr(C)]
pub struct GenericEvent {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
    pub full_sequence: u32,
}

#[repr(C)]
pub struct InternAtomCookie {
    pub sequence: u32,
}

#[repr(C)]
pub struct InternAtomReply {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: u32,
}

#[repr(C)]
pub struct KeyPressEvent {
    pub response_type: u8,
    pub key_code: KeyCode,
    pub sequence: u16,
    pub time: TimeStamp,
    pub root_window_id: u32,
    pub event_window_id: u32,
    pub child_window_id: u32,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

#[repr(C)]
pub struct ResizeRequestEvent {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window_id: u32,
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
pub struct Screen {
    pub root_window: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_length: u8,
}

#[repr(C)]
pub struct ScreenIterator {
    pub data: *mut Screen,
    pub remaining: i32,
    pub index: i32,
}

#[repr(C)]
pub struct Setup {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub pad1: [u8; 4],
}

#[repr(C)]
pub struct VoidCookie {
    pub sequence: u32,
}

// Functions from libxcb.a
#[link(name = "xcb")]
extern "C" {
    #[link_name = "xcb_change_property"]
    pub fn change_property(
        connection: *mut Connection,
        mode: u8,
        window_id: u32,
        property: u32,
        atom_type: u32,
        format: u8,
        data_length: u32,
        data: *const c_void,
    ) -> VoidCookie;

    #[link_name = "xcb_configure_window"]
    pub fn configure_window(
        connection: *mut Connection,
        window_id: u32,
        value_mask: u16,
        value_list: *const c_void,
    ) -> i32;

    #[link_name = "xcb_connect"]
    pub fn connect(display: *const u8, screen: *mut i32) -> *mut Connection;

    #[link_name = "xcb_create_window"]
    pub fn create_window(
        connection: *mut Connection,
        depth: u8,
        window_id: u32,
        parent_window_id: u32,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: u32,
        value_mask: u32,
        value_list: *const c_void,
    ) -> VoidCookie;

    #[link_name = "xcb_destroy_window"]
    pub fn destroy_window(connection: *mut Connection, window_id: u32) -> VoidCookie;

    #[link_name = "xcb_disconnect"]
    pub fn disconnect(connection: *mut Connection);

    #[link_name = "xcb_flush"]
    pub fn flush(connection: *mut Connection) -> i32;

    #[link_name = "xcb_generate_id"]
    pub fn generate_id(connection: *mut Connection) -> u32;

    #[link_name = "xcb_get_geometry"]
    pub fn get_geometry(connection: *mut Connection, drawable: u32) -> GetGeometryCookie;

    #[link_name = "xcb_get_geometry_reply"]
    pub fn get_geometry_reply(
        connection: *mut Connection,
        cookie: GetGeometryCookie,
        error: *mut *mut GenericError,
    ) -> *mut GetGeometryReply;

    #[link_name = "xcb_get_setup"]
    pub fn get_setup(connection: *mut Connection) -> *const Setup;

    #[link_name = "xcb_intern_atom"]
    pub fn intern_atom(
        connection: *mut Connection,
        only_if_exists: u8,
        name_length: u16,
        name: *const u8,
    ) -> InternAtomCookie;

    #[link_name = "xcb_intern_atom_reply"]
    pub fn intern_atom_reply(
        connection: *mut Connection,
        cookie: InternAtomCookie,
        error: *mut *mut GenericError,
    ) -> *mut InternAtomReply;

    #[link_name = "xcb_map_window"]
    pub fn map_window(connection: *mut Connection, window_id: u32) -> VoidCookie;

    #[link_name = "xcb_poll_for_event"]
    pub fn poll_for_event(connection: *mut Connection) -> *mut GenericEvent;

    #[link_name = "xcb_setup_roots_iterator"]
    pub fn setup_roots_iterator(i: *const Setup) -> ScreenIterator;
}

// Functions from libxcb-icccm.a
pub mod icccm {
    #[link(name = "xcb-icccm")]
    extern "C" {
        #[link_name = "xcb_icccm_set_wm_class"]
        pub fn set_wm_class(
            connection: *mut super::Connection,
            window_id: u32,
            class_length: u32,
            class_name: *const u8,
        ) -> super::VoidCookie;
    }
}
