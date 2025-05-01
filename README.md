# git-commit-analytics

English | [简体中文](https://github.com/analyticsjs/git-commit-analytics/blob/main/README.zh-CN.md)

A tool for analyzing git commit logs and generating daily, weekly, or custom work reports.

![git-commit-analytics](https://cdn.chengpeiquan.com/img/2025/05/202505020137671.gif)

## 🚀 Download

This is a client tool, so you need to download the program to use it. See: [The Latest Release](https://github.com/analyticsjs/git-commit-analytics/releases/latest) to download.

> Note: This tool requires Git to be installed and properly configured in your system's PATH. Please make sure Git is installed before running the program.

## ⚡ Usage

Create and fill in your configuration file, and then run the program to get your work report.

## 📂 Configuration File

You need to create a `config.json` at the same folder with the program, and write the content in the following format.

```json
{
  "lang": "en",
  "authors": ["my-name"],
  "dateRange": ["2025-04-01", "2025-05-01"],
  "repos": ["/path/to/my-project-folder"],
  "format": {
    "my-project-folder": "My Awesome Project"
  },
  "includes": ["feat", "fix", "docs", "style", "refactor", "test", "chore"],
  "excludes": ["typo", "backup"]
}
```

**NOTE：** Please configure the repos path according to your operating system. For example:

- On Windows, you must use backslashes (\), e.g., `D:\\path\\to\\folder-name`
- On macOS, use forward slashes (/), e.g., `/path/to/folder-name`

The configuration items are described as follows:

|    key    |           type            | description                                                                                                                                                                                 |
| :-------: | :-----------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|   lang    |          string           | Set program default language, support `en` (English, default) and `zh` (Simplified Chinese).                                                                                                |
|  authors  |         string[]          | Filter commit authors. Supports multiple author names, useful if you use different names in different repositories.                                                                         |
| dateRange |     [string, string]      | Specify [start date, end date]. Supports valid date formats. The statistics cover from `00:00:00` of the start date to `23:59:59` of the end date. If not set, defaults to the current day. |
|   repos   |         string[]          | The Git repository folders on your computer. Please switch to the branch you want to analyze in advance.                                                                                    |
|  format   | { [key: string]: string } | Format your folder names as project names.                                                                                                                                                  |
| includes  |         string[]          | Commit message prefixes to include in the statistics.                                                                                                                                       |
| excludes  |         string[]          | Exclude commit messages containing these keywords from the results.                                                                                                                         |

Among them, `authors` / `includes` / `excludes` will be created as regular expressions to match data.

## 📚 Report File

The report file will be generated in `markdown` syntax (probably the most common format for developer?) and saved as a file in `.txt` format (probably the most compatible format?).

The project name will be classified as the second-level title, and 7 types of commit prefixes will be classified as the third-level title:

|   type   |   description   |
| :------: | :-------------: |
|   feat   |    Features     |
|   fix    |    Bug Fixes    |
|   docs   |  Documentation  |
|  style   | Optimized Style |
| refactor |   Refactored    |
|   test   |   Test Cases    |
|  chore   |     Chores      |

You can click [Commit message and Change log writing guide](https://www.ruanyifeng.com/blog/2016/01/commit_message_change_log.html) to learn how to standardize the git commit.

## 📝 Release Notes

Please refer to [CHANGELOG](./CHANGELOG.md) for details.

## 📜 License

[MIT License](./LICENSE) © 2022 [chengpeiquan](https://github.com/chengpeiquan)
