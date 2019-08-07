fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;

    Ok(())
}

fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{date} in {target} [{level}]: {message}",
                date = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("dirscod", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}