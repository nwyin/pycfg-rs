use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct Edge {
    pub target: usize,
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Statement {
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BasicBlock {
    pub id: usize,
    pub label: String,
    pub statements: Vec<Statement>,
    pub successors: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Metrics {
    pub blocks: usize,
    pub edges: usize,
    pub branches: usize,
    pub cyclomatic_complexity: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionCfg {
    pub name: String,
    pub line: usize,
    pub blocks: Vec<BasicBlock>,
    pub metrics: Metrics,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileCfg {
    pub file: String,
    pub functions: Vec<FunctionCfg>,
}

impl Metrics {
    pub(crate) fn compute(blocks: &[BasicBlock]) -> Self {
        let num_blocks = blocks.len();
        let num_edges: usize = blocks.iter().map(|b| b.successors.len()).sum();
        let branches = blocks.iter().filter(|b| b.successors.len() > 1).count();
        let cyclomatic = if num_blocks == 0 {
            1
        } else {
            (num_edges as isize - num_blocks as isize + 2).max(1) as usize
        };
        Metrics {
            blocks: num_blocks,
            edges: num_edges,
            branches,
            cyclomatic_complexity: cyclomatic,
        }
    }
}

impl fmt::Display for FunctionCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "def {}:", self.name)?;
        writeln!(f)?;
        for block in &self.blocks {
            if block.label == "entry" || block.label == "exit" {
                write!(f, "  Block {} ({}):", block.id, block.label)?;
            } else {
                write!(f, "  Block {}:", block.id)?;
            }
            writeln!(f)?;
            for stmt in &block.statements {
                writeln!(f, "    [L{}] {}", stmt.line, stmt.text)?;
            }
            for edge in &block.successors {
                writeln!(f, "    -> Block {} [{}]", edge.target, edge.label)?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "  # blocks={} edges={} branches={} cyclomatic_complexity={}",
            self.metrics.blocks,
            self.metrics.edges,
            self.metrics.branches,
            self.metrics.cyclomatic_complexity
        )
    }
}
