#[allow(unused_imports)]
use antlr_rust::{
    char_stream::CharStream, 
    common_token_stream::CommonTokenStream,
    error_listener::ConsoleErrorListener, 
    tree::LeafNode
};

mod modelicalexer;
mod modelicaparser;
mod modelicalistener;
// use modelicalexer::modelicaLexer;
// use modelicaparser::modelicaParser;

// pub struct Test {
//     pub name: String,
// }

#[allow(dead_code)]
pub struct ModelicaErrorListener {
    pub listener: ConsoleErrorListener,
}

impl ModelicaErrorListener {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ModelicaErrorListener {
            listener: ConsoleErrorListener {},
        }
    }
}