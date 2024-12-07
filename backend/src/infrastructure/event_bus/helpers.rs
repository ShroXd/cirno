/// A macro for registering event handlers with an event bus
///
/// ## Arguments
///
/// * `$event_bus` - The event bus instance to register the handler with
/// * `$event_type` - The pattern to match events against
/// * `$handler` - The handler function to execute when events match
/// * `$config` - Optional event handler configuration (defaults to one_time)
///
/// ## Example
/// ```
/// listen_event!(
///     event_bus,
///     DomainEvent::MediaLibrary(MediaLibraryEventType::MediaLibrarySaved),
///     |event| async move { println!("Event received: {:?}", event) }
/// )
/// ```
///
/// ## Returns
/// Registers the handler with the event bus and returns the registration future
///

#[macro_export]
macro_rules! listen_event {
    ($event_bus:expr, $event_type:pat, $handler:expr $(,)?) => {
        $event_bus
            .on(
                |event| matches!(event, $event_type),
                $handler,
                EventHandlerConfig::one_time(),
            )
            .await
    };

    ($event_bus:expr, $event_type:pat, $handler:expr, $config:expr $(,)?) => {
        $event_bus
            .on(|event| matches!(event, $event_type), $handler, $config)
            .await
    };
}

/// A macro for registering a chain of dependent event handlers with an event bus
///
/// This macro allows you to set up a sequence of event handlers where each handler
/// can trigger events that are processed by subsequent handlers in the chain.
///
/// ## Arguments
///
/// * `$event_bus` - The event bus instance to register the handlers with
/// * For each handler in the chain:
///   * `match_pattern` - The pattern to match events against
///   * `handler` - The handler function to execute when events match
///   * `config` - Optional event handler configuration (defaults to one_time)
///
/// ## Example
/// ```
/// chain_events!(
///     event_bus,
///     {
///         match_pattern: DomainEvent::ScanStarted,
///         handler: |event| async move {
///             // Process scan started event
///             event_bus.publish(DomainEvent::ProcessingComplete)?;
///             Ok(())
///         },
///         config: EventHandlerConfig::one_time()
///     },
///     {
///         match_pattern: DomainEvent::ProcessingComplete,
///         handler: |event| async move {
///             // Handle processing completion
///             Ok(())
///         },
///         config: EventHandlerConfig::with_retry()
///     }
/// )
/// ```
///
/// ## Returns
/// Registers all handlers in the chain with the event bus and returns a future that
/// completes when all handlers are registered. Each handler can trigger events that
/// will be picked up by subsequent handlers in the chain.
///

#[macro_export]
macro_rules! chain_events {
    (
        $event_bus:expr,
        $(
            {
                match_pattern: $pattern:pat,
                handler: $handler:expr,
                config: $config:expr
            }
        ),+ $(,)?
    ) => {
        async move {
            $(
                $event_bus
                    .on(
                        |event| matches!(event, $pattern),
                        $handler,
                        $config
                    )
                    .await;
            )+
        }.await
    };
}
