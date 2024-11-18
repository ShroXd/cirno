/// A macro for handling controller results and converting them to HTTP responses
///
/// ## Arguments
///
/// * `$result` - A Result containing either the success value or an error
/// * `$ok_status` - The HTTP status to use for successful responses (e.g. HttpResponse::Ok())
/// * `$err_status` - The HTTP status to use for error responses (e.g. HttpResponse::InternalServerError())
///
/// ## Example
/// ```
/// handle_controller_result!(
///     controller_function(),
///     HttpResponse::Ok(),
///     HttpResponse::InternalServerError()
/// )
/// ```
///
/// ## Returns
/// Returns a JSON response with either:
/// - The success value and provided OK status
/// - The error message string and provided error status
///

#[macro_export]
macro_rules! handle_controller_result {
    ($result:expr, $ok_status:expr, $err_status:expr) => {
        match $result {
            Ok(result) => $ok_status.json(result),
            Err(e) => $err_status.json(e.to_string()),
        }
    };
}
