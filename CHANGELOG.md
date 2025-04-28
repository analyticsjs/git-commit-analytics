## [2.0.2](https://github.com/analyticsjs/git-commit-analytics/compare/v2.0.1...v2.0.2) (2025-04-28)


### Bug Fixes

* **report:** deduplicate commit messages within each group in the report ([e669454](https://github.com/analyticsjs/git-commit-analytics/commit/e669454aaedbf02a2b2db495a9da0b3de3a5908c))
* **windows:** statically link MSVC CRT to fix missing vcruntime140.dll issue ([097997e](https://github.com/analyticsjs/git-commit-analytics/commit/097997e740a1a333ebd51e16f7a65fd3e9e69c96))

## [2.0.1](https://github.com/analyticsjs/git-commit-analytics/compare/v2.0.0...v2.0.1) (2025-04-28)


### Bug Fixes

* **release:** upload built artifacts with versioned filenames to GitHub assets ([98616fc](https://github.com/analyticsjs/git-commit-analytics/commit/98616fcdb6eff5cb75b4dbc1ae4f2143fe86692d))

# [2.0.0](https://github.com/analyticsjs/git-commit-analytics/compare/v1.5.1...v2.0.0) (2025-04-27)


* feat!: rewrite project in Rust ([3540027](https://github.com/analyticsjs/git-commit-analytics/commit/3540027a75cef19fa4a804391fd34bdb305a9d5f))


### Features

* **config:** support config file reading and global access ([7f8d2ad](https://github.com/analyticsjs/git-commit-analytics/commit/7f8d2adc7749329905230f7e443f515213c5d8b0))
* env-based root path selection & extract config constants ([f08b360](https://github.com/analyticsjs/git-commit-analytics/commit/f08b3607beb232277f427654fec310a8897b2df7))
* **i18n:** add basic internationalization support ([1c2c14c](https://github.com/analyticsjs/git-commit-analytics/commit/1c2c14cdceb3d6d4d86f9b9d89f9acc8ab9bd1e8))
* **main:** implement main process with global config, error handling, and report generation ([8d9f7a8](https://github.com/analyticsjs/git-commit-analytics/commit/8d9f7a8f31e9a56a8726beed8c6c41e8c92e5963))
* **utils:** add get_repo_name function with robust path handling and tests ([8c177a6](https://github.com/analyticsjs/git-commit-analytics/commit/8c177a6e699110eecdc5287d060da3a6fb23652e))
* **utils:** add keyboard interaction utilities ([c5b8b2c](https://github.com/analyticsjs/git-commit-analytics/commit/c5b8b2c1ba4a4ecc1e8f2862f6d894bd84eefc38))
* **utils:** add save_report_markdown for generating i18n-friendly Markdown reports ([7a3b551](https://github.com/analyticsjs/git-commit-analytics/commit/7a3b55107f2890425c6d83c8d8a4a67d924be026))
* **utils:** implement filter_logs to filter submission records of specified rules ([75e874b](https://github.com/analyticsjs/git-commit-analytics/commit/75e874b87a7578cf9941a350ebf8e97a1f4380d7))
* **utils:** implement format_commit function with unit tests ([4cd4fc5](https://github.com/analyticsjs/git-commit-analytics/commit/4cd4fc5b4d04d61655310784f8eb1543706a4b26))
* **utils:** implement format_log function for parsing and structuring git log lines ([f3a7379](https://github.com/analyticsjs/git-commit-analytics/commit/f3a737954fc65a3e1d45f48cea1085dcb7cb7bae))
* **utils:** implement get_repo_logs for cross-platform git log retrieval ([d2178d0](https://github.com/analyticsjs/git-commit-analytics/commit/d2178d026f85ed050c596b5e7000a73ea5bbd603))


### BREAKING CHANGES

* The entire project has been refactored and rewritten in Rust.
Previous JavaScript implementation and related files have been removed.
All usage, configuration, and build processes are now based on the Rust version.
