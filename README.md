# git-commit-analytics

A tool to analyze your git repository's commit log. I can help you generate daily/weekly or longer work reports.

一个可以分析你的 Git 仓库 commit 记录的工具。它可以帮你生成一份工作日报 / 周报，或者你需要的更长时间范围的工作报告。

## Download

This is a client tool, so you need to download the program to use it. See: [The Latest Release](https://github.com/analyticsjs/git-commit-analytics/releases/latest) to download.

这是一个客户端工具，所以你需要下载程序去使用它，点击 [最新版本](https://github.com/analyticsjs/git-commit-analytics/releases/latest) 去下载客户端。

## Change Log

You can look at the [CHANGELOG](./CHANGELOG.md) to understand the content of each update. 

你可以查看 [更新记录](./CHANGELOG.md) 去了解每个版本的更新内容。

## Usage

Create and fill in your configuration file, and then run the program to get your work report.

创建并填写你的配置文件，然后运行程序，即可获得你的工作报告。

## Configuration

You need to create a `config.json` at the same folder with the program, and write the content in the following format.

你需要在与程序相同的文件夹下，创建一个名为 `config.json` 的文件，并写入以下格式的内容。

```json
{
  "lang": "en",
  "authors": [
    "chengpeiquan"
  ],
  "dateRange": [
    "2021-12-01",
    "2022-01-31"
  ],
  "repos": [
    "D:\\Git\\git-commit-analytics"
  ],
  "format": {
    "git-commit-analytics": "Git Commit Analytics"
  },
  "includes": [
    "feat",
    "fix",
    "docs",
    "style",
    "refactor",
    "test",
    "chore"
  ],
  "excludes": [
    "typo",
    "backup",
    "progress"
  ]
}
```

The configuration items are described as follows:

配置项说明如下：

key|type|description
:-:|:-:|:-:
lang|string|Set program default language, support `en` (English) and `zh` (Simplified Chinese).<br>设置软件的默认语言，支持 `en` （英语）和 `zh` （简体中文）。
authors|string[]|Filter the author name of commits, support multiple author names, for you may have different names in different repos.<br>筛选 commit 的作者名称，支持多个作者名称，用于你在不同的仓库可能有不同的名字。
dateRange|[string, string]|Fill in [start date, end date], support the legal time format, and count from the start date `00:00:00` to the end date `23:59:59`.<br>填写 [开始日期， 结束日期] ， 支持合法的时间格式，会从开始日期的 `00:00:00` 统计到截止日期的 `23:59:59` 。
repos|string[]|The Git repo folder on your computer, need to be switched to the branch you want to count.<br>你电脑里的 Git 仓库文件夹，需要提前切换到你要统计的分支。
format|{ [key: string]: string }|Format your folder name as the project name.<br>格式化你的文件夹名称为项目名。
includes|string[]|The commit message prefix to be included in the statistics.<br>要纳入统计的 commit message 前缀。
excludes|string[]|In the statistical results, exclude commit messages that contain these keywords.<br>在统计出来的结果里，排除掉包含了这些关键词的 commit message 。

Among them, `authors` / `includes` / `excludes` will be created as regular expressions to match data.

其中，`authors` / `includes` / `excludes` 会创建为正则表达式去匹配数据。

## License

[MIT License](./LICENSE) © 2019 [chengpeiquan](https://github.com/chengpeiquan)
