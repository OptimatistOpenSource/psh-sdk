#![no_std]
#![cfg(target_arch = "wasm32")]

#[allow(unused_imports)]
use profiling_prelude_intrinsics as intrinsics;

#[macro_export]
macro_rules! print {
    () => {
        intrinsics::log("")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        intrinsics::log(format!("{}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        intrinsics::log("\n")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        intrinsics::log(format!("{}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! eprint {
    () => {
        intrinsics::log_err("")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        intrinsics::log_err(format!("{}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        intrinsics::log_err("\n")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        intrinsics::log_err(format!("{}\n", format_args!($($arg)*)));
    }};
}
