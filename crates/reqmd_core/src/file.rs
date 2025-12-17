use ::reqmd_markdown::ast;
use ::std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct File {
    pub path: Option<PathBuf>,
    pub source: String,
    pub ast: ast::Document,
}
