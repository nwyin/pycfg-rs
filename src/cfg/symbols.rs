use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

use super::source_map::line_from_offset;

pub(crate) struct FunctionVisit<'a> {
    pub(crate) qualified_name: String,
    pub(crate) leaf_name: String,
    pub(crate) line: usize,
    pub(crate) body: &'a [Stmt],
}

pub(crate) fn visit_functions<'a, F>(source: &str, stmts: &'a [Stmt], visit: &mut F)
where
    F: FnMut(FunctionVisit<'a>),
{
    visit_scope(source, stmts, "", visit);
}

fn visit_scope<'a, F>(source: &str, stmts: &'a [Stmt], prefix: &str, visit: &mut F)
where
    F: FnMut(FunctionVisit<'a>),
{
    for stmt in stmts {
        match stmt {
            Stmt::FunctionDef(func_def) => {
                let qualified_name = qualify_name(prefix, func_def.name.as_str());
                visit(FunctionVisit {
                    qualified_name: qualified_name.clone(),
                    leaf_name: func_def.name.to_string(),
                    line: line_from_offset(source, func_def.range().start()),
                    body: &func_def.body,
                });
                visit_scope(source, &func_def.body, &qualified_name, visit);
            }
            Stmt::ClassDef(class_def) => {
                let class_prefix = qualify_name(prefix, class_def.name.as_str());
                visit_scope(source, &class_def.body, &class_prefix, visit);
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
