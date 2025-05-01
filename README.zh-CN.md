# git-commit-analytics

[English](https://github.com/analyticsjs/git-commit-analytics/blob/main/README.md) | 简体中文

一款用于分析 Git 提交日志并生成每日、每周或自定义工作报告的工具。

![git-commit-analytics](https://cdn.chengpeiquan.com/img/2025/05/202505020137671.gif)

## 🚀 下载安装

这是一个客户端工具，所以你需要下载程序去使用它，点击 [最新版本](https://github.com/analyticsjs/git-commit-analytics/releases/latest) 去下载客户端。

> 注意： 本工具依赖于已安装并配置好的 Git，请确保在运行前已正确安装 Git 并将其添加到环境变量中。

## ⚡ 使用方法

创建并填写你的配置文件，然后运行程序，即可获得你的工作报告。

## 📂 配置文件

需要在与程序相同的文件夹下，创建一个名为 `config.json` 的文件，并写入以下格式的内容。

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

**提醒：** 请根据您的操作系统正确配置 repos 字段的路径。例如：

- 在 Windows 系统中，路径必须使用反斜杠（\），如：`D:\\path\\to\\folder-name`
- 在 macOS 下，请使用正斜杠（/），如：`/path/to/folder-name`

配置项说明如下：

|    key    |           type            | description                                                                                                                                  |
| :-------: | :-----------------------: | :------------------------------------------------------------------------------------------------------------------------------------------- |
|   lang    |          string           | 设置软件的默认语言，支持 `en` （英语，默认值）和 `zh` （简体中文）。                                                                         |
|  authors  |         string[]          | 筛选 commit 的作者名称，支持多个作者名称，用于你在不同的仓库可能有不同的名字。                                                               |
| dateRange |     [string, string]      | 填写 [开始日期， 结束日期] ， 支持合法的时间格式，会从开始日期的 `00:00:00` 统计到截止日期的 `23:59:59` （如果不配置则默认运行程序的当天）。 |
|   repos   |         string[]          | 你电脑里的 Git 仓库文件夹，需要提前切换到你要统计的分支。                                                                                    |
|  format   | { [key: string]: string } | 格式化你的文件夹名称为项目名。                                                                                                               |
| includes  |         string[]          | 要纳入统计的 commit message 前缀。                                                                                                           |
| excludes  |         string[]          | 在统计出来的结果里，排除掉包含了这些关键词的 commit message 。                                                                               |

其中，`authors` / `includes` / `excludes` 会创建为正则表达式去匹配数据。

## 📚 报告文件

报告文件会以 `markdown` 语法生成（可能是对程序员最通用的格式？），并以 `.txt` 格式的文件保存（可能是兼容性最好的格式？）。

会以项目名称作为二级标题归类，以 7 个类型的 commit 前缀作为三级标题归类：

|   type   | description |
| :------: | :---------: |
|   feat   |  功能开发   |
|   fix    |  BUG 修复   |
|   docs   |  完善文档   |
|  style   |  优化样式   |
| refactor |  代码重构   |
|   test   |  测试用例   |
|  chore   |  其他优化   |

可以点击 [Commit message 和 Change log 编写指南](https://www.ruanyifeng.com/blog/2016/01/commit_message_change_log.html) 学习如何规范化提交 Git Commit 。

## 📝 Release Notes

详细更新内容请参考 [更新记录](./CHANGELOG.md) 。

## 📜 License

[MIT License](./LICENSE) © 2022 [chengpeiquan](https://github.com/chengpeiquan)
