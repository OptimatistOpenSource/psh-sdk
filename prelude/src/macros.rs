#[macro_export]
macro_rules! print {
    () => {
        crate::op::log("")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        crate::op::log_err(format!("{}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        crate::op::log("\n")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        crate::op::log(format!("{}\n", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! eprint {
    () => {
        crate::op::log_err("")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        crate::op::log_err(format!("{}", format_args!($($arg)*)));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {
        crate::op::log_err("\n")
    };
    ($($arg:tt)*) => {{
        use alloc::format;
        crate::op::log_err(format!("{}\n", format_args!($($arg)*)));
    }};
}
