# LiteML
一个用 Rust 实现的轻量级标记语言编译器，将自定义简约语法编译成标准 HTML 页面。

## 编译
```bash
cargo build --release
```
生成的可执行文件在 `target/release/liteml`

## 用法
```bash
cargo run -- 源文件.liteml [输出文件.html]
```

## 语法示例
```
page {
    title("标题", level=1);
    title("副标题", level=2);
}
```

## 编译结果
自动生成标准 HTML5 文件。

## 协议
MIT
