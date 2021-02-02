use hir::{Function, Semantics};
use ide_db::base_db::FilePosition;
use ide_db::RootDatabase;
use syntax::{
    algo::{ancestors_at_offset, find_node_at_offset},
    ast, AstNode,
};

// Feature: View Definitions At Cursor
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: View Definitions At Cursor**
// |===
pub(crate) fn view_def_map(db: &RootDatabase, position: FilePosition) -> String {
    def_map_at(db, position).unwrap_or("Not inside a function body".to_string())
}

fn def_map_at(db: &RootDatabase, position: FilePosition) -> Option<String> {
    let sema = Semantics::new(db);
    let source_file = sema.parse(position.file_id);

    let node = ancestors_at_offset(source_file.syntax(), position.offset).next()?;
    let krate = sema.scope(&node).krate()?;

    Some(krate.debug_def_map(db))
}
