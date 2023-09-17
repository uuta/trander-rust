#[macro_export]
macro_rules! info_request_log {
    () => {
        info!(
            message = "Received request",
            method_name = module_path!(),
            line = line!(),
            column = column!(),
            file = file!()
        );
    };
}
