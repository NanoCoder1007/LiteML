use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::fs;

mod lexer;
mod parser;
mod codegen;

/// 命令行参数
#[derive(Parser, Debug)]
#[command(name = "lite_ml")]
#[command(author = "Your Name")]
#[command(version = "0.1")]
#[command(about = "LiteML → HTML 编译器")]
struct Cli {
    /// 输入文件（必须是 .ltml）
    input: PathBuf,

    /// 可选的输出文件路径（默认同名 .html）
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. 检查后缀名（改为 .ltml）
    if cli.input.extension().and_then(|s| s.to_str()) != Some("ltml") {
        anyhow::bail!("输入文件必须是 .ltml 结尾");
    }

    // 2. 读取源码
    let src = fs::read_to_string(&cli.input)?;

    // 3. 词法分析
    let tokens = lexer::lex(&src)?;

    // 4. 解析成 AST
    let ast = parser::parse(&tokens)?;

    // 5. 生成 HTML 文本
    let html = codegen::generate_html(&ast);

    // 6. 确定输出路径
    let out_path: PathBuf = match cli.output {
        Some(p) => p,
        None => {
            // 默认把 .ltml 替换为 .html
            let mut p = cli.input.clone();
            p.set_extension("html");
            p
        }
    };

    // 7. 写入文件
    fs::write(&out_path, html)?;
    println!("已生成 {}", out_path.display());

    Ok(())
}
