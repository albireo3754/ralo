pub fn init_log_settings() {
    let log_settings = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .with_file(true)
        .finish();

    tracing::subscriber::set_global_default(log_settings).unwrap();
}
