use crate::parser::{Program, Stmt, TitleStmt};

/// 生成完整的 HTML 文本
pub fn generate_html(prog: &Program) -> String {
    // 本语言只允许一个函数（index），所以直接取第一个
    let func = &prog.functions[0];
    let func_name = &func.name; // 现在真的用了

    // 收集 body 中所有标题
    let mut body = String::new();
    let mut first_title: Option<String> = None;

    for stmt in &func.body {
        match stmt {
            Stmt::Title(TitleStmt { text, level }) => {
                // level 已在解析阶段检查，这里再 clamp 以防万一
                let lvl = (*level).clamp(1, 6);
                body.push_str(&format!("<h{lvl}>{text}</h{lvl}>\n", lvl = lvl, text = text));
                if first_title.is_none() {
                    first_title = Some(text.clone());
                }
            }
        }
    }

    // <title> 使用第一次出现的标题文字；若标题为空则用默认文本
    let title_text = first_title.unwrap_or_else(|| "LiteML".to_string());

    // 在 head 中加入 generator meta，展示入口函数名（展示 name 被使用）
    let generator_meta = format!(r#"<meta name="generator" content="LiteML {}">"#, func_name);

    format!(
        "<!DOCTYPE html>\n\
        <html>\n\
        <head>\n\
            <meta charset=\"utf-8\">\n\
            {generator_meta}\n\
            <title>{title}</title>\n\
        </head>\n\
        <body>\n\
        {body}\
        </body>\n\
        </html>\n",
        generator_meta = generator_meta,
        title = title_text,
        body = body,
    )
}
