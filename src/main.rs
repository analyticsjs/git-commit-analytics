mod config;
mod i18n;
mod utils;

use std::collections::HashMap;

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

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize global configuration.
    // After initialization, the config is stored in a global OnceLock singleton,
    // and can be accessed from other modules via `config::store::global()`,
    // such as in the i18n module.
    let config = init_config()?;

    print_config();

    let mut result: HashMap<String, HashMap<String, Vec<LogInfo>>> = HashMap::new();

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
            result
                .entry(repo_name.clone())
                .or_default()
                .entry(type_name)
                .or_default()
                .push(log_info);
        }

        // save_report_markdown(&result, "report.txt").expect(t("report_gen_error"));
        save_report_markdown(&result, "report.txt")?;

        exit_on_keypress(Some(t("press_to_exit")));
    }

    Ok(())
}

fn main() {
    // Print the current working directory
    // When the program crashes, it can help locate the cause
    println!("");
    println!(
        "Current working directory: \n{}",
        &std::env::current_dir().unwrap().display()
    );

    // Using `?` to propagate errors to `main` causes Rust's default error output
    // (via the `std::process::Termination` trait) to use the Debug format (`{:?}`)
    // for printing error types. As a result, only the enum variant name is shown
    // (e.g., `FileNotFound`), not the user-friendly message from the Display trait.
    //
    // Wrapping the main logic in a separate `run` function and handling errors
    // explicitly with `eprintln!` ensures that the Display output is used.
    if let Err(e) = run() {
        eprintln!("{}", e); // Use Display format to output the error
        std::process::exit(1);
    }
}
