use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false);

    let fmt_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    #[cfg(feature = "console")]
    let console_layer = console_subscriber::ConsoleLayer::builder()
        .with_default_env()
        .spawn();

    let reg = tracing_subscriber::registry()
        .with(fmt_layer.with_filter(fmt_filter)); // фильтр ТОЛЬКО на fmt

    #[cfg(feature = "console")]
    let reg = reg.with(console_layer); // без фильтра: получит tokio=trace

    reg.init();
}

// fn init_tracing() {
//     let console_layer = console_subscriber::ConsoleLayer::builder()
//         .with_default_env()
//         .spawn();
//
//     let fmt_layer = tracing_subscriber::fmt::layer()
//         .compact()
//         .with_file(true)
//         .with_line_number(true)
//         .with_thread_ids(true)
//         .with_target(false);
//
//     // Фильтр только для текстовых логов
//     let fmt_filter = EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| EnvFilter::new("info"));
//
//     tracing_subscriber::registry()
//         .with(console_layer)            // без фильтра (получает все события, включая tokio=TRACE)
//         .with(fmt_layer.with_filter(fmt_filter)) // фильтр применён только к fmt
//         .init();
// }