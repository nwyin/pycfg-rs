use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

use super::source_map::LineIndex;

pub(crate) struct FunctionVisit<'a> {
    pub(crate) qualified_name: String,
    pub(crate) line: usize,
    pub(crate) body: &'a [Stmt],
}

pub(crate) fn visit_functions<'a, F>(line_index: &LineIndex, stmts: &'a [Stmt], visit: &mut F)
where
    F: FnMut(FunctionVisit<'a>),
{
    visit_scope(line_index, stmts, "", visit);
}

fn visit_scope<'a, F>(line_index: &LineIndex, stmts: &'a [Stmt], prefix: &str, visit: &mut F)
where
    F: FnMut(FunctionVisit<'a>),
{
    for stmt in stmts {
        match stmt {
            Stmt::FunctionDef(func_def) => {
                let qualified_name = qualify_name(prefix, func_def.name.as_str());
                visit(FunctionVisit {
                    qualified_name: qualified_name.clone(),
                    line: line_index.line_from_offset(func_def.range().start()),
                    body: &func_def.body,
                });
                visit_scope(line_index, &func_def.body, &qualified_name, visit);
            }
            Stmt::ClassDef(class_def) => {
                let class_prefix = qualify_name(prefix, class_def.name.as_str());
                visit_scope(line_index, &class_def.body, &class_prefix, visit);
            }
            _ => {}
        }
    }
}

fn qualify_name(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{prefix}.{name}")
    }
}
