use phf::phf_map;

pub static MESSAGES: phf::Map<&'static str, &'static str> = phf_map! {

    // Configuration file I/O
    "config_loaded" => "Configuration loaded successfully",
    "config_not_found" => "Configuration file not found",
    "failed_init_config" => "Failed to initialize configuration: {}",
    "failed_print_config" => "Failed to print configuration: {}",
    "config_file_not_found" => "Config file not found: {}",
    "failed_parse_config" => "Failed to parse config: {}",
    "global_config_not_initialized" => "Global configuration not initialized",

    // Configuration file details
    "config_details" => "---- Configuration Details -----",
    "language" => "Language",
    "authors" => "Authors",
    "date_range" => "Date Range",
    "repos" => "Repositories",
    "includes" => "Includes",
    "excludes" => "Excludes",
    "format" => "Format",

    // Keypress
    "wait_for_key" => "Press any key to continue...",
    "press_to_exit" => "Press any key to exit...",
};
