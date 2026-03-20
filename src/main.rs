use std::env;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result, anyhow};
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: liteml <源文件.liteml> [输出文件.html]");
        eprintln!("示例: liteml hello.liteml");
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let source = fs::read_to_string(input_path)
        .with_context(|| format!("无法读取文件: {}", input_path.display()))?;

    // 解析并编译
    let elements = parse_page(&source)?;
    let html = compile_html(&elements);

    // 确定输出文件名
    let output_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        input_path.with_extension("html").to_string_lossy().to_string()
    };

    // 写入文件
    fs::write(&output_path, html)
        .with_context(|| format!("无法写入文件: {}", output_path))?;

    println!("编译成功: {}", output_path);

    Ok(())
}

#[derive(Debug)]
struct TitleElement {
    text: String,
    level: u32,
}

fn parse_page(source: &str) -> Result<Vec<TitleElement>> {
    // 查找 page { ... } 块
    let page_re = Regex::new(r"(?s)page\s*\{\s*(.*?)\s*}")
        .map_err(|e| anyhow!("正则表达式错误: {}", e))?;

    let page_caps = page_re.captures(source)
        .ok_or_else(|| anyhow!("未找到有效的 page 块"))?;

    // 清理首尾空白
    let page_content = page_caps[1].trim();

    // 匹配所有的 title 声明（支持多行）
    let title_re = Regex::new(
        r#"title\s*\(\s*"([^"]+)"\s*(?:,\s*level\s*=\s*(\d+))?\s*\)\s*;"#
    ).map_err(|e| anyhow!("正则表达式错误: {}", e))?;

    let mut elements = Vec::new();

    // 查找所有匹配的 title
    for caps in title_re.captures_iter(page_content) {
        let text = caps[1].to_string();

        // 解析 level 属性，默认为 1
        let level = match caps.get(2) {
            Some(level_match) => {
                let level_num: u32 = level_match.as_str().parse()
                    .map_err(|_| anyhow!("level 必须是数字"))?;

                if level_num < 1 || level_num > 6 {
                    return Err(anyhow!("level 必须在 1 到 6 之间，当前值: {}", level_num));
                }
                level_num
            }
            None => 1,
        };

        elements.push(TitleElement { text, level });
    }

    if elements.is_empty() {
        return Err(anyhow!("page 块中没有找到任何 title 声明"));
    }

    Ok(elements)
}

fn compile_html(elements: &[TitleElement]) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"zh-CN\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    // 用第一个 title 作为页面标题
    if let Some(first_title) = elements.first() {
        html.push_str(&format!("    <title>{}</title>\n", first_title.text));
    } else {
        html.push_str("    <title>LiteML Document</title>\n");
    }
    html.push_str("</head>\n");
    html.push_str("<body>\n");

    // 编译所有元素
    for elem in elements {
        html.push_str(&format!("    <h{1}>{0}</h{1}>\n", elem.text, elem.level));
    }

    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}