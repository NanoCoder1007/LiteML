# LiteML
一个用 Rust 实现的轻量级标记语言编译器，将自定义简约语法编译成标准 HTML 页面。

## 安装
**注意**：在此之前，确保您已经安装 **Rust工具链**

```bash
cargo install liteml
```

确认安装
```bash
liteml
```

## 用法
```bash
liteml 源文件.liteml [输出文件.html]
```

## 语法示例
```
page {
    title("标题", level=1);
    title("副标题", level=2);
}
```

## 文档
查看官方[文档](https://nanocoder1007.github.io/nanocoder1007/LiteML/book/index.html)。

## 编译结果
自动生成标准 HTML5 文件。

## 协议
MIT
