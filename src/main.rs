//
// Rust port of src/main.c / src/main.h.
//
// Bygfoot Football Manager -- a small and simple GTK-based
// football management game.
//
// The C program is a GTK2 app using `gtk_init` / `gtk_main`; this
// port uses the `gtk4` crate, where the equivalent lifecycle is
// `gtk::Application::run()`.
//

mod domain;
mod store;
mod ui;

use std::env;
use std::sync::{Mutex, OnceLock};

use gtk::glib;
use gtk::prelude::*;

use domain::option::{self, OptionsList};
use store::file_store::{self, FileStore};
use ui::App;

const DEBUG_LEVEL_DEFAULT: i32 = 0;

#[cfg(any(target_os = "macos", target_os = "windows"))]
const DEBUG_OUTPUT_DEFAULT: i32 = 2;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const DEBUG_OUTPUT_DEFAULT: i32 = 0;

const USAGE: &str = "bygfoot - a simple and addictive GTK football manager";

/// main.c: `load_last_save` -- whether the last save gets loaded at
/// startup (cl switch `-l`).
static LOAD_LAST_SAVE: Mutex<bool> = Mutex::new(false);

/// variables.h: `country.sid` (cl switch `-c`).
static COUNTRY_SID: Mutex<Option<String>> = Mutex::new(None);

/// main.c: `debug_level` (cl switch `-d`). `debug.c` is not ported yet.
static DEBUG_LEVEL: Mutex<i32> = Mutex::new(DEBUG_LEVEL_DEFAULT);

/// main.c: `debug_output` (cl switch `-o`). `debug.c` is not ported yet.
static DEBUG_OUTPUT: Mutex<i32> = Mutex::new(DEBUG_OUTPUT_DEFAULT);

/// C: the `options` global, filled by `main_init_variables()`.
static OPTIONS: OnceLock<Mutex<OptionsList>> = OnceLock::new();

/// C: the `FileStore` equivalent, created by `main_init_variables()`.
static FILE_STORE: OnceLock<Mutex<FileStore>> = OnceLock::new();

/// The command line options of `main_parse_cl_arguments` (main.c).
///
/// Note: the C program parses the same argv twice (once for the
/// developer debug options, once for the game options). In Rust a
/// single lenient pass over the union of both groups is equivalent,
/// since the debug parser used `ignore_unknown_options` and the GTK
/// option group is consumed internally by `gtk::Application`.
#[derive(Debug, Default)]
struct ClArguments {
    /// `--last-save` / `-l`
    last_save: bool,
    /// `--support-dir` / `-s DIR`
    support_dir: Option<String>,
    /// `--country` / `-c SID`
    country: Option<String>,
    /// `--lang` / `-L CODE`
    lang: Option<String>,
    /// `--testcom` / `-t`
    testcom: bool,
    /// `--commentary-file` / `-C FILE`
    commentary_file: Option<String>,
    /// `--token-file` / `-T FILE`
    token_file: Option<String>,
    /// `--event-name` / `-e EVENTNAME`
    event_name: Option<String>,
    /// `--num-passes` / `-n N`
    num_passes: i32,
    /// `--calodds` / `-O`
    calodds: bool,
    /// `--num-matches` / `-m N`
    num_matches: i32,
    /// `--num-skilldiff` / `-S N`
    num_skilldiff: i32,
    /// `--debug-level` / `-d N` (from `main_parse_debug_cl_arguments`)
    debug_level: i32,
    /// `--debug-output` / `-o N` (from `main_parse_debug_cl_arguments`)
    debug_output: i32,
    /// `--deb` / `-D STRING` (from `main_parse_debug_cl_arguments`)
    deb: Option<String>,
}

impl ClArguments {
    fn new() -> Self {
        Self {
            num_passes: 1,
            num_matches: 100,
            num_skilldiff: 20,
            ..Default::default()
        }
    }
}

/// Read the value following the current option, advancing `index`.
fn option_value(args: &[String], index: &mut usize) -> Option<String> {
    *index += 1;
    args.get(*index).cloned().or_else(|| {
        eprintln!("{USAGE}");
        eprintln!("error: option requires a value");
        None
    })
}

