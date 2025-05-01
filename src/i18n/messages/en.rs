use phf::phf_map;

pub static MESSAGES: phf::Map<&'static str, &'static str> = phf_map! {
  // Git
  "err_git_not_found" => "Git is not installed or not in system PATH. Please install Git first.\nVisit https://git-scm.com/downloads to download.",
  "err_git_not_working" => "Git is installed but not working properly. Please check your Git installation.\nTo reinstall, visit https://git-scm.com/downloads .",

  // Configuration file I/O
  "config_loaded" => "Configuration loaded successfully",
  "failed_print_config" => "Failed to print configuration: {}",
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

    // Commit categories
    "commit_category_features" => "Features",
    "commit_category_bug_fixes" => "Bug Fixes",
    "commit_category_docs" => "Documentation",
    "commit_category_style" => "Optimized Style",
    "commit_category_refactor" => "Refactored",
    "commit_category_test" => "Test Cases",
    "commit_category_chores" => "Chores",

  // Git repository error messages
  "err_repo_not_found" => "Repo path does not exist or is not a directory: {}",
  "err_git_log_failed" => "git log failed in directory: {}\nstderr: {}",

  // Save report
  "no_report_generated" => "No report generated",
  "err_save_report_failed" => "Failed to save report: {}",
  "report_saved" => "Report saved to {}",
};
