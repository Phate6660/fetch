// TODO: replace reqwest with a lighter crate :(
// -- kiedtl

mod env;
mod cpu;
mod wmde;
mod pkgs;
mod music;
mod uptime;
mod device;
mod distro;
mod kernel;
mod network;
mod output;

use clap::{App, Arg};
use log::error;
use snafu::{OptionExt, ResultExt, Snafu};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::result;

use crate::env::*;
use crate::cpu::*;
use crate::wmde::*;
use crate::pkgs::*;
use crate::music::*;
use crate::uptime::*;
use crate::device::*;
use crate::distro::*;
use crate::kernel::*;
use crate::network::*;
use crate::output::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to retrieve device name: {}", source))]
    DeviceName { source: io::Error },
    #[snafu(display("Unable to retrieve Linux distro: {}", source))]
    OsRelease { source: io::Error },
    #[snafu(display("Unable to retrieve kernel version: {}", source))]
    KernelVersion { source: io::Error },
    #[snafu(display("Unable to read the provided logo file: {}", source))]
    ReadLogo { source: io::Error },
    #[snafu(display("Unable to retrieve uptime: {}", source))]
    Uptime { source: io::Error },
    #[snafu(display("Unable to determine home directory"))]
    HomeDir,
    #[snafu(display("Unable to open .xinitrc: {}", source))]
    OpenXInitRc { source: io::Error },
    #[snafu(display("Empty .xinitrc"))]
    EmptyXInitRc,
    #[snafu(display("Unable to read .xinitrc: {}", source))]
    ReadXInitRc { source: io::Error },
    #[snafu(display("Unable to guess window manager"))]
    GuessWm,
    #[snafu(display("Unable to retrieve USER, SHELL, or EDITOR/VISUAL."))]
    EnvError { source: std::env::VarError },
    #[snafu(display("Unable to retrieve IP address: {}", source))]
    Reqwest { source: reqwest::Error },
    #[snafu(display("Unable to retrieve package count."))]
    Pkgcount { source: io::Error },
    #[snafu(display("Unable to retrive mpd information."))]
    Mpc { source: io::Error },
    #[snafu(display("Unable to retrieve CPU information: {}", source))]
    CPUErr { source: io::Error },
}

pub type Result<T, E = Error> = result::Result<T, E>;

// Default art.
fn get_default_logo() -> String {
    let mut logo: String;
    logo = format!("{}",           bold(" \\    / /\\   |    |    |--- \\   /\n"));
    logo = format!("{}{}\n", logo, bold("  \\  / /__\\  |    |    |---  \\ /"));
           format!("{}{}\n", logo, bold("   \\/ /----\\ |___ |___ |---   |"))
}