/// Parse an integer option value, falling back to `default` on error.
fn option_int(args: &[String], index: &mut usize, default: i32) -> i32 {
    match option_value(args, index) {
        Some(value) => value.parse::<i32>().unwrap_or(default),
        None => default,
    }
}

/// Parse the command line arguments.
///
/// Mirrors `main_parse_cl_arguments` (main.c) plus the debug option
/// group of `main_parse_debug_cl_arguments`; unknown options are
/// warned about and ignored.
fn parse_cl_arguments(args: &[String]) -> ClArguments {
    let mut parsed = ClArguments::new();
    let mut index = 1;

    while index < args.len() {
        match args[index].as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "-l" | "--last-save" => parsed.last_save = true,
            "-s" | "--support-dir" => parsed.support_dir = option_value(args, &mut index),
            "-c" | "--country" => parsed.country = option_value(args, &mut index),
            "-L" | "--lang" => parsed.lang = option_value(args, &mut index),
            "-t" | "--testcom" => parsed.testcom = true,
            "-C" | "--commentary-file" => parsed.commentary_file = option_value(args, &mut index),
            "-T" | "--token-file" => parsed.token_file = option_value(args, &mut index),
            "-e" | "--event-name" => parsed.event_name = option_value(args, &mut index),
            "-n" | "--num-passes" => parsed.num_passes = option_int(args, &mut index, 1),
            "-O" | "--calodds" => parsed.calodds = true,
            "-m" | "--num-matches" => parsed.num_matches = option_int(args, &mut index, 100),
            "-S" | "--num-skilldiff" => parsed.num_skilldiff = option_int(args, &mut index, 20),
            "-d" | "--debug-level" => parsed.debug_level = option_int(args, &mut index, 0),
            "-o" | "--debug-output" => parsed.debug_output = option_int(args, &mut index, -1),
            "-D" | "--deb" => parsed.deb = option_value(args, &mut index),
            other => {
                // GTK consumes its own options (e.g. --gdk-debug) inside
                // gtk::Application::run, so anything else is foreign.
                eprintln!("warning: ignoring unknown option `{other}`");
            }
        }
        index += 1;
    }

    parsed
}

/// main.c: `main_parse_debug_cl_arguments`.
fn main_parse_debug_cl_arguments(parsed: &ClArguments) {
    *DEBUG_LEVEL.lock().unwrap() = parsed.debug_level;
    if parsed.debug_output != -1 {
        *DEBUG_OUTPUT.lock().unwrap() = parsed.debug_output;
    }
    if parsed.deb.is_some() {
        // C: `statp = debug_text`, shown in the debug window (not ported yet).
        // TODO: port the debug window of window.c / debug.c.
    }
    if parsed.debug_level != -1 {
        // C: window_create(WINDOW_DEBUG) -- not ported yet.
    }
}

/// main.c: `main_parse_cl_arguments` (side effects).
fn main_parse_cl_arguments(parsed: &ClArguments) {
    if parsed.calodds {
        let mut options = OPTIONS.get().unwrap().lock().unwrap();
        option_add(&mut options, "int_opt_calodds", 1);
        option_add(
            &mut options,
            "int_opt_calodds_skilldiffmax",
            parsed.num_skilldiff,
        );
        option_add(&mut options, "int_opt_calodds_matches", parsed.num_matches);
    }

    if parsed.testcom {
        // TODO: port lg_commentary_test (lg_commentary.c)
        eprintln!("warning: --testcom is not implemented yet (lg_commentary.c not ported)");
        main_exit_program(0, None);
    }

    if let Some(support_dir) = parsed.support_dir.as_ref() {
        // C: normalize to a path with a trailing separator, then add the
        // directory (and everything below it) as a support directory.
        let mut full_path = std::path::PathBuf::from(support_dir);
        if !full_path
            .to_string_lossy()
            .ends_with(std::path::MAIN_SEPARATOR)
        {
            full_path.push("");
        }
        file_store::add_support_directory_recursive(full_path);
    }

    if parsed.lang.is_some() {
        // C: language_set(language_get_code_index(lang) + 1);
        //     file_load_hints_file();
        // TODO: port language.c (language_set) and reload the hints.
    }

    if let Some(country_sid) = parsed.country.as_ref() {
        *COUNTRY_SID.lock().unwrap() = Some(country_sid.clone());
    }

    *LOAD_LAST_SAVE.lock().unwrap() = parsed.last_save;
}

