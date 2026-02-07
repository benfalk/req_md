use ::reqmd_ast as ast;
use ::std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct File {
    path: Option<PathBuf>,
    source: String,
    ast: ast::Document,
}

impl File {
    /// Loads a Markdown String and Optional Path
    ///
    /// ````rust
    /// let md_content = r##"
    /// # Sample Request
    ///
    /// This is a sample markdown file request.
    ///
    /// ```http
    /// POST /api/widget
    /// Content-Type: application/json
    /// ```
    /// ```json
    /// {
    ///     "name": "Some Widget",
    ///     "description": "This is my widget"
    /// }
    /// ```
    /// "##;
    ///
    /// let file = ::reqmd_core::File::load(md_content, None).unwrap();
    /// assert!(file.path().is_none());
    /// ````
    /// ---
    pub fn load(
        source: impl Into<String>,
        path: Option<PathBuf>,
    ) -> Result<Self, ::reqmd_ast::Error> {
        let source = source.into();
        let ast = ::reqmd_ast::parse_markdown(&source)?;
        Ok(Self { path, source, ast })
    }

    /// Location of the file, if any
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// the original source of the markdown file
    pub fn source(&self) -> &str {
        &self.source
    }

    /// raw parsed AST of the markdown file
    pub fn document(&self) -> &ast::Document {
        &self.ast
    }
}
