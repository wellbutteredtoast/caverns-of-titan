//! An in-house ANSI log system built for CoT.
//! 
//! Since this is built in plain Rust using ANSI, this should be fully universal, no need to tinker from OS to OS, well hopefully.
//! The output files specified by the `out_to_file` flag are standard, using the ISO local timestamp and some other stuff.
//! All logs are dumped into `.logs/(id)-(date)-(time).log` - The date and time of the log is relative to the time and date the game started
//!

// ============================ UTILITY FUNCS ============================ //

fn generate_log() {}

fn write_to_log(content: &String) {}

fn write(content: &String) {}

// ============================ PUBLIC FUNCS ============================ //

pub fn clear_terminal() {}

pub fn log_debug(message: &String, out_to_file: bool) {}

pub fn log_info(message: &String, out_to_file: bool) {}

pub fn log_warning(message: &String, out_to_file: bool) {}

pub fn log_error(message: &String, out_to_file: bool) {}

pub fn log_fatal(message: &String, out_to_file: bool) {}