//! # libgpiod public API
//!
//! This is the complete documentation of the public API made available to
//! users of libgpiod.
//!
//! The API is logically split into several parts such as: GPIO chip & line
//! operators, GPIO events handling etc.
//!
//! ### General note on error handling
//! All routines exported by libgpiod set `errno` to one of the error values
//! defined in `errno.h` upon failure. The way of notifying the caller that an
//! error occurred varies between functions, but in general a function that
//! returns an int, returns `-1` on error, while a function returning a pointer
//! bails out on error condition by returning `NULL` pointer.
#![allow(dead_code)]

use libc::*;

// Re-export enums to be even more C like
pub use gpiod_line_bias::*;
pub use gpiod_line_bulk_cb::*;
pub use gpiod_line_direction::*;
pub use gpiod_line_drive::*;
pub use gpiod_line_event_type::*;
pub use gpiod_line_request_flag::*;
pub use gpiod_line_request_type::*;

/// Opaque type representing a GPIO chip.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct gpiod_chip {
    _data: [u8; 0],
    _metadata: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque type representing a GPIO line handle.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct gpiod_line {
    _data: [u8; 0],
    _metadata: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Opaque type representing a line bulk object.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct gpiod_line_bulk {
    _data: [u8; 0],
    _metadata: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    /// Check if the file pointed to by path is a GPIO chip character device.
    ///
    /// ## Arguments
    /// * `path` Path to check.
    ///
    /// ## Return
    /// True if the file exists and is a GPIO chip character device or a
    /// symbolic link to it.
    pub fn gpiod_is_gpiochip_device(path: *const c_char) -> bool;
}

extern "C" {
    /// Open a [`gpiod_chip`] by path.
    ///
    /// ## Arguments
    /// * `path` Path to the [`gpiod_chip`] device file.
    ///
    /// ## Return
    /// GPIO chip handle or `NULL` if an error occurred.
    pub fn gpiod_chip_open(path: *const c_char) -> *mut gpiod_chip;
}

extern "C" {
    /// Increase the refcount on this GPIO object.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    ///
    /// ## Return
    /// Passed reference to the GPIO chip.
    pub fn gpiod_chip_ref(chip: *mut gpiod_chip) -> *mut gpiod_chip;
}

extern "C" {
    /// Decrease the refcount on this GPIO object. If the refcount reaches `0`,
    /// close the chip device and free all associated resources.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    pub fn gpiod_chip_unref(chip: *mut gpiod_chip);
}

extern "C" {
    /// Get the GPIO chip name as represented in the kernel.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    ///
    /// ## Return
    /// Pointer to a human-readable string containing the chip name.
    pub fn gpiod_chip_get_name(chip: *mut gpiod_chip) -> *const c_char;
}

extern "C" {
    /// Get the GPIO chip label as represented in the kernel.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    ///
    /// ## Return
    /// Pointer to a human-readable string containing the chip label.
    pub fn gpiod_chip_get_label(chip: *mut gpiod_chip) -> *const c_char;
}

extern "C" {
    /// Get the number of GPIO lines exposed by this chip.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    ///
    /// ## Return
    /// Number of GPIO lines.
    pub fn gpiod_chip_get_num_lines(chip: *mut gpiod_chip) -> c_uint;
}

extern "C" {
    /// Get the handle to the GPIO line at given offset.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    /// * `offset` The offset of the GPIO line.
    ///
    /// ## Return
    /// Pointer to the GPIO line handle or `NULL` if an error occured.
    pub fn gpiod_chip_get_line(
        chip: *mut gpiod_chip,
        offset: c_uint,
    ) -> *mut gpiod_line;
}

extern "C" {
    /// Retrieve a set of lines and store them in a line bulk object.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    /// * `offsets` Array of offsets of lines to retrieve.
    /// * `num_offsets` Number of lines to retrieve.
    ///
    /// ## Return
    /// New line bulk object or `NULL` on error.
    pub fn gpiod_chip_get_lines(
        chip: *mut gpiod_chip,
        offsets: *mut c_uint,
        num_offsets: c_uint,
    ) -> *mut gpiod_line_bulk;
}

extern "C" {
    /// Retrieve all lines exposed by a chip and store them in a bulk object.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    ///
    /// ## Return
    /// New line bulk object or `NULL` on error.
    pub fn gpiod_chip_get_all_lines(chip: *mut gpiod_chip) -> *mut gpiod_line_bulk;
}

extern "C" {
    /// Map a GPIO line's name to its offset within the chip.
    ///
    /// ## Arguments
    /// * `chip` The GPIO chip object.
    /// * `name` Name of the GPIO line to map.
    ///
    /// ## Return
    /// Offset of the line within the chip or `-1` if a line with given name
    /// is not exposed by the chip.
    pub fn gpiod_chip_find_line(
        chip: *mut gpiod_chip,
        name: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Allocate and initialize a new line bulk object.
    ///
    /// ## Arguments
    /// * `max_lines` Maximum number of lines this object can hold.
    ///
    /// ## Return
    /// New line bulk object or `NULL` on error.
    pub fn gpiod_line_bulk_new(max_lines: c_uint) -> *mut gpiod_line_bulk;
}

extern "C" {
    /// Reset a bulk object. Remove all lines and set size to `0`.
    ///
    /// ## Arguments
    /// * `bulk` Bulk object to reset.
    pub fn gpiod_line_bulk_reset(bulk: *mut gpiod_line_bulk);
}

extern "C" {
    /// Release all resources allocated for this bulk object.
    ///
    /// ## Arguments
    /// * `bulk` Bulk object to free.
    pub fn gpiod_line_bulk_free(bulk: *mut gpiod_line_bulk);
}

extern "C" {
    /// Add a single line to a GPIO bulk object.
    ///
    /// ## Arguments
    /// * `bulk` Line bulk object.
    /// * `line` Line to add.
    ///
    /// ## Return
    /// `0` on success, `-1` on error.
    ///
    /// ## Note
    /// The line is added at the next free bulk index.
    ///
    /// The function can fail if this bulk already holds its maximum amount of
    /// lines or if the added line is associated with a different chip than all
    /// the other lines already held by this object.
    pub fn gpiod_line_bulk_add_line(
        bulk: *mut gpiod_line_bulk,
        line: *mut gpiod_line,
    ) -> c_int;
}

extern "C" {
    /// Retrieve the line handle from a line bulk object at given index.
    ///
    /// ## Arguments
    /// * `bulk` Line bulk object.
    /// * `index` Index of the line to retrieve.
    ///
    /// ## Return
    /// Line handle at given index or `NULL` if index is greater or equal to
    /// the number of lines this bulk can hold.
    pub fn gpiod_line_bulk_get_line(
        bulk: *mut gpiod_line_bulk,
        index: c_uint,
    ) -> *mut gpiod_line;
}

extern "C" {
    /// Retrieve the number of GPIO lines held by this line bulk object.
    ///
    /// ## Arguments
    /// * `bulk` Line bulk object.
    ///
    /// ## Return
    /// Number of lines held by this line bulk.
    pub fn gpiod_line_bulk_num_lines(bulk: *mut gpiod_line_bulk) -> c_uint;
}

/// Values returned by the callback passed to [`gpiod_line_bulk_foreach_line()`].
#[allow(non_camel_case_types)]
pub enum gpiod_line_bulk_cb {
    /// Continue the loop.
    GPIOD_LINE_BULK_CB_NEXT = 0,
    /// Stop the loop.
    GPIOD_LINE_BULK_CB_STOP = 1,
}

/// Signature of the callback passed to [`gpiod_line_bulk_foreach_line()`].
///
/// ## Arguments
/// * `line` GPIO line object.
/// * `user_data` additional user data.
///
/// ## Return
/// Returns [`GPIOD_LINE_BULK_CB_NEXT`] or [`GPIOD_LINE_BULK_CB_STOP`]
#[allow(non_camel_case_types)]
pub type gpiod_line_bulk_foreach_cb = extern "C" fn(line: *mut gpiod_line, user_data: *mut c_void) -> c_int;

extern "C" {
    /// Iterate over all lines held by this bulk object.
    ///
    /// ## Arguments
    /// * `bulk` Bulk object to iterate over.
    /// * `func` Callback to be called for each line.
    /// * `data` User data pointer that is passed to the callback.
    pub fn gpiod_line_bulk_foreach_line(
        bulk: *mut gpiod_line_bulk,
        func: gpiod_line_bulk_foreach_cb,
        data: *mut c_void,
    );
}

/// Possible direction settings.
#[allow(non_camel_case_types)]
pub enum gpiod_line_direction {
    /// Direction is input - we're reading the state of a GPIO line.
    GPIOD_LINE_DIRECTION_INPUT = 1,
    /// Direction is output - we're driving the GPIO line.
    GPIOD_LINE_DIRECTION_OUTPUT = 2,
}

/// Possible drive settings.
#[allow(non_camel_case_types)]
pub enum gpiod_line_drive {
    /// Drive setting is push-pull
    GPIOD_LINE_DRIVE_PUSH_PULL = 1,
    /// Line output is open-drain.
    GPIOD_LINE_DRIVE_OPEN_DRAIN = 2,
    /// Line output is open-source.
    GPIOD_LINE_DRIVE_OPEN_SOURCE = 3,
}

/// Possible internal bias settings.
#[allow(non_camel_case_types)]
pub enum gpiod_line_bias {
    /// The internal bias state is unknown.
    GPIOD_LINE_BIAS_UNKNOWN = 1,
    /// The internal bias is disabled.
    GPIOD_LINE_BIAS_DISABLED = 2,
    /// The internal pull-up bias is enabled.
    GPIOD_LINE_BIAS_PULL_UP = 3,
    /// The internal pull-down bias is enabled.
    GPIOD_LINE_BIAS_PULL_DOWN = 4,
}

extern "C" {
    /// Read the GPIO line offset.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Line offset.
    pub fn gpiod_line_offset(line: *mut gpiod_line) -> c_uint;
}

extern "C" {
    /// Read the GPIO line name.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Name of the GPIO line as it is represented in the kernel. This
    /// routine returns a pointer to a null-terminated string or `NULL` if
    /// the line is unnamed.
    pub fn gpiod_line_name(line: *mut gpiod_line) -> *const c_char;
}

extern "C" {
    /// Read the GPIO line consumer name.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Name of the GPIO consumer name as it is represented in the
    /// kernel. This routine returns a pointer to a null-terminated string
    /// or `NULL` if the line is not used.
    pub fn gpiod_line_consumer(line: *mut gpiod_line) -> *const c_char;
}

extern "C" {
    /// Read the GPIO line direction setting.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Returns [`GPIOD_LINE_DIRECTION_INPUT`] or [`GPIOD_LINE_DIRECTION_OUTPUT`].
    pub fn gpiod_line_direction(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Check if the signal of this line is inverted.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// True if this line is "active-low", false otherwise.
    pub fn gpiod_line_is_active_low(line: *mut gpiod_line) -> bool;
}

extern "C" {
    /// Read the GPIO line bias setting.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Returns [`GPIOD_LINE_BIAS_PULL_UP`], [`GPIOD_LINE_BIAS_PULL_DOWN`],
    /// [`GPIOD_LINE_BIAS_DISABLED`] or [`GPIOD_LINE_BIAS_UNKNOWN`].
    pub fn gpiod_line_bias(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Check if the line is currently in use.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// True if the line is in use, false otherwise.
    ///
    /// ## Note
    /// The user space can't know exactly why a line is busy. It may have been
    /// requested by another process or hogged by the kernel. It only matters that
    /// the line is used and we can't request it.
    pub fn gpiod_line_is_used(line: *mut gpiod_line) -> bool;
}

extern "C" {
    /// Read the GPIO line drive setting.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Returns [`GPIOD_LINE_DRIVE_PUSH_PULL`], [`GPIOD_LINE_DRIVE_OPEN_DRAIN`] or
    /// [`GPIOD_LINE_DRIVE_OPEN_SOURCE`].
    pub fn gpiod_line_drive(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Get the handle to the GPIO chip controlling this line.
    ///
    /// ## Arguments
    /// * `line` The GPIO line object.
    ///
    /// ## Return
    /// Pointer to the GPIO chip handle controlling this line.
    pub fn gpiod_line_get_chip(line: *mut gpiod_line) -> *mut gpiod_chip;
}

/// Available types of requests.
#[allow(non_camel_case_types)]
pub enum gpiod_line_request_type {
    /// Request the line(s), but don't change current direction.
    GPIOD_LINE_REQUEST_DIRECTION_AS_IS = 1,
    /// Request the line(s) for reading the GPIO line state.
    GPIOD_LINE_REQUEST_DIRECTION_INPUT = 2,
    /// Request the line(s) for setting the GPIO line state.
    GPIOD_LINE_REQUEST_DIRECTION_OUTPUT = 3,
    /// Only watch falling edge events.
    GPIOD_LINE_REQUEST_EVENT_FALLING_EDGE = 4,
    /// Only watch rising edge events.
    GPIOD_LINE_REQUEST_EVENT_RISING_EDGE = 5,
    /// Monitor both types of events.
    GPIOD_LINE_REQUEST_EVENT_BOTH_EDGES = 6,
}

/// Miscellaneous GPIO request flags.
#[allow(non_camel_case_types)]
pub enum gpiod_line_request_flag {
    /// The line is an open-drain port.
    GPIOD_LINE_REQUEST_FLAG_OPEN_DRAIN = 1,
    /// The line is an open-source port.
    GPIOD_LINE_REQUEST_FLAG_OPEN_SOURCE = 2,
    /// The active state of the line is low (high is the default).
    GPIOD_LINE_REQUEST_FLAG_ACTIVE_LOW = 4,
    /// The line has neither either pull-up nor pull-down resistor.
    GPIOD_LINE_REQUEST_FLAG_BIAS_DISABLED = 8,
    /// The line has pull-down resistor enabled.
    GPIOD_LINE_REQUEST_FLAG_BIAS_PULL_DOWN = 16,
    /// The line has pull-up resistor enabled.
    GPIOD_LINE_REQUEST_FLAG_BIAS_PULL_UP = 32,
}

/// Structure holding configuration of a line request.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct gpiod_line_request_config {
    /// Name of the consumer.
    pub consumer: *const c_char,
    /// Request type.
    pub request_type: c_int,
    /// Other configuration flags.
    pub flags: c_int,
}

extern "C" {
    /// Reserve a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `config` Request options.
    /// * `default_val` Initial line value - only relevant if we're setting
    /// the direction to output.
    ///
    /// ## Return
    /// `0` if the line was properly reserved. In case of an error this
    /// routine returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If this routine succeeds, the caller takes ownership of the GPIO line until
    /// it's released.
    pub fn gpiod_line_request(
        line: *mut gpiod_line,
        config: *const gpiod_line_request_config,
        default_val: c_int,
    ) -> c_int;
}

extern "C" {
    /// Reserve a single line, set the direction to input.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the line was properly reserved, `-1` on failure.
    pub fn gpiod_line_request_input(
        line: *mut gpiod_line,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Reserve a single line, set the direction to output.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `default_val` Initial line value.
    ///
    /// ## Return
    /// `0` if the line was properly reserved, `-1` on failure.
    pub fn gpiod_line_request_output(
        line: *mut gpiod_line,
        consumer: *const c_char,
        default_val: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request rising edge event notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_rising_edge_events(
        line: *mut gpiod_line,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Request falling edge event notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_falling_edge_events(
        line: *mut gpiod_line,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Request all event type notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_both_edges_events(
        line: *mut gpiod_line,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Reserve a single line, set the direction to input.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the line was properly reserved, `-1` on failure.
    pub fn gpiod_line_request_input_flags(
        line: *mut gpiod_line,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Reserve a single line, set the direction to output.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    /// * `default_val` Initial line value.
    ///
    /// ## Return
    /// `0` if the line was properly reserved, `-1` on failure.
    pub fn gpiod_line_request_output_flags(
        line: *mut gpiod_line,
        consumer: *const c_char,
        flags: c_int,
        default_val: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request rising edge event notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_rising_edge_events_flags(
        line: *mut gpiod_line,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request falling edge event notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_falling_edge_events_flags(
        line: *mut gpiod_line,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request all event type notifications on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_both_edges_events_flags(
        line: *mut gpiod_line,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Reserve a set of GPIO lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `config` Request options.
    /// * `default_vals` Initial line values - only relevant if we're setting
    /// the direction to output.
    ///
    /// ## Return
    /// `0` if all lines were properly requested. In case of an error
    /// this routine returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If this routine succeeds, the caller takes ownership of the GPIO lines
    /// until they're released. All the requested lines must be provided by the
    /// same [`gpiod_chip`].
    pub fn gpiod_line_request_bulk(
        bulk: *mut gpiod_line_bulk,
        config: *const gpiod_line_request_config,
        default_vals: *const c_int,
    ) -> c_int;
}

extern "C" {
    /// Reserve a set of GPIO lines, set the direction to input.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the lines were properly reserved, `-1` on failure.
    pub fn gpiod_line_request_bulk_input(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Reserve a set of GPIO lines, set the direction to output.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `consumer` Name of the consumer.
    /// * `default_vals` Initial line values.
    ///
    /// ## Return
    /// `0` if the lines were properly reserved, `-1` on failure.
    pub fn gpiod_line_request_bulk_output(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        default_vals: *const c_int,
    ) -> c_int;
}

extern "C" {
    /// Request rising edge event notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_rising_edge_events(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Request falling edge event notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_falling_edge_events(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Request all event type notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_both_edges_events(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
    ) -> c_int;
}

extern "C" {
    /// Reserve a set of GPIO lines, set the direction to input.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the lines were properly reserved, `-1` on failure.
    pub fn gpiod_line_request_bulk_input_flags(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Reserve a set of GPIO lines, set the direction to output.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    /// * `default_vals` Initial line values.
    ///
    /// ## Return
    /// `0` if the lines were properly reserved, `-1` on failure.
    pub fn gpiod_line_request_bulk_output_flags(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        flags: c_int,
        default_vals: *const c_int,
    ) -> c_int;
}

extern "C" {
    /// Request rising edge event notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_rising_edge_events_flags(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request falling edge event notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_falling_edge_events_flags(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Request all event type notifications on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to request.
    /// * `consumer` Name of the consumer.
    /// * `flags` Additional request flags.
    ///
    /// ## Return
    /// `0` if the operation succeeds, `-1` on failure.
    pub fn gpiod_line_request_bulk_both_edges_events_flags(
        bulk: *mut gpiod_line_bulk,
        consumer: *const c_char,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Release a previously reserved line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    pub fn gpiod_line_release(line: *mut gpiod_line);
}

extern "C" {
    /// Release a set of previously reserved lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to release.
    ///
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_release_bulk(bulk: *mut gpiod_line_bulk);
}

extern "C" {
    /// Read current value of a single GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// `0` or `1` if the operation succeeds. On error this routine returns `-1`
    /// and sets the last error number.
    pub fn gpiod_line_get_value(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Read current values of a set of GPIO lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `values` An array big enough to hold [`gpiod_line_bulk_num_lines(bulk)`](gpiod_line_bulk_num_lines) values.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If succeeds, this routine fills the values array with a set of values in
    /// the same order, the lines are added to [`gpiod_line_bulk`]. If the lines were not
    /// previously requested together, the behavior is undefined.
    pub fn gpiod_line_get_value_bulk(
        bulk: *mut gpiod_line_bulk,
        values: *mut c_int,
    ) -> c_int;
}

extern "C" {
    /// Set the value of a single GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `value` New value.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    pub fn gpiod_line_set_value(
        line: *mut gpiod_line,
        value: c_int,
    ) -> c_int;
}

extern "C" {
    /// Set the values of a set of GPIO lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to reserve.
    /// * `values` An array holding [`gpiod_line_bulk_num_lines(bulk)`](gpiod_line_bulk_num_lines) new values for lines.
    /// A `NULL` pointer is interpreted as a logical low for all lines.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_set_value_bulk(
        bulk: *mut gpiod_line_bulk,
        values: *const c_int,
    ) -> c_int;
}

extern "C" {
    /// Update the configuration of a single GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `direction` Updated direction which may be one of
    /// [`GPIOD_LINE_REQUEST_DIRECTION_AS_IS`],
    /// [`GPIOD_LINE_REQUEST_DIRECTION_INPUT`], or
    /// [`GPIOD_LINE_REQUEST_DIRECTION_OUTPUT`].
    /// * `flags` Replacement flags.
    /// * `value` The new output value for the line when direction is
    /// [`GPIOD_LINE_REQUEST_DIRECTION_OUTPUT`].
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    pub fn gpiod_line_set_config(
        line: *mut gpiod_line,
        direction: c_int,
        flags: c_int,
        value: c_int,
    ) -> c_int;
}

extern "C" {
    /// Update the configuration of a set of GPIO lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines.
    /// * `direction` Updated direction which may be one of
    /// [`GPIOD_LINE_REQUEST_DIRECTION_AS_IS`],
    /// [`GPIOD_LINE_REQUEST_DIRECTION_INPUT`], or
    /// [`GPIOD_LINE_REQUEST_DIRECTION_OUTPUT`].
    /// * `flags` Replacement flags.
    /// * `values` An array holding [`gpiod_line_bulk_num_lines(bulk)`](gpiod_line_bulk_num_lines) new logical values
    /// for lines when direction is
    /// [`GPIOD_LINE_REQUEST_DIRECTION_OUTPUT`].
    /// A `NULL` pointer is interpreted as a logical low for all lines.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_set_config_bulk(
        bulk: *mut gpiod_line_bulk,
        direction: c_int,
        flags: c_int,
        values: *const c_int,
    ) -> c_int;
}

extern "C" {
    /// Update the configuration flags of a single GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `flags` Replacement flags.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    pub fn gpiod_line_set_flags(
        line: *mut gpiod_line,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Update the configuration flags of a set of GPIO lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines.
    /// * `flags` Replacement flags.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_set_flags_bulk(
        bulk: *mut gpiod_line_bulk,
        flags: c_int,
    ) -> c_int;
}

extern "C" {
    /// Set the direction of a single GPIO line to input.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    pub fn gpiod_line_set_direction_input(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Set the direction of a set of GPIO lines to input.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_set_direction_input_bulk(bulk: *mut gpiod_line_bulk)
        -> c_int;
}

extern "C" {
    /// Set the direction of a single GPIO line to output.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `value` The logical value output on the line.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    pub fn gpiod_line_set_direction_output(
        line: *mut gpiod_line,
        value: c_int,
    ) -> c_int;
}

extern "C" {
    /// Set the direction of a set of GPIO lines to output.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines.
    /// * `values` An array holding [`gpiod_line_bulk_num_lines(bulk)`](gpiod_line_bulk_num_lines) new logical values
    /// for lines. A `NULL` pointer is interpreted as a logical low
    /// for all lines.
    ///
    /// ## Return
    /// `0` is the operation succeeds. In case of an error this routine
    /// returns `-1` and sets the last error number.
    ///
    /// ## Note
    /// If the lines were not previously requested together, the behavior is
    /// undefined.
    pub fn gpiod_line_set_direction_output_bulk(
        bulk: *mut gpiod_line_bulk,
        values: *const c_int,
    ) -> c_int;
}

/// Event type.
#[allow(non_camel_case_types)]
pub enum gpiod_line_event_type {
    GPIOD_LINE_EVENT_RISING_EDGE = 1,
    GPIOD_LINE_EVENT_FALLING_EDGE = 2,
}

/// Structure holding event info.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct gpiod_line_event {
    /// Best estimate of time of event occurrence.
    pub ts: timespec,
    /// Type of the event that occurred.
    pub event_type: c_int,
    /// Offset of line on which the event occurred.
    pub offset: c_int,
}

extern "C" {
    /// Wait for an event on a single line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `timeout` Wait time limit.
    ///
    /// ## Return
    /// `0` if wait timed out, `-1` if an error occurred, `1` if an event
    /// occurred.
    pub fn gpiod_line_event_wait(
        line: *mut gpiod_line,
        timeout: *const timespec,
    ) -> c_int;
}

extern "C" {
    /// Wait for events on a set of lines.
    ///
    /// ## Arguments
    /// * `bulk` Set of GPIO lines to monitor.
    /// * `timeout` Wait time limit.
    /// * `event_bulk` Bulk object in which to store the line handles on which
    /// events occurred. Can be `NULL`.
    ///
    /// ## Return
    /// `0` if wait timed out, `-1` if an error occurred, `1` if at least one
    /// event occurred.
    pub fn gpiod_line_event_wait_bulk(
        bulk: *mut gpiod_line_bulk,
        timeout: *const timespec,
        event_bulk: *mut gpiod_line_bulk,
    ) -> c_int;
}

extern "C" {
    /// Read next pending event from the GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `event` Buffer to which the event data will be copied.
    ///
    /// ## Return
    /// `0` if the event was read correctly, `-1` on error.
    ///
    /// ## Note
    /// This function will block if no event was queued for this line.
    pub fn gpiod_line_event_read(
        line: *mut gpiod_line,
        event: *mut gpiod_line_event,
    ) -> c_int;
}

extern "C" {
    /// Read up to a certain number of events from the GPIO line.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    /// * `events` Buffer to which the event data will be copied. Must hold at
    /// least the amount of events specified in `num_events`.
    /// * `num_events` Specifies how many events can be stored in the buffer.
    ///
    /// ## Return
    /// On success returns the number of events stored in the buffer, on
    /// failure `-1` is returned.
    pub fn gpiod_line_event_read_multiple(
        line: *mut gpiod_line,
        events: *mut gpiod_line_event,
        num_events: c_uint,
    ) -> c_int;
}

extern "C" {
    /// Get the event file descriptor.
    ///
    /// ## Arguments
    /// * `line` GPIO line object.
    ///
    /// ## Return
    /// Number of the event file descriptor or `-1` if the user tries to
    /// retrieve the descriptor from a line that wasn't configured for
    /// event monitoring.
    ///
    /// ## Note
    /// Users may want to poll the event file descriptor on their own. This routine
    /// allows to access it.
    pub fn gpiod_line_event_get_fd(line: *mut gpiod_line) -> c_int;
}

extern "C" {
    /// Read the last GPIO event directly from a file descriptor.
    ///
    /// ## Arguments
    /// * `fd` File descriptor.
    /// * `event` Buffer in which the event data will be stored.
    ///
    /// ## Return
    /// `0` if the event was read correctly, `-1` on error.
    ///
    /// ## Note
    /// Users who directly poll the file descriptor for incoming events can also
    /// directly read the event data from it using this routine. This function
    /// translates the kernel representation of the event to the libgpiod format.
    pub fn gpiod_line_event_read_fd(
        fd: c_int,
        event: *mut gpiod_line_event,
    ) -> c_int;
}

extern "C" {
    /// Read up to a certain number of events directly from a file descriptor.
    ///
    /// ## Arguments
    /// * `fd` File descriptor.
    /// * `events` Buffer to which the event data will be copied. Must hold at
    /// least the amount of events specified in `num_events`.
    /// * `num_events` Specifies how many events can be stored in the buffer.
    ///
    /// ## Return
    /// On success returns the number of events stored in the buffer, on
    /// failure `-1` is returned.
    pub fn gpiod_line_event_read_fd_multiple(
        fd: c_int,
        events: *mut gpiod_line_event,
        num_events: c_uint,
    ) -> c_int;
}

extern "C" {
    /// Get the API version of the library as a human-readable string.
    ///
    /// ## Return
    /// Human-readable string containing the library version.
    pub fn gpiod_version_string() -> *const c_char;
}
