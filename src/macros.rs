macro_rules! run {
    ($command:expr$(,$arg:expr)*) => {
        println!(concat!("RUN>", $command)$(, $arg)*);
    };
}

macro_rules! error {
    ($message:expr$(,$arg:expr)*) => {
        eprintln!(concat!("{} ", $message), "error:".red().bold()$(, $arg)*);
    };
}

macro_rules! warn {
    ($message:expr$(,$arg:expr)*) => {
        println!(concat!("{} ", $message), "warning:".yellow().bold()$(, $arg)*);
    };
}

macro_rules! log {
    ($message:expr$(,$arg:expr)*) => {
        unsafe {
            if VERBOSE {
                println!(concat!("{} ", $message), "info:".bright_blue().bold()$(, $arg)*);
            }
        }
    };
}
