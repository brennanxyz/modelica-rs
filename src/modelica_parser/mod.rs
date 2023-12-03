#[allow(unused_imports)]
use antlr_rust::{
    char_stream::CharStream, 
    common_token_stream::CommonTokenStream,
    error_listener::ConsoleErrorListener, 
    InputStream,
    tree::{LeafNode, ParseTreeListener, ParseTree,},
    token_factory::ArenaCommonFactory,
};

use modelicalistener::*;
use modelicalexer::*;

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

struct Listener {}

impl<'input> ParseTreeListener<'input, modelicaparser::modelicaParserContextType> for Listener {
    fn enter_every_rule(&mut self, ctx: &dyn modelicaparser::modelicaParserContext<'input>) {
        println!(
            "rule entered {}",
            modelicaparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> modelicaListener<'input> for Listener {}

pub fn read() {
    let input = InputStream::new(r#"model Test end Test;"#);
    let mut lexer = modelicalexer::modelicaLexer::new(input);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = modelicaparser::modelicaParser::new(token_source);
    let mut error_listener = ModelicaErrorListener::new();
    parser.add_parse_listener(Box::new(Listener {}));
    println!("\nstart parsing parser_test_modelica");
    let result = parser.stored_definition();
    assert!(result.is_ok());
    println!("finished parsing parser_test_modelica");
    println!("result: {}", result.unwrap().to_string_tree(&*parser));

    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
    // let mut listener = modelicalistener::modelicaListener::new();
    // antlr_rust::tree::ParseTreeWalker::DEFAULT.walk(&mut listener, &tree);
    // println!("{:?}", listener);
}