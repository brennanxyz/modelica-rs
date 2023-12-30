#[allow(unused_imports)]
use antlr_rust::{
    char_stream::CharStream, 
    common_token_stream::CommonTokenStream,
    error_listener::ConsoleErrorListener, 
    InputStream,
    ListenerId,
    tree::{LeafNode, ParseTreeListener, ParseTree,},
    token_factory::ArenaCommonFactory,
};

use modelicalistener::*;
mod modelicalexer;
mod modelicaparser;
mod modelicalistener;

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

#[derive(Debug, Clone,)]
pub struct MyListener {
    pub class_name: String,
    pub class_contents: Vec<(String, isize, isize)>,
    pub gathering_instance: bool,
    pub current_instance_name: Option<String>,
    pub current_instance_start: Option<isize>,
    pub current_instance_end: Option<isize>,
}

impl MyListener {
    pub fn new(class_name: String) -> Self {
        MyListener {
            class_name,
            class_contents: Vec::new(),
            gathering_instance: false,
            current_instance_name: None,
            current_instance_start: None,
            current_instance_end: None,
        }
    }

    pub fn get_class_contents(&mut self, raw_data: &str,) -> ListenerId<&mut MyListener> {
        let input = InputStream::new(raw_data);
        let lexer = modelicalexer::modelicaLexer::new(input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = modelicaparser::modelicaParser::new(token_source);
        let _error_listener = ModelicaErrorListener::new();

        let listener_id = parser.add_parse_listener(Box::new(self));
        let result = parser.stored_definition();
        assert!(result.is_ok());
        listener_id
    }
}

impl<'input> ParseTreeListener<'input, modelicaparser::modelicaParserContextType> for &mut MyListener {
    fn enter_every_rule(&mut self, ctx: &dyn modelicaparser::modelicaParserContext<'input>) {
        let rule_name = modelicaparser::ruleNames
            .get(ctx.get_rule_index())
            .unwrap_or(&"ERROR");

        if !self.gathering_instance {
            if rule_name == &"class_definition" {
                if &ctx.start().text.to_string() == &self.class_name {
                    self.gathering_instance = true;
                    self.current_instance_start = Some(ctx.start().start);
                }
            }
        } else {
            if self.current_instance_name.is_none() {
                if rule_name == &"class_specifier" {
                    self.current_instance_name = Some(ctx.start().text.to_string());
                }
            }
        }
    }

    fn exit_every_rule(&mut self, ctx: &dyn modelicaparser::modelicaParserContext<'input>) {
        if self.gathering_instance {
            let rule_name = modelicaparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"ERROR");

            if rule_name == &"class_definition" {
                if let Some(ci_name) = &self.current_instance_name.clone() {
                    if &&ctx.stop().text.to_string() == &ci_name {
                        println!("EXITED A {}", self.class_name);
                        self.gathering_instance = false;
                        self.current_instance_end = Some(ctx.stop().stop);
                        let start = self.current_instance_start.unwrap_or(0);
                        let end = self.current_instance_end.unwrap_or(1);

                        let tuple = (ci_name.clone(), start, end);

                        self.class_contents.push(
                            tuple
                        );
                        self.current_instance_name = None;
                        self.current_instance_start = None;
                        self.current_instance_end = None;

                        println!("CLASS CONTENTS: {:?}", self.class_contents);
                    }
                } 
            }
        }
    }
}

impl<'input> modelicaListener<'input> for &mut MyListener {}