/// C: `option_add` (option.c).
fn option_add(options: &mut OptionsList, name: &str, value: i32) {
    let mut option = option::Option::new(name.to_string());
    option.value = value;
    options.push(option);
}

/// main.c: `main_init_variables`.
///
/// The C version nullifies/initializes roughly thirty global variables
/// (leagues, cups, users, windows, ...); as those are ported they get
/// added here. For now: the file store, the options list and the
/// default options added by `main_init_variables` in main.c.
fn main_init_variables() {
    let file_store = FileStore::new();

    let mut options = OptionsList::new();
    option_add(&mut options, "int_opt_calodds", 0);

    // C: "Some of these (or all) are disabled (set to 1) in supernational
    //    country defs."
    let defaults = [
        "int_opt_goto_mode",
        "int_opt_disable_finances",
        "int_opt_disable_transfers",
        "int_opt_disable_stadium",
        "int_opt_disable_contracts",
        "int_opt_disable_boost_on",
        "int_opt_disable_ya",
        "int_opt_disable_training_camp",
    ];
    for name in defaults {
        option_add(&mut options, name, 0);
    }

    // TODO: port file_load_conf_files (file.c) and
    //       xml_strategy_load_strategies (xml_strategy.c); the C version
    //       also computes `current_interest` and calls `language_set`.
    FILE_STORE.set(Mutex::new(file_store)).ok();
    OPTIONS.set(Mutex::new(options)).ok();
}

/// main.c: `main_init`.
fn main_init(args: &[String]) {
    let parsed = parse_cl_arguments(args);
    main_parse_debug_cl_arguments(&parsed);

    // C: `file_add_support_directory_recursive` for the directories below
    //    (the package data dir and $HOME/.bygfoot are added by
    //    FileStore::new inside main_init_variables).
    //    C: on non-Mac builds these come from the *current* directory;
    //    the Rust port uses the executable's parent like file_store.rs.
    if let Ok(cwd) = env::current_dir() {
        file_store::add_support_directory_recursive(cwd.join("support_files"));
        file_store::add_support_directory_recursive(cwd.join("saves"));
    }

    main_init_variables();

    *LOAD_LAST_SAVE.lock().unwrap() = false;
    main_parse_cl_arguments(&parsed);
}

/// The startup decision block of `main()` in main.c.
///
/// C:
/// ```c
/// if((load_last_save && !load_game_from_command_line("last_save")) ||
///         (!load_last_save && (argc == 1 || !load_game_from_command_line(argv[1]))))
/// {
///     if(country.sid == NULL) { splash } else { startup }
/// }
/// ```
pub fn decide_start_window(app: &App) {
    let load_last_save = *LOAD_LAST_SAVE.lock().unwrap();

    // TODO: port load_game_from_command_line (load_save.c)
    let loaded_save = false;

    if (load_last_save && !loaded_save) || !load_last_save {
        if COUNTRY_SID.lock().unwrap().is_none() {
            app.show_splash();
        } else {
            app.show_startup();
        }
    }
}

/// main.c: `main_exit_program` -- exit with the given code and message.
///
/// The C version quits the GTK main loop, frees all memory
/// (`free_memory` in free.c, not ported) and calls `exit`. GTK
/// reference counting destroys the widgets; the equivalent of
/// `gtk_main_quit` is `gtk::Application::quit`.
fn main_exit_program(exit_code: i32, message: Option<&str>) -> ! {
    if let Some(message) = message {
        eprintln!("{message}");
    }
    std::process::exit(exit_code);
}

fn main() -> glib::ExitCode {
    // C: gtk_init + main_init (NLS binding and the Win32 stdout/stderr
    // redirection are skipped in the port for now).
    let args: Vec<String> = env::args().collect();
    main_init(&args);

    let app = App::default();
    app.run()
}