// Main function
fn main() {
    pretty_env_logger::init();

    // Variables
    let matches = App::new("rsfetch")
                    .version("1.9.0")
                    .about("\nAn fetch tool for Linux. Fast (~1ms execution time) and somewhat(?) minimal.\n\nAll options are off by default. \n\nAccepted values for the package manager are \"pacman\", \"apt\", \"xbps\", \"dnf\", \"pkg\", \"eopkg\", \"rpm\", \"apk\", \"pip\", \"portage\", and \"cargo\".")
                    .arg(Arg::with_name("credits")
                        .long("credits")
                        .help("List of past and current contributors for this project."))
                    .arg(Arg::with_name("no-bold")
                        .short("b")
                        .long("no-bold")
                        .help("Turn bold for field titles off."))
                    .arg(Arg::with_name("no-borders")
                        .short("B")
                        .long("no-borders")
                        .help("Turn borders off."))
                    .arg(Arg::with_name("no-caps")
                        .short("c")
                        .long("no-caps")
                        .help("Turn all caps off."))
                    .arg(Arg::with_name("cpu")
                         .long("cpu")
                         .help("Turn CPU information on."))
                    .arg(Arg::with_name("user")
                        .short("U")
                        .long("user")
                        .help("Turn user name off."))
                    .arg(Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .help("Turn device name off."))
                    .arg(Arg::with_name("ip_address")
                        .short("i")
                        .long("ip_address")
                        .help("Turn ip address display on."))
                    .arg(Arg::with_name("editor")
                        .short("e")
                        .long("editor")
                        .help("Turn default editor name on. (Must have $EDITOR/$VISUAL variable set.)"))
                    .arg(Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .help("Turn default shell name off."))
                    .arg(Arg::with_name("wm")
                        .short("w")
                        .long("wm")
                        .help("Turn WM or DE name off."))
                    .arg(Arg::with_name("distro")
                        .short("d")
                        .long("distro")
                        .help("Turn distro name off."))
                    .arg(Arg::with_name("kernel")
                        .short("k")
                        .long("kernel")
                        .help("Turn kernel version off."))
                    .arg(Arg::with_name("uptime")
                        .short("u")
                        .long("uptime")
                        .help("Turn uptime info off."))
                    .arg(Arg::with_name("minimal")
                        .short("M")
                        .long("minimal")
                        .help("Turn minimal mode on."))
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
                        .help("Choose where to get music info. The only supported options is \"mpd\".\n")
                        .takes_value(true))
                    .arg(Arg::with_name("logo")
                        .short("l")
                        .long("logo")
                        .help("Turn the logo or ascii art off."))
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
                        .help("Specify the corner character. Only used when borders are enabled.")
                        .takes_value(true))
                    .get_matches();

    if matches.is_present("credits") {
        println!();
        println!("Maintainer:       valley             (Reddit: /u/Valley6660) (Github: Phate6660)");
        println!("Contributor:      Kied Llaentenn     (Reddit: /u/kiedtl)     (Github: kiedtl)");
        println!("Contributor:      Lauren{}iu Nicola                           (Github: lnicola)\n",
            std::char::from_u32(539 as u32).unwrap());
        println!("With thanks to:   \"/r/rust\", \"/u/tablair\", \"/u/kabocha_\", \"/u/DebuggingPanda\", for their contributions, and the tool \"neofetch\" for giving the inspiration to create this.");
        println!();
        return;
    }
    
    let bold = !matches.is_present("no-bold");
    let caps = !matches.is_present("no-caps");
    let borders = !matches.is_present("no-borders");

    // For the options that require bools or other input.
    let corners = matches.value_of("corners").unwrap_or("■");
    let music = matches.value_of("music").unwrap_or("");
    let logofile = matches.value_of("logofile").unwrap_or("");
    let packages = matches.value_of("packages");

    let style;
    if matches.is_present("minimal") {
        style = OutputType::Minimal;
    } else {
        style = OutputType::Rsfetch;
    }

    let corner: char;
    if matches.is_present("minimal") || !borders {
        corner = ' ';
    } else if borders {
        corner = corners.chars().collect::<Vec<char>>()[0];
    } else {
        corner = '■';
    }

    let opts = OutputOptions {
        output_type: style,
        caps:        caps,
        bold:        bold,
        use_borders: borders,
        borders:     corner,
    };

    //let format;
    // env: variable that holds $USER, $SHELL, and $VISUAL or $EDITOR.
    let mut env = EnvInfo::new();
    
    // --- OUTPUT ---
    // if there aren't any options, then no information fields
    // will be enabled, which means we may as well exit now
    if std::env::args().collect::<Vec<String>>().len() < 2 {
        std::process::exit(0); // get the hell outta here!
    }

    if matches.is_present("logo") {
        print!("\n"); // print blank line before output.
    }

    let mut writer = OutputHelper::new(opts);
    
    // Determine the logo to use.
    if matches.is_present("logo") {
        let mut logo: String = "".to_owned();
        if !logofile.is_empty() {
            let res_logo = std::fs::read_to_string(logofile)
                .context(ReadLogo);
            match res_logo {
                Ok(l)  => logo = l,
                Err(e) => error!("{:?}", e),
            }
        } else {
            logo = get_default_logo();
        }
        writer.ascii(logo);
    }

    if matches.is_present("user") {
        match env.get(EnvItem::User) {
            Ok(()) => writer.add("USER", &env.format(EnvItem::User)),
            Err(e) => error!("{}", e),
        }
    }
    
    if matches.is_present("host") {
        let mut device = DeviceInfo::new();
        match device.get() {
            Ok(()) => writer.add("HOST", &device.format()),
            Err(e) => error!("{}", e),
        }
    }
    
    if matches.is_present("uptime") {
        let mut uptime = UptimeInfo::new();
        match uptime.get() {
            Ok(()) => writer.add("UPTIME", &uptime.format()),
            Err(e) => error!("{}", e),
        }
    }
    
    if matches.is_present("distro") {
        let mut distro = DistroInfo::new();
        match distro.get() {
            Ok(()) => writer.add("DISTRO", &distro.format()),
            Err(e) => error!("{}", e),
        }
    }

    if matches.is_present("kernel") {
        let mut kernel = KernelInfo::new();
        match kernel.get() {
            Ok(()) => writer.add("KERNEL", &kernel.format()),
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("wm") {
        let mut wmde = WMDEInfo::new();
        match wmde.get() {
            Ok(()) => writer.add("WM/DE", &wmde.format()),
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("editor") {
        match env.get(EnvItem::Editor) {
            Ok(()) => writer.add("EDITOR", &env.format(EnvItem::Editor)),
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("shell") {
        match env.get(EnvItem::Shell) {
            Ok(()) => writer.add("SHELL", &env.format(EnvItem::Shell)),
            Err(e) => error!("{}", e),
        }
    }
    if matches.is_present("cpu") {
        let mut cpu = CPUInfo::new();
        match cpu.get() {
            Ok(()) => writer.add("CPU", &cpu.format()),
            Err(e) => error!("{}", e),
        }
    }

    if matches.is_present("ip_address") {
        let mut ip = NetworkInfo::new();
        match ip.get() {
            Ok(()) => writer.add("IP ADDRESS", &ip.format()),
            Err(e) => error!("{}", e),
        }
    }

    if let Some(packages) = packages {
        let mut pkgs = PkgInfo::new();
        pkgs.set_manager(packages);
        
        match pkgs.get() {
            Ok(()) => writer.add(
                &format!("PACKAGES ({})", packages.to_ascii_uppercase()), &pkgs.format()),
            Err(e) => error!("{}", e),
        }
    }

    if music == "mpd" {
        let mut mpd = MusicInfo::new();
        
        match mpd.get() {
            Ok(()) => writer.add("MUSIC (MPD)", &mpd.format()),
            Err(e) => error!("{}", e),
        }
    }

    writer.output();

    print!("\n"); // blank line
}
