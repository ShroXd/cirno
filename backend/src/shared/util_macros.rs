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

/// A macro for defining a standard actor message handler with database operations
///
/// ## Arguments
///
/// * `message_type` - The type of the message being handled
/// * `return_type` - The return type of the handler
/// * `database_function` - The database function to call
/// * `success_return` - A closure that processes the successful database result
/// * `error_return` - The value to return in case of error
///
/// ## Example
/// ```
/// define_actor_message_handler!(
///     message_type = GetMediaItems,
///     return_type = Vec<MediaItemDto>,
///     database_function = query_series,
///     success_return = |res| res,
///     error_return = Vec::<MediaItemDto>::new()
/// )
/// ```
///
/// ## Returns
/// Implements a Handler trait that:
/// - Calls the specified database function with the message
/// - Processes the result with success_return on success
/// - Returns error_return on failure
///

#[macro_export]
macro_rules! define_actor_message_handler {
    (
        message_type = $msg_type:ty,
        return_type = $result_type:ty,
        db_call = $db_call:expr,
        success_return = $success_return:expr,
        error_return = $error_return:expr
    ) => {
        impl Handler<$msg_type> for Database {
            type Result = ResponseActFuture<Self, $result_type>;

            #[instrument(skip(self, msg))]
            fn handle(&mut self, msg: $msg_type, _: &mut Self::Context) -> Self::Result {
                debug!("Processing {}", msg);
                let pool = self.get_connection_pool();
                let query_manager = self.get_query_manager();

                Box::pin(
                    // TODO: Currently we pass parameters by value to database functions due to limitations
                    // in macro parameter type handling. This incurs copy overhead. Need to find a way to
                    // pass references through macros in the future.
                    async move { ($db_call)(&pool, query_manager, msg).await }
                        .into_actor(self)
                        .then(|result, _actor, _ctx| match result {
                            // Use parentheses to ensure success_return is parsed as an closure function
                            Ok(data) => fut::ready(($success_return)(data)),
                            Err(e) => {
                                error!("Error processing {}: {:?}", stringify!($msg_type), e);
                                fut::ready($error_return)
                            }
                        }),
                )
            }
        }
    };
}

/// A macro for forwarding pipeline actions from a WebSocket actor to a pipeline actor
///
/// ## Arguments
///
/// * `$self` - The WebSocket actor instance
/// * `$pipeline_action` - The pipeline action to forward
///
/// ## Example
/// ```
/// process_pipeline_action!(
///     self,
///     Play
/// )
/// ```
///
/// ## Returns
/// Logs debug information and:
/// - If pipeline address exists, forwards the action to the pipeline actor
/// - If forwarding fails, logs an error message
///

#[macro_export]
macro_rules! process_pipeline_action {
    ($self:ident, $pipeline_action:ident) => {{
        debug!(
            "WebSocket actor received {} action",
            stringify!($pipeline_action)
        );

        if let Some(pipeline_addr) = $self.pipeline_addr.as_ref() {
            debug!(
                "WebSocket actor sending {} action to pipeline",
                stringify!($pipeline_action)
            );

            if let Err(e) = pipeline_addr.try_send(PipelineAction::$pipeline_action) {
                error!("Failed to forward message to pipeline: {:?}", e);
            }
        }
    }};
}

/// A macro for defining event payload structs with type field and constructor
///
/// ## Arguments
///
/// * `$vis` - Visibility modifier for the struct
/// * `$name` - Name of the struct to define
/// * `$field_name` - Name of each field in the struct
/// * `$field_ty` - Type of each field in the struct
///
/// ## Example
/// ```
/// define_payload!(
///     pub RegisterClient {
///         client_key: String
///     }
/// )
/// ```
///
/// ## Returns
/// Defines a struct that:
/// - Has a "type" field with the struct name
/// - Has the specified fields with their types
/// - Includes a constructor that sets the type and fields
/// - Field names are converted to camelCase in both Rust and TypeScript
///   since the JSON payload will be sent via WebSocket to the web client
///   and consumed by TypeScript code
///

#[macro_export]
macro_rules! define_payload {
    (
        $(#[$outer:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_name:ident: $field_ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Serialize, Deserialize, TS)]
        #[serde(rename_all = "camelCase")]
        #[ts(export, rename_all = "camelCase")]
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_name: $field_ty,
            )*
        }

        impl $name {
            $vis fn new($($field_name: $field_ty,)*) -> Self {
                Self {
                    $($field_name,)*
                }
            }
        }
    };
}
