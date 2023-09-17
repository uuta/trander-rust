#[macro_export]
macro_rules! error_log {
    ($e:expr) => {
        error!(
            message = format!("Error occurred: {:?}", $e),
            method_name = module_path!(),
            line = line!(),
            column = column!(),
            file = file!()
        );
    };
}
