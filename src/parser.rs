use anyhow::{anyhow, Result};
use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Title(TitleStmt),
    // 将来可以在这里加入更多语句
}

#[derive(Debug)]
pub struct TitleStmt {
    pub text: String,
    pub level: u8, // 1~6，默认 1
}

/// 解析入口，返回完整的 Program
pub fn parse(tokens: &[Token]) -> Result<Program> {
    let mut cursor = Cursor::new(tokens);
    let mut functions = Vec::new();

    while !cursor.is_eof() {
        functions.push(parse_function(&mut cursor)?);
    }

    Ok(Program { functions })
}

/* ---------- 递归下降实现 ---------- */

struct Cursor<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&'a Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<&'a Token> {
        match self.next() {
            Some(tok) if &tok.kind == kind => Ok(tok),
            Some(tok) => Err(anyhow!(
                "期望 token {:?}，但得到 {:?}（位置 {:?}）",
                kind,
                tok.kind,
                tok.span
            )),
            None => Err(anyhow!("意外的文件结束，期望 token {:?}", kind)),
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        match self.next() {
            Some(Token {
                     kind: TokenKind::Ident(name),
                     ..
                 }) => Ok(name.clone()),
            Some(tok) => Err(anyhow!("期望标识符，但得到 {:?}", tok.kind)),
            None => Err(anyhow!("意外的文件结束，期望标识符")),
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

/* ---------- 语法解析函数 ---------- */

fn parse_function(cursor: &mut Cursor) -> Result<Function> {
    // identifier
    let name = cursor.expect_ident()?;
    // 只允许 index 作为入口，否则报错
    if name != "index" {
        return Err(anyhow!("只能有一个入口函数 `index`，但找到了 `{}`", name));
    }

    // {
    cursor.expect(&TokenKind::LBrace)?;

    // 解析块内部的若干语句
    let mut body = Vec::new();
    while let Some(tok) = cursor.peek() {
        match &tok.kind {
            TokenKind::RBrace => {
                cursor.next(); // consume '}'
                break;
            }
            TokenKind::Ident(_) => {
                body.push(parse_stmt(cursor)?);
            }
            other => {
                return Err(anyhow!(
                    "在函数体内部意外的 token {:?}（位置 {:?}）",
                    other,
                    tok.span
                ));
            }
        }
    }

    Ok(Function { name, body })
}

fn parse_stmt(cursor: &mut Cursor) -> Result<Stmt> {
    // 目前只支持 title(...)
    match cursor.peek() {
        Some(Token {
                 kind: TokenKind::Ident(name),
                 ..
             }) if name == "title" => {
            cursor.next(); // consume "title"
            parse_title_stmt(cursor).map(Stmt::Title)
        }
        Some(tok) => Err(anyhow!(
            "未知语句起始 token {:?}（位置 {:?}）",
            tok.kind,
            tok.span
        )),
        None => Err(anyhow!("意外的文件结束，期待语句")),
    }
}

fn parse_title_stmt(cursor: &mut Cursor) -> Result<TitleStmt> {
    // '('
    cursor.expect(&TokenKind::LParen)?;

    // 第一个必选参数：字符串文字
    let text = match cursor.next() {
        Some(Token {
                 kind: TokenKind::StringLit(s),
                 ..
             }) => s.clone(),
        Some(tok) => {
            return Err(anyhow!(
                "title 第一个参数应为字符串文字，实际得到 {:?}",
                tok.kind
            ))
        }
        None => return Err(anyhow!("意外的文件结束，期待 title 的字符串参数")),
    };

    // 可能的逗号 + 命名参数（目前仅 level）
    let mut level: u8 = 1; // 默认
    if let Some(Token {
                    kind: TokenKind::Comma,
                    ..
                }) = cursor.peek()
    {
        cursor.next(); // consume ','
        // 接下来必须是 "level"
        match cursor.next() {
            Some(Token {
                     kind: TokenKind::Ident(name),
                     ..
                 }) if name == "level" => {}
            Some(tok) => {
                return Err(anyhow!(
                    "title 的命名参数只能是 `level`，但得到 {:?}",
                    tok.kind
                ))
            }
            None => return Err(anyhow!("意外的文件结束，期待 `level` 标识符")),
        };
        // '='
        cursor.expect(&TokenKind::Equals)?;
        // 数字
        let num = match cursor.next() {
            Some(Token {
                     kind: TokenKind::Number(n),
                     ..
                 }) => *n,
            Some(tok) => {
                return Err(anyhow!("`level` 参数右侧应为整数，实际得到 {:?}", tok.kind))
            }
            None => return Err(anyhow!("意外的文件结束，期待 `level` 的数值")),
        };
        // 检查范围
        if !(1..=6).contains(&num) {
            return Err(anyhow!("`level` 参数只能在 1~6 之间，得到 {}", num));
        }
        level = num as u8;
    }

    // ')'
    cursor.expect(&TokenKind::RParen)?;
    // ';'
    cursor.expect(&TokenKind::Semicolon)?;

    Ok(TitleStmt { text, level })
}
