//! Handles launching appropriate modes based on cmd line input
//!

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches};

use log::*;

pub struct Launcher {
    args: ArgMatches, //let see if we hold them or parse them only once
}
