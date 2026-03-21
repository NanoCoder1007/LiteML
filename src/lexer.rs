use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Equals,
    StringLit(String),
    Number(u32),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: std::ops::Range<usize>, // 位置，仅用于错误报告
}

/// 使用单一正则一次性切分全部 token，跳过空白和注释
pub fn lex(input: &str) -> Result<Vec<Token>> {
    // 只匹配我们需要的 token，顺序必须和下面的捕获组对应
    let re = Regex::new(
        r#"(?x)
        (?P<ws>\s+)                         # 空白
        |(?P<comment>//[^\n]*)              # 行注释（可选）
        |(?P<ident>[A-Za-z_][A-Za-z0-9_]*)  # 标识符
        |(?P<lbrace>\{)                      # {
        |(?P<rbrace>\})                      # }
        |(?P<lparen>\()                      # (
        |(?P<rparen>\))                      # )
        |(?P<semicolon>;)                   # ;
        |(?P<comma>,)                        # ,
        |(?P<equals>=)                       # =
        |(?P<number>\d+)                    # 整数
        |(?P<string>"([^"\\]|\\.)*")         # 双引号字符串
    "#,
    )
        .unwrap();

    let mut tokens = Vec::new();

    let mut pos = 0;
    while pos < input.len() {
        // find 从当前位置开始的下一个匹配
        let mat = re
            .find_at(input, pos)
            .ok_or_else(|| anyhow!("无法在位置 {} 之后继续词法分析", pos))?;

        // ensure 连续匹配（不允许跳过未匹配字符）
        if mat.start() != pos {
            return Err(anyhow!(
                "非法字符 '{}' 在位置 {}",
                &input[pos..mat.start()],
                pos
            ));
        }

        // 读取捕获的子组
        let caps = re.captures(&input[pos..mat.end()]).unwrap();
        let token = if caps.name("ws").is_some() || caps.name("comment").is_some() {
            // 跳过
            None
        } else if let Some(m) = caps.name("ident") {
            Some(Token {
                kind: TokenKind::Ident(m.as_str().to_string()),
                span: pos..pos + m.end(),
            })
        } else if caps.name("lbrace").is_some() {
            Some(Token {
                kind: TokenKind::LBrace,
                span: pos..pos + 1,
            })
        } else if caps.name("rbrace").is_some() {
            Some(Token {
                kind: TokenKind::RBrace,
                span: pos..pos + 1,
            })
        } else if caps.name("lparen").is_some() {
            Some(Token {
                kind: TokenKind::LParen,
                span: pos..pos + 1,
            })
        } else if caps.name("rparen").is_some() {
            Some(Token {
                kind: TokenKind::RParen,
                span: pos..pos + 1,
            })
        } else if caps.name("semicolon").is_some() {
            Some(Token {
                kind: TokenKind::Semicolon,
                span: pos..pos + 1,
            })
        } else if caps.name("comma").is_some() {
            Some(Token {
                kind: TokenKind::Comma,
                span: pos..pos + 1,
            })
        } else if caps.name("equals").is_some() {
            Some(Token {
                kind: TokenKind::Equals,
                span: pos..pos + 1,
            })
        } else if let Some(m) = caps.name("number") {
            let num: u32 = m.as_str().parse().unwrap();
            Some(Token {
                kind: TokenKind::Number(num),
                span: pos..pos + m.end(),
            })
        } else if let Some(m) = caps.name("string") {
            // 去掉首尾双引号，并处理基本转义（只处理 \" 和 \\）
            let raw = &m.as_str()[1..m.as_str().len() - 1];
            let unescaped = raw
                .replace(r#"\""#, "\"")
                .replace(r#"\\"#, r"\");
            Some(Token {
                kind: TokenKind::StringLit(unescaped),
                span: pos..pos + m.end(),
            })
        } else {
            return Err(anyhow!(
                "未匹配的 token 在位置 {}: '{}'",
                pos,
                &input[pos..mat.end()]
            ));
        };

        if let Some(tok) = token {
            tokens.push(tok);
        }

        pos = mat.end();
    }

    Ok(tokens)
}
