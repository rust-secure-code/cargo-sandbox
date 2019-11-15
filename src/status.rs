//! Status mcaros

macro_rules! status {
    ($status:expr, $msg:expr) => {
        {
            use crossterm::style::{Colorize, Styler};
            println!("{:>12} {}", $status.green().bold(), $msg);
        }
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        status!($status, format!($fmt, $($arg)+));
    };
}
