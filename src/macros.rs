#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($var:expr) => {
        ::web_sys::console::debug_1(&format!("{}: {:?}", stringify!($var), &$var).into());
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($var:expr) => {
        // no-op(release)
    };
}

#[macro_export]
macro_rules! error {
    ($e:expr, $push_toast:expr) => {
        ::web_sys::console::error_1(&format!("{:?}", &$e).into());
        ($push_toast).emit(($crate::models::ToastKind::Error, format!("{:?}", &$e)));
    };
}
