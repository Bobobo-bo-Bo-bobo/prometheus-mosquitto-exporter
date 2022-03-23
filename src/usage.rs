use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2021-2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.
    
{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!(
        "Usage: {} [-V|--version] -c <config>|--config=<config> [-h|--help]

    -V                  Show version information
    --version

    -c <config>         Configuration file
    --config=<config>

    -h                  Show this help text
    --help

    -q                  Quiet operation. Only log warning
    --quiet             and error messages
",
        constants::NAME
    );
}
