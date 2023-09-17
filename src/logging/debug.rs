#[macro_export]
macro_rules! debug_log {
    ($e:expr) => {
        debug!(
            message = format!("Debug log: {:?}", $e),
            method_name = module_path!(),
            line = line!(),
            column = column!(),
            file = file!()
        );
    };
}
