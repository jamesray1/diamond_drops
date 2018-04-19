/* Copyright 2018 James Ray (@jamesray1), Josiah Evans (@ChosunOne), Luke Schoen (@ltfschoen)

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE 
AUTHORS, James Ray, Josiah @ChosunOne, and Luke Schoen
BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
 
For more information, please refer to <http://unlicense.org>
*/

use cli::config;

#[derive(PartialEq)]
enum ConfigType {
    Mode,
    Nil
}

/// Parse arguments from the command line and produce a configuration from them.
pub fn parse_cli_args(args: Vec<String>) -> Result<config::Config, &'static str> {
    let mut config_type = ConfigType::Nil;
    
    // Default Case
    let mut mode = config::Mode::Both;

    for arg in args {
        // Match `-` prefixed args to a list of valid configuration points and set their
        // values with the following non `-` prefixed value
        if arg.starts_with("-") {
            match arg.to_lowercase().as_ref() {
                    "-mode" => { config_type = ConfigType::Mode; },
                    _ => { return Err("Invalid configuration argument, try \
                        cargo run -- -mode <argument>, \
                        where argument is proposer, p, notary, n, both, or b."); }
                }
        } else if config_type == ConfigType::Mode {
            // Match provided value to mode type
            match arg.to_lowercase().as_ref() {
                "proposer" | "p" => { mode = config::Mode::Proposer; },
                "notary" | "n" => { mode = config::Mode::Notary; },
                "both" | "b" => { mode = config::Mode::Both; },
                _ => { return Err("Invalid configuration value, try \
                    cargo run -- -mode <argument>, \
                    where argument is proposer, p, notary, n, both, or b."); }
            }

            config_type = ConfigType::Nil;
        } else {
            return Err("No configuration argument supplied, try \
                        cargo run -- -mode <argument>, \
                        where argument is proposer, p, notary, n, both, or b.");
        }
    }

    if config_type == ConfigType::Nil {
        Ok(config::Config::new(mode))
    } else {
        Err("No configuration value supplied, try \
            cargo run -- -mode <argument>, \
            where argument is proposer, p, notary, n, both, or b.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_client_mode_to_proposer() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("proposer")];
        let config_verbose = parse_cli_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("p")];
        let config_short = parse_cli_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Proposer);
        assert_eq!(config_short.mode, config::Mode::Proposer);
    }

    #[test]
    fn it_sets_client_mode_to_notary() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("notary")];
        let config_verbose = parse_cli_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("n")];
        let config_short = parse_cli_args(test_args_short).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Notary);
        assert_eq!(config_short.mode, config::Mode::Notary);
    }

    #[test]
    fn it_sets_client_mode_to_both() {
        // Verbose command
        let test_args_verbose = vec![String::from("-mode"), String::from("both")];
        let config_verbose = parse_cli_args(test_args_verbose).unwrap();
        // Short command
        let test_args_short = vec![String::from("-mode"), String::from("b")];
        let config_short = parse_cli_args(test_args_short).unwrap();
        // Default mode
        let test_args_default = vec![];
        let config_default = parse_cli_args(test_args_default).unwrap();

        assert_eq!(config_verbose.mode, config::Mode::Both);
        assert_eq!(config_short.mode, config::Mode::Both);
        assert_eq!(config_default.mode, config::Mode::Both);
    }

    #[test]
    fn it_reports_invalid_arguments() {
        // Invalid configuration
        let test_args_configuration = vec![String::from("-bin"), String::from("notary")];
        let error_configuration = parse_cli_args(test_args_configuration);

        // Invalid value
        let test_args_value = vec![String::from("-mode"), String::from("bin")];
        let error_value = parse_cli_args(test_args_value);

        // No configuration
        let test_args_no_arg = vec![String::from("mode"), String::from("both")];
        let error_no_arg = parse_cli_args(test_args_no_arg);

        // No value
        let test_args_no_value = vec![String::from("-mode")];
        let error_no_value = parse_cli_args(test_args_no_value);

        assert_eq!(error_configuration, Err("Invalid configuration argument, \
            try cargo run -- -mode <argument>, \
            where argument is proposer, p, notary, n, both, or b."));
        assert_eq!(error_value, Err("Invalid configuration value, try \
            cargo run -- -mode <argument>, \
            where argument is proposer, p, notary, n, both, or b."));
        assert_eq!(error_no_arg, Err("No configuration argument supplied, try \
            cargo run -- -mode <argument>, \
            where argument is proposer, p, notary, n, both, or b."));
        assert_eq!(error_no_value, Err("No configuration value supplied, try \
            cargo run -- -mode <argument>, \
            where argument is proposer, p, notary, n, both, or b."));
    }
}
