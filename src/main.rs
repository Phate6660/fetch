// TODO: replace reqwest with a lighter crate :(
// -- kiedtl

mod env;
mod cpu;
mod wmde;
mod pkgs;
mod uptime;
mod device;
mod distro;
mod kernel;
mod network;

use clap::{App, Arg};
use log::error;
use prettytable::{cell, format, row, Table};
use snafu::{OptionExt, ResultExt, Snafu};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::Command;
use std::result;

use crate::env::*;
use crate::cpu::*;
use crate::wmde::*;
use crate::pkgs::*;
use crate::uptime::*;
use crate::device::*;
use crate::distro::*;
use crate::kernel::*;
use crate::network::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not retrieve device name: {}", source))]
    DeviceName { source: io::Error },
    #[snafu(display("Could not read the OS release: {}", source))]
    OsRelease { source: io::Error },
    #[snafu(display("Could not read the kernel version: {}", source))]
    KernelVersion { source: io::Error },
    #[snafu(display("Could not read the logo file: {}", source))]
    ReadLogo { source: io::Error },
    #[snafu(display("Could not format uptime: {}", source))]
    FormatUptime { source: fmt::Error },
    #[snafu(display("Could not determine home directory"))]
    HomeDir,
    #[snafu(display("Could not open .xinitrc: {}", source))]
    OpenXInitRc { source: io::Error },
    #[snafu(display("Empty .xinitrc"))]
    EmptyXInitRc,
    #[snafu(display("Could not read .xinitrc: {}", source))]
    ReadXInitRc { source: io::Error },
    #[snafu(display("Could not guess window manager"))]
    GuessWm,
    #[snafu(display("Could not determine editor"))]
    Editor { source: std::env::VarError },
    #[snafu(display("Could not retrieve IP address: {}", source))]
    Reqwest { source: reqwest::Error },
    #[snafu(display(
        "Could not retrieve package count. Perhaps you input the wrong package manager?"
    ))]
    Pkgcount { source: io::Error },
    #[snafu(display("Could not run mpc"))]
    Mpc { source: io::Error },
}

type Result<T, E = Error> = result::Result<T, E>;

// escape character (U+001B)
const E: char = '\x1B';

// Function for making bold text.
fn make_bold(text: &str) -> String {
    format!("{}[1m{}{}[0m", E, text, E)
}

// Function for adding rows to the table.
fn add_row(table: &mut Table, bold: bool, caps: bool, border: bool, title: &str, value: &str) {
    let mut title_str = title.to_string();
    if !caps {
        title_str = title_str.to_lowercase();
    }
    if bold {
        title_str = make_bold(&title_str);
    }
    if !border {
        table.add_row(row![title_str, value]);
    } else {
        table.add_row(row![title_str, "=", value]);
    }
}

// For custom art.
fn print_logo(file: &str) -> Result<()> {
    let fs = File::open(file).context(ReadLogo)?;
    for line in BufReader::new(fs).lines() {
        println!("{}", make_bold(&line.context(ReadLogo)?));
    }
    Ok(())
}

// Default art.
fn print_default_logo() {
    println!("{}", make_bold(" \\    / /\\   |    |    |--- \\   /"));
    println!("{}", make_bold("  \\  / /__\\  |    |    |---  \\ /"));
    println!("{}", make_bold("   \\/ /----\\ |___ |___ |---   |"));
}

fn get_mpd_song() -> Result<String> {
    let mpc = Command::new("mpc")
        .arg("current")
        .arg("-f")
        .arg("%artist% - (%date%) %album% - %title%")
        .output()
        .context(Mpc)?;
    let mut mus = String::from_utf8_lossy(&mpc.stdout).into_owned();
    mus.pop();
    Ok(mus)
}

