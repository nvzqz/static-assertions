/// Asserts that a given configuration is set.
///
/// # Examples
///
/// A project may not support a set of configurations and thus you may want to
/// report why:
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// // We should only be compiling for Unix or Linux
/// # #[cfg(any(unix, linux))]
/// assert_cfg!(any(unix, linux));
/// # fn main() {}
/// ```
///
/// If users need to specify a database back-end:
///
/// ```
/// # #[macro_use]
/// # extern crate static_assertions;
/// # #[cfg(target_pointer_width = "0")] // Impossible
/// assert_cfg!("Must exclusively use MySQL or MongoDB as database back-end",
///             all(not(all(feature = "mysql", feature = "mongodb")),
///                 any(    feature = "mysql", feature = "mongodb")));
/// # fn main() {}
/// ```
///
/// We can't be compiling for both Unix _and_ Windows simultaneously:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_cfg!("No, that's not how it works! ಠ_ಠ", all(unix, windows));
/// # }
/// ```
#[macro_export]
macro_rules! assert_cfg {
    () => {};
    ($msg:expr, $($cfg:tt)*) => {
        #[cfg(not($($cfg)*))]
        compile_error!($msg);
    };
    ($($cfg:tt)*) => {
        #[cfg(not($($cfg)*))]
        compile_error!(concat!("Cfg does not pass: ", stringify!($($cfg)*)));
    };
}
