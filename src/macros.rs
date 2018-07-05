macro_rules! run {
    ($command:expr$(,$arg:expr)*) => {
        println!(concat!("RUN>", $command)$(, $arg)*);
    };
}
