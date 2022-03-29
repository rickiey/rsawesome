use log::{debug, info, warn};

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        setup_logging().expect("failed to initialize logging.");
        info!("test 4564564654654654");
        for i in 0..5 {
            info!("executing section: {}", i);

            debug!("section {} 1/4 complete.", i);

            debug!("section {} 1/2 complete.", i);

            debug!("section {} 3/4 complete.", i);

            info!("section {} completed!", i);
        }
    }
}
