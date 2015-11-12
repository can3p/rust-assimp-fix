//! Defines logging functions

use std::ptr;

use types::AiBool;
use ffi;

pub use log::LogStream::{Stdout, Stderr, Debugger, File /* TODO: ,Custom */ };
use std::ffi::CString;

/// Default logging options for assimp
pub enum LogStream<'a> {
    /// Log to stdout
    Stdout,
    /// Log to stderr
    Stderr,
    /// MSVC only: Stream the log to the debugger
    /// (this relies on OutputDebugString from the Win32 SDK)
    Debugger,
    /// Log to the given file
    File(&'a str),
    // /// TODO
    // /// Log to the given writer
    // Custom(&'a mut (Writer+'a))
}

// TODO//{{{
// pub struct Logger {
//     log: Vec<LogStream>
// }

// pub fn get_error_string() -> Option<String> {
//     unsafe {
//         let pstr = ffi::aiGetErrorString();
//         if pstr.is_null() {
//             return None;
//         }
//         let cstr = CString::new(pstr as *const i8, false);
//         match cstr {
//             Some(ss) => Some(ss.into_string()),
//             None => None,
//         }
//     }
// }

// extern fn stream_call_back(msg: *const c_char, data: *const u8) {
//     unsafe {
//         // this code should work, but gives an Internal Compiler Error
//         // let writer = data as *mut Writer;
//         let cstr = CString::new(msg, false);
//         (*stream).write(cstr.as_bytes()).unwrap();
//     }
// }//}}}

/// Enable/Disable verbose logging for all log streams
pub fn enable_verbose_logging(choice: bool) {
    unsafe {
        ffi::aiEnableVerboseLogging(AiBool::new(choice))
    }
}

/// Attach a log stream to assimp. Multiple log streams may be attach
/// simultaneously
pub fn add_log_stream(log_type: LogStream) {
    unsafe {
        let null = ptr::null();
        let log = match log_type {
            File(fname) => ffi::aiGetPredefinedLogStream(ffi::DefaultLogStream_FILE, CString::new(fname).unwrap().as_ptr()),
            //File(fname) => fname.with_c_str(|s|
                //ffi::aiGetPredefinedLogStream(ffi::DefaultLogStream_FILE, s) ),
            Stdout =>
                ffi::aiGetPredefinedLogStream(ffi::DefaultLogStream_STDOUT, null),
            Stderr =>
                ffi::aiGetPredefinedLogStream(ffi::DefaultLogStream_STDERR, null),
            Debugger =>
                ffi::aiGetPredefinedLogStream(ffi::DefaultLogStream_DEBUGGER, null),
            // // TODO
            // Custom(_writer) => {
            //     // writer.write_be_u32(0u32);
            //     // ffi::LogStream {
            //     //     callback: stream_call_back,
            //     //     // user data will be used to reference our writer
            //     //     user: mem::transmute(writer),
            //     // }
            //     unimplemented!();
            // }
        };
        ffi::aiAttachLogStream(&log);
    }
}

/// Closes all log streams
pub fn detach_all_log_streams() {
    unsafe {
        ffi::aiDetachAllLogStreams();
    }
}

// vim: et tw=78 sw=4:
