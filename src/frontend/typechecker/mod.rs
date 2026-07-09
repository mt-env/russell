// decorates the AST such that each node gets a TypeId, which indexes into an arena of Types, with
// constraints decided by the Context
pub mod inference;

pub mod types;

// turns an AST decorated with TypeIds into an AST decorated with concrete TypeValues, by resolving
// the TypeIds in the Context
pub mod zonking;
