# Rust 文件追加换行工具

> 递归获取指定目录下所有文件，如当前文件最后一行不存在换行，则添加换行。

config.toml

```toml
# 处理文件目录
path = "/CLionProjects/rust-record/file-append-newline"
# 排除文件前缀
exclude_prefix = ["target"]
# 排除文件后缀
exclude_suffix = ["lock"]
```
