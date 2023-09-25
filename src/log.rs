macro_rules! log {
    ($($arg:tt)*) => {{
        #[cfg(feature = "log")]
        println!($($arg)*)
    }};
}
