mod config;
mod i18n;
mod utils;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use config::{init_config, print_config};
use i18n::t;
use utils::{
    filter_logs::filter_logs,
    format_log::{format_log, LogInfo},
    get_repo_logs::get_repo_logs,
    get_repo_name::get_repo_name,
    keypress::exit_on_keypress,
    save_report::save_report_markdown,
};

fn run(root_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize global configuration.
    // After initialization, the config is stored in a global OnceLock singleton,
    // and can be accessed from other modules via `config::store::global()`,
    // such as in the i18n module.
    let config = init_config(&root_path)?;

    print_config();

    let mut result: HashMap<String, HashMap<String, Vec<LogInfo>>> = HashMap::new();

    // Deduplicate logs by repoName and typeName
    let mut dedup_map: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();

    for repo_dir in &config.repos {
        // Get repo name
        let repo_name =
            get_repo_name(repo_dir, &config.format).unwrap_or_else(|| "undefined".to_string());

        let logs = get_repo_logs(repo_dir)?;

        // Append repoName to the beginning of each log line
        let logs_with_repo: Vec<String> = logs
            .into_iter()
            .map(|log| format!("{}|||{}", repo_name, log))
            .collect();

        // Filter logs according to the rules in the configuration file
        let filtered_logs = match filter_logs(
            &logs_with_repo,
            &config.authors,
            &config.includes,
            &config.excludes,
        ) {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "{}",
                    t("err_filter_logs_failed").replace("{}", &e.to_string())
                );
                continue;
            }
        };

        // Formatting and aggregating logs
        for log in filtered_logs {
            let log_info = format_log(&log);
            let type_name = log_info.type_name.clone();

            // get HashSet of type_name in repo_name, if not exist, create a new one
            let type_set = dedup_map
                .entry(repo_name.clone())
                .or_default()
                .entry(type_name.clone())
                .or_default();

            // if message not in type_set, push to result
            if type_set.insert(log_info.message.clone()) {
                result
                    .entry(repo_name.clone())
                    .or_default()
                    .entry(type_name)
                    .or_default()
                    .push(log_info);
            }
        }

        save_report_markdown(&result, &root_path)?;

        exit_on_keypress(Some(t("press_to_exit")));
    }

    Ok(())
}

fn main() {
    // Get the APP_ENV environment variable from the startup command (see Makefile.toml).
    // Since the working directories differ between development and production,
    // we need to handle them separately.
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());

    let root_path = if env == "development" {
        std::env::current_dir().unwrap()
    } else {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    };

    // Print the root path to help locate issues if the program crashes.
    println!("");
    println!("\nProgram root directory:\n{}", root_path.display());

    // Using `?` to propagate errors to `main` causes Rust's default error output
    // (via the `std::process::Termination` trait) to use the Debug format (`{:?}`)
    // for printing error types. As a result, only the enum variant name is shown
    // (e.g., `FileNotFound`), not the user-friendly message from the Display trait.
    //
    // Wrapping the main logic in a separate `run` function and handling errors
    // explicitly with `eprintln!` ensures that the Display output is used.
    if let Err(e) = run(root_path) {
        eprintln!("{}", e); // Use Display format to output the error
        std::process::exit(1);
    }
}