// Main function
fn main() {
    pretty_env_logger::init();
    // Variables
    let mut table = Table::new();
    let matches = App::new("rsfetch")
                    .version("1.9.0")
                    .about("\nMy info fetch tool for Linux. Fast (1ms execution time) and somewhat(?) minimal.\n\nAll options are on (with the exception of package count, editor, window manager, and ip address). Music info is turned off by default.\n\nAccepted values for the package manager are \"pacman\", \"apt\", \"xbps\", \"dnf\", \"pkg\", \"eopkg\", \"rpm\", \"apk\", \"pip\", \"portage\", and \"cargo\".")
                    .arg(Arg::with_name("credits")
                        .long("credits")
                        .value_name(" ")
                        .help("Links to those who helped make this, and thanks to others who've helped me.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-bold")
                        .short("b")
                        .long("no-bold")
                        .help("Turn bold for field titles off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-borders")
                        .short("B")
                        .long("no-borders")
                        .help("Turn borders off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-caps")
                        .short("c")
                        .long("no-caps")
                        .help("Turn all caps off.")
                        .takes_value(false))
                    .arg(Arg::with_name("cpu")
                         .long("cpu")
                         .help("Turn CPU information (model, frequency, and processor count) on.")
                         .takes_value(false))
                    .arg(Arg::with_name("no-user")
                        .short("U")
                        .long("no-user")
                        .help("Turn user name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-host")
                        .short("h")
                        .long("no-host")
                        .help("Turn device name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("ip_address")
                        .short("i")
                        .long("ip_address")
                        .help("Turn ip address display on.")
                        .takes_value(false))
                    .arg(Arg::with_name("editor")
                        .short("e")
                        .long("editor")
                        .help("Turn default editor name on. (Must have $EDITOR variable set.).")
                        .takes_value(false))
                    .arg(Arg::with_name("no-shell")
                        .short("s")
                        .long("no-shell")
                        .help("Turn default shell name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-wm-de")
                        .short("w")
                        .long("no-wm-de")
                        .help("Turn window manager or desktop environment name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-distro")
                        .short("d")
                        .long("no-distro")
                        .help("Turn distro name off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-kernel")
                        .short("k")
                        .long("no-kernel")
                        .help("Turn kernel version off.")
                        .takes_value(false))
                    .arg(Arg::with_name("no-uptime")
                        .short("u")
                        .long("no-uptime")
                        .help("Turn uptime info off.")
                        .takes_value(false))
                    .arg(Arg::with_name("minimal")
                        .short("M")
                        .long("minimal")
                        .help("Turn minimal mode on.")
                        .takes_value(false))
                    .arg(Arg::with_name("packages")
                        .short("p")
                        .long("packages")
                        .value_name("PKG MNGR")
                        .help("Turn total package count on.")
                        .takes_value(true))
                    .arg(Arg::with_name("music")
                        .short("m")
                        .long("music")
                        .value_name("SOURCE")
                        .help("Choose where to get music info. Supported options are \"mpd\" (mpc) and no (none).\n")
                        .takes_value(true))
                    .arg(Arg::with_name("no-logo")
                        .short("l")
                        .long("no-logo")
                        .help("Turn the logo or ascii art off.")
                        .takes_value(false))
                    .arg(Arg::with_name("logofile")
                        .short("L")
                        .long("logofile")
                        .value_name("FILE")
                        .help("Specify the file from which to read a custom ASCII logo.")
                        .takes_value(true))
                    .arg(Arg::with_name("corners")
                        .short("C")
                        .long("corners")
                        .value_name("CHARACTER")
                        .help("Specify the corner style. Choose either \"■\" or \"0\". Only used when borders are enabled.")
                        .takes_value(true))
                    .get_matches();
    if matches.is_present("credits") {
        println!();
        println!("Main Developer:   valley  (Reddit: /u/Valley6660) (Github: Phate6660)");
        println!("Contributor:      kiedtl  (Reddit: /u/kiedtl)     (Github: kiedtl)");
        println!("Contributor:      lnicola                         (Github: lnicola)\n");
        println!("With thanks to:   \"/r/rust\", \"/u/tablair\", \"/u/kabocha_\", \"/u/DebuggingPanda\" for all the help they gave; and the tool \"neofetch\" for giving me the inspiration to make this.");
        println!();
        return;
    }
    let bold = !matches.is_present("no-bold");
    let caps = !matches.is_present("no-caps");
    let borders = !matches.is_present("no-borders");
    // For the options that require bools or other input.
    let corners = matches.value_of("corners").unwrap_or("■");
    let music = matches.value_of("music").unwrap_or("no");
    let logofile = matches.value_of("logofile").unwrap_or("");
    let packages = matches.value_of("packages");
    let format;
   
    // env: variable that holds $USER, $SHELL, and $VISUAL or $EDITOR.
    let mut env = EnvInfo::new();
    
    // Determine if borders are used, and if they are, the style of the corners.
    if matches.is_present("minimal") {
        format = format::FormatBuilder::new()
            .column_separator(' ')
            .borders(' ')
            .separators(
                &[format::LinePosition::Top, format::LinePosition::Bottom],
                format::LineSeparator::new(' ', ' ', ' ', ' '),
            )
            .padding(0, 0)
            .build();
        table.set_format(format);
    } else if borders {
        if corners == "■" {
            format = format::FormatBuilder::new()
                .column_separator(' ')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top, format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '─', '■', '■'),
                )
                .padding(1, 1)
                .build();
            table.set_format(format);
        } else if corners == "0" {
            format = format::FormatBuilder::new()
                .column_separator(' ')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top, format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '─', '0', '0'),
                )
                .padding(1, 1)
                .build();
            table.set_format(format);
        }
    }
    // Begin output. Data for variables will *only* be collected if the option for that specific output is turned on. Therefore making the program much more efficient.
    println!(); // Print blank line before output.
                // Determine the logo to use.
    if !matches.is_present("minimal") {
        if !matches.is_present("no-logo") {
            if !logofile.is_empty() {
                if let Err(e) = print_logo(logofile) {
                    error!("{}", e);
                }
            } else {
                print_default_logo()
            }
            println!(); // print a newline
        }
    }

    if !matches.is_present("no-user") {
        match env.get(EnvItem::User) {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", env.format(EnvItem::User));
            } else {
                add_row(&mut table, bold, caps, borders, "USER", &env.format(EnvItem::User));
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-host") {
        let mut device = DeviceInfo::new();
        match device.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", device.format());
            } else {
                add_row(&mut table, bold, caps, borders, "HOST", &device.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-uptime") {
        let mut uptime = UptimeInfo::new();
        match uptime.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", uptime.format());
            } else {
                add_row(&mut table, bold, caps, borders, "UPTIME", &uptime.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-distro") {
        let mut distro = DistroInfo::new();
        match distro.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", distro.format());
            } else {
                add_row(&mut table, bold, caps, borders, "DISTRO", &distro.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-kernel") {
        let mut kernel = KernelInfo::new();
        match kernel.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", kernel.format());
            } else {
                add_row(&mut table, bold, caps, borders, "KERNEL", &kernel.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-wm-de") {
        let mut wmde = WMDEInfo::new();
        match wmde.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", wmde.format());
            } else {
                add_row(&mut table, bold, caps, borders, "WM/DE", &wmde.format());
            },
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("editor") {
        match env.get(EnvItem::Editor) {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", env.format(EnvItem::Editor));
            } else {
                add_row(&mut table, bold, caps, borders, "EDITOR", &env.format(EnvItem::Editor));
            },
            Err(e) => error!("{}", e),
        }
    }
    if !matches.is_present("no-shell") {
        match env.get(EnvItem::Shell) {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", env.format(EnvItem::Shell));
            } else {
                add_row(&mut table, bold, caps, borders, "SHELL", &env.format(EnvItem::Shell));
            },
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("cpu") {
        let mut cpu = CPUInfo::new();
        match cpu.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", cpu.format());
            } else {
                add_row(&mut table, bold, caps, borders, "CPU", &cpu.format());
            },
            Err(e) => error!("{}", e),
        }
    }

    if matches.is_present("ip_address") {
        let mut ip = NetworkInfo::new();
        match ip.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", ip.format());
            } else {
                add_row(&mut table, bold, caps, borders, "IP ADDRESS", &ip.format());
            },
            Err(e) => error!("{}", e),
        }
    }

    if let Some(packages) = packages {
        let pkgs = PkgInfo::new();
        pkgs.set_manager(packages);
        
        match pkgs.get() {
            Ok(()) => if matches.is_present("minimal") {
                println!("{}", pkgs.format());
            } else {
                add_row(&mut table, bold, caps, borders,
                        &format("PACKAGES ({})", packages.to_ascii_uppercase()),
                        &pkgs.format());
            },
            Err(e) => error!("{}", e),
        }
    }

    if music == "mpd" {
        if matches.is_present("minimal") {
            match get_mpd_song() {
                Ok(mus) => println!("{}", &mus),
                Err(e) => error!("{}", e),
            }
        } else {
            match get_mpd_song() {
                Ok(mus) => add_row(&mut table, bold, caps, borders, "MUSIC (MPD)", &mus),
                Err(e) => error!("{}", e),
            }
        }
    }
    if !matches.is_present("minimal") {
        table.printstd();
    }

    print!("\n"); // blank line
}
