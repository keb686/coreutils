// This file is part of the uutils coreutils package.
//
// (c) Joao Oliveira <joaoxsouls@gmail.com>
// (c) Jian Zeng <anonymousknight96 AT gmail.com>
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// last synced with: uname (GNU coreutils) 8.21

// spell-checker:ignore (API) nodename osname sysname (options) mnrsv mnrsvo

use clap::{crate_version, Arg, ArgAction, Command};
use platform_info::*;
use uucore::{
    error::{FromIo, UResult},
    format_usage,
};

const ABOUT: &str = r#"Print certain system information.
With no OPTION, same as -s."#;
const USAGE: &str = "{} [OPTION]...";

pub mod options {
    pub static ALL: &str = "all";
    pub static KERNEL_NAME: &str = "kernel-name";
    pub static NODENAME: &str = "nodename";
    pub static KERNEL_VERSION: &str = "kernel-version";
    pub static KERNEL_RELEASE: &str = "kernel-release";
    pub static MACHINE: &str = "machine";
    pub static PROCESSOR: &str = "processor";
    pub static HARDWARE_PLATFORM: &str = "hardware-platform";
    pub static OS: &str = "operating-system";
}

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches = uu_app().try_get_matches_from(args)?;

    let uname =
        PlatformInfo::new().map_err_context(|| "failed to create PlatformInfo".to_string())?;
    let mut output = String::new();

    let all = matches.get_flag(options::ALL);
    let kernel_name = matches.get_flag(options::KERNEL_NAME);
    let nodename = matches.get_flag(options::NODENAME);
    let kernel_release = matches.get_flag(options::KERNEL_RELEASE);
    let kernel_version = matches.get_flag(options::KERNEL_VERSION);
    let machine = matches.get_flag(options::MACHINE);
    let processor = matches.get_flag(options::PROCESSOR);
    let hardware_platform = matches.get_flag(options::HARDWARE_PLATFORM);
    let os = matches.get_flag(options::OS);

    let none = !(all
        || kernel_name
        || nodename
        || kernel_release
        || kernel_version
        || machine
        || os
        || processor
        || hardware_platform);

    if kernel_name || all || none {
        output.push_str(&uname.sysname());
        output.push(' ');
    }

    if nodename || all {
        // maint: [2023-01-14; rivy] remove `.trim_end_matches('\0')` when platform-info nodename-NUL bug is fixed (see GH:uutils/platform-info/issues/32)
        output.push_str(uname.nodename().trim_end_matches('\0'));
        output.push(' ');
    }

    if kernel_release || all {
        output.push_str(&uname.release());
        output.push(' ');
    }

    if kernel_version || all {
        output.push_str(&uname.version());
        output.push(' ');
    }

    if machine || all {
        output.push_str(&uname.machine());
        output.push(' ');
    }

    if os || all {
        output.push_str(&uname.osname());
        output.push(' ');
    }

    // This option is unsupported on modern Linux systems
    // See: https://lists.gnu.org/archive/html/bug-coreutils/2005-09/msg00063.html
    if processor {
        output.push_str("unknown");
        output.push(' ');
    }

    // This option is unsupported on modern Linux systems
    // See: https://lists.gnu.org/archive/html/bug-coreutils/2005-09/msg00063.html
    if hardware_platform {
        output.push_str("unknown");
        output.push(' ');
    }

    println!("{}", output.trim_end());

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .arg(
            Arg::new(options::ALL)
                .short('a')
                .long(options::ALL)
                .help("Behave as though all of the options -mnrsvo were specified.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::KERNEL_NAME)
                .short('s')
                .long(options::KERNEL_NAME)
                .alias("sysname") // Obsolescent option in GNU uname
                .help("print the kernel name.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::NODENAME)
                .short('n')
                .long(options::NODENAME)
                .help(
                    "print the nodename (the nodename may be a name that the system \
                is known by to a communications network).",
                )
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::KERNEL_RELEASE)
                .short('r')
                .long(options::KERNEL_RELEASE)
                .alias("release") // Obsolescent option in GNU uname
                .help("print the operating system release.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::KERNEL_VERSION)
                .short('v')
                .long(options::KERNEL_VERSION)
                .help("print the operating system version.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::MACHINE)
                .short('m')
                .long(options::MACHINE)
                .help("print the machine hardware name.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::OS)
                .short('o')
                .long(options::OS)
                .help("print the operating system name.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::PROCESSOR)
                .short('p')
                .long(options::PROCESSOR)
                .help("print the processor type (non-portable)")
                .action(ArgAction::SetTrue)
                .hide(true),
        )
        .arg(
            Arg::new(options::HARDWARE_PLATFORM)
                .short('i')
                .long(options::HARDWARE_PLATFORM)
                .help("print the hardware platform (non-portable)")
                .action(ArgAction::SetTrue)
                .hide(true),
        )
}
