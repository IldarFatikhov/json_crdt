use std::collections::HashSet;

pub enum Mutation {
    Assign,
    Insert,
    Delete
}

pub struct LamportTimestamp{}

pub struct Cursor{}

pub struct Operation {
    id:LamportTimestamp,
    deps:HashSet<LamportTimestamp>,
    cursor:Cursor,
    pub mutation:Mutation,
}


