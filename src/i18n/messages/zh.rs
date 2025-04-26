use phf::phf_map;

pub static MESSAGES: phf::Map<&'static str, &'static str> = phf_map! {
    // Configuration file I/O
    "config_loaded" => "配置加载成功",
    "failed_print_config" => "打印配置失败：{}",
    "failed_parse_config" => "解析配置失败：{}",
    "global_config_not_initialized" => "全局配置未初始化",

    // Configuration file details
    "config_details" => "----------- 配置详情 -----------",
    "language" => "语言",
    "authors" => "作者",
    "date_range" => "日期范围",
    "repos" => "仓库",
    "includes" => "包含",
    "excludes" => "排除",
    "format" => "格式",

    // Keypress
    "wait_for_key" => "按任意键继续...",
    "press_to_exit" => "按任意键退出...",

    // Commit categories
    "commit_category_features" => "功能开发",
    "commit_category_bug_fixes" => "BUG修复",
    "commit_category_docs" => "完善文档",
    "commit_category_style" => "优化样式",
    "commit_category_refactor" => "代码重构",
    "commit_category_test" => "测试用例",
    "commit_category_chores" => "其他优化",

    // Git repository error messages
    "err_repo_not_found" => "仓库路径不存在或不是目录：{}",
    "err_git_log_failed" => "git log 执行失败，目录：{}\n错误信息：{}",

    // Save report
    "no_report_generated" => "未生成报告",
    "err_save_report_failed" => "保存报告失败：{}",
    "report_saved" => "报告已保存到：{}",
};
