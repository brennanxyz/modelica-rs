// Generated from modelica.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::modelicalistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const T__63:isize=64; 
		pub const T__64:isize=65; 
		pub const T__65:isize=66; 
		pub const T__66:isize=67; 
		pub const T__67:isize=68; 
		pub const T__68:isize=69; 
		pub const T__69:isize=70; 
		pub const T__70:isize=71; 
		pub const T__71:isize=72; 
		pub const T__72:isize=73; 
		pub const T__73:isize=74; 
		pub const T__74:isize=75; 
		pub const T__75:isize=76; 
		pub const T__76:isize=77; 
		pub const T__77:isize=78; 
		pub const T__78:isize=79; 
		pub const T__79:isize=80; 
		pub const T__80:isize=81; 
		pub const T__81:isize=82; 
		pub const T__82:isize=83; 
		pub const T__83:isize=84; 
		pub const T__84:isize=85; 
		pub const T__85:isize=86; 
		pub const T__86:isize=87; 
		pub const T__87:isize=88; 
		pub const IDENT:isize=89; 
		pub const STRING:isize=90; 
		pub const UNSIGNED_NUMBER:isize=91; 
		pub const WS:isize=92; 
		pub const COMMENT:isize=93; 
		pub const LINE_COMMENT:isize=94;
	pub const RULE_stored_definition:usize = 0; 
	pub const RULE_class_definition:usize = 1; 
	pub const RULE_class_specifier:usize = 2; 
	pub const RULE_class_prefixes:usize = 3; 
	pub const RULE_long_class_specifier:usize = 4; 
	pub const RULE_short_class_specifier:usize = 5; 
	pub const RULE_der_class_specifier:usize = 6; 
	pub const RULE_base_prefix:usize = 7; 
	pub const RULE_enum_list:usize = 8; 
	pub const RULE_enumeration_literal:usize = 9; 
	pub const RULE_composition:usize = 10; 
	pub const RULE_language_specification:usize = 11; 
	pub const RULE_external_function_call:usize = 12; 
	pub const RULE_element_list:usize = 13; 
	pub const RULE_element:usize = 14; 
	pub const RULE_import_clause:usize = 15; 
	pub const RULE_import_list:usize = 16; 
	pub const RULE_extends_clause:usize = 17; 
	pub const RULE_constraining_clause:usize = 18; 
	pub const RULE_component_clause:usize = 19; 
	pub const RULE_type_prefix:usize = 20; 
	pub const RULE_type_specifier:usize = 21; 
	pub const RULE_component_list:usize = 22; 
	pub const RULE_component_declaration:usize = 23; 
	pub const RULE_condition_attribute:usize = 24; 
	pub const RULE_declaration:usize = 25; 
	pub const RULE_modification:usize = 26; 
	pub const RULE_class_modification:usize = 27; 
	pub const RULE_argument_list:usize = 28; 
	pub const RULE_argument:usize = 29; 
	pub const RULE_element_modification_or_replaceable:usize = 30; 
	pub const RULE_element_modification:usize = 31; 
	pub const RULE_element_redeclaration:usize = 32; 
	pub const RULE_element_replaceable:usize = 33; 
	pub const RULE_component_clause1:usize = 34; 
	pub const RULE_component_declaration1:usize = 35; 
	pub const RULE_short_class_definition:usize = 36; 
	pub const RULE_equation_section:usize = 37; 
	pub const RULE_algorithm_section:usize = 38; 
	pub const RULE_equation:usize = 39; 
	pub const RULE_statement:usize = 40; 
	pub const RULE_if_equation:usize = 41; 
	pub const RULE_if_statement:usize = 42; 
	pub const RULE_for_equation:usize = 43; 
	pub const RULE_for_statement:usize = 44; 
	pub const RULE_for_indices:usize = 45; 
	pub const RULE_for_index:usize = 46; 
	pub const RULE_while_statement:usize = 47; 
	pub const RULE_when_equation:usize = 48; 
	pub const RULE_when_statement:usize = 49; 
	pub const RULE_connect_clause:usize = 50; 
	pub const RULE_expression:usize = 51; 
	pub const RULE_simple_expression:usize = 52; 
	pub const RULE_logical_expression:usize = 53; 
	pub const RULE_logical_term:usize = 54; 
	pub const RULE_logical_factor:usize = 55; 
	pub const RULE_relation:usize = 56; 
	pub const RULE_rel_op:usize = 57; 
	pub const RULE_arithmetic_expression:usize = 58; 
	pub const RULE_add_op:usize = 59; 
	pub const RULE_term:usize = 60; 
	pub const RULE_mul_op:usize = 61; 
	pub const RULE_factor:usize = 62; 
	pub const RULE_primary:usize = 63; 
	pub const RULE_name:usize = 64; 
	pub const RULE_component_reference:usize = 65; 
	pub const RULE_function_call_args:usize = 66; 
	pub const RULE_function_arguments:usize = 67; 
	pub const RULE_named_arguments:usize = 68; 
	pub const RULE_named_argument:usize = 69; 
	pub const RULE_function_argument:usize = 70; 
	pub const RULE_output_expression_list:usize = 71; 
	pub const RULE_expression_list:usize = 72; 
	pub const RULE_array_subscripts:usize = 73; 
	pub const RULE_subscript_:usize = 74; 
	pub const RULE_comment:usize = 75; 
	pub const RULE_string_comment:usize = 76; 
	pub const RULE_annotation:usize = 77;
	pub const ruleNames: [&'static str; 78] =  [
		"stored_definition", "class_definition", "class_specifier", "class_prefixes", 
		"long_class_specifier", "short_class_specifier", "der_class_specifier", 
		"base_prefix", "enum_list", "enumeration_literal", "composition", "language_specification", 
		"external_function_call", "element_list", "element", "import_clause", 
		"import_list", "extends_clause", "constraining_clause", "component_clause", 
		"type_prefix", "type_specifier", "component_list", "component_declaration", 
		"condition_attribute", "declaration", "modification", "class_modification", 
		"argument_list", "argument", "element_modification_or_replaceable", "element_modification", 
		"element_redeclaration", "element_replaceable", "component_clause1", "component_declaration1", 
		"short_class_definition", "equation_section", "algorithm_section", "equation", 
		"statement", "if_equation", "if_statement", "for_equation", "for_statement", 
		"for_indices", "for_index", "while_statement", "when_equation", "when_statement", 
		"connect_clause", "expression", "simple_expression", "logical_expression", 
		"logical_term", "logical_factor", "relation", "rel_op", "arithmetic_expression", 
		"add_op", "term", "mul_op", "factor", "primary", "name", "component_reference", 
		"function_call_args", "function_arguments", "named_arguments", "named_argument", 
		"function_argument", "output_expression_list", "expression_list", "array_subscripts", 
		"subscript_", "comment", "string_comment", "annotation"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;89] = [
		None, Some("'within'"), Some("';'"), Some("'final'"), Some("'encapsulated'"), 
		Some("'partial'"), Some("'class'"), Some("'model'"), Some("'operator'"), 
		Some("'record'"), Some("'block'"), Some("'expandable'"), Some("'connector'"), 
		Some("'type'"), Some("'package'"), Some("'pure'"), Some("'impure'"), Some("'function'"), 
		Some("'end'"), Some("'extends'"), Some("'='"), Some("'enumeration'"), 
		Some("'('"), Some("':'"), Some("')'"), Some("'der'"), Some("','"), Some("'public'"), 
		Some("'protected'"), Some("'external'"), Some("'redeclare'"), Some("'inner'"), 
		Some("'outer'"), Some("'replaceable'"), Some("'import'"), Some("'.*'"), 
		Some("'.{'"), Some("'}'"), Some("'constrainedby'"), Some("'flow'"), Some("'stream'"), 
		Some("'discrete'"), Some("'parameter'"), Some("'constant'"), Some("'input'"), 
		Some("'output'"), Some("'if'"), Some("':='"), Some("'each'"), Some("'initial'"), 
		Some("'equation'"), Some("'algorithm'"), Some("'break'"), Some("'return'"), 
		Some("'then'"), Some("'elseif'"), Some("'else'"), Some("'for'"), Some("'loop'"), 
		Some("'in'"), Some("'while'"), Some("'when'"), Some("'elsewhen'"), Some("'connect'"), 
		Some("'or'"), Some("'and'"), Some("'not'"), Some("'<'"), Some("'<='"), 
		Some("'>'"), Some("'>='"), Some("'=='"), Some("'<>'"), Some("'+'"), Some("'-'"), 
		Some("'.+'"), Some("'.-'"), Some("'*'"), Some("'/'"), Some("'./'"), Some("'^'"), 
		Some("'.^'"), Some("'false'"), Some("'true'"), Some("'['"), Some("']'"), 
		Some("'{'"), Some("'.'"), Some("'annotation'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;95]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, Some("IDENT"), Some("STRING"), Some("UNSIGNED_NUMBER"), 
		Some("WS"), Some("COMMENT"), Some("LINE_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,modelicaParserExt<'input>, I, modelicaParserContextType , dyn modelicaListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type modelicaTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, modelicaParserContextType , dyn modelicaListener<'input> + 'a>;

/// Parser for modelica grammar
pub struct modelicaParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				modelicaParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> modelicaParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> modelicaParser<'input, I, DefaultErrorStrategy<'input,modelicaParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for modelicaParser
pub trait modelicaParserContext<'input>:
	for<'x> Listenable<dyn modelicaListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=modelicaParserContextType>
{}

antlr_rust::coerce_from!{ 'input : modelicaParserContext<'input> }

impl<'input> modelicaParserContext<'input> for TerminalNode<'input,modelicaParserContextType> {}
impl<'input> modelicaParserContext<'input> for ErrorNode<'input,modelicaParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn modelicaParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn modelicaListener<'input> + 'input }

pub struct modelicaParserContextType;
antlr_rust::tid!{modelicaParserContextType}

impl<'input> ParserNodeType<'input> for modelicaParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn modelicaParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct modelicaParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> modelicaParserExt<'input>{
}
antlr_rust::tid! { modelicaParserExt<'a> }

impl<'input> TokenAware<'input> for modelicaParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for modelicaParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for modelicaParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "modelica.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- stored_definition ----------------
pub type Stored_definitionContextAll<'input> = Stored_definitionContext<'input>;


pub type Stored_definitionContext<'input> = BaseParserRuleContext<'input,Stored_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Stored_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Stored_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Stored_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stored_definition(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_stored_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Stored_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stored_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stored_definition }
}
antlr_rust::tid!{Stored_definitionContextExt<'a>}

impl<'input> Stored_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Stored_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Stored_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Stored_definitionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Stored_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn class_definition_all(&self) ->  Vec<Rc<Class_definitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn class_definition(&self, i: usize) -> Option<Rc<Class_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn name_all(&self) ->  Vec<Rc<NameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn name(&self, i: usize) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Stored_definitionContextAttrs<'input> for Stored_definitionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn stored_definition(&mut self,)
	-> Result<Rc<Stored_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Stored_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_stored_definition);
        let mut _localctx: Rc<Stored_definitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(163);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 {
				{
				{
				recog.base.set_state(156);
				recog.base.match_token(T__0,&mut recog.err_handler)?;

				recog.base.set_state(158);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__86 || _la==IDENT {
					{
					/*InvokeRule name*/
					recog.base.set_state(157);
					recog.name()?;

					}
				}

				recog.base.set_state(160);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(165);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(174);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__14) | (1usize << T__15) | (1usize << T__16))) != 0) {
				{
				{
				recog.base.set_state(167);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__2 {
					{
					recog.base.set_state(166);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule class_definition*/
				recog.base.set_state(169);
				recog.class_definition()?;

				recog.base.set_state(170);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(176);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(177);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- class_definition ----------------
pub type Class_definitionContextAll<'input> = Class_definitionContext<'input>;


pub type Class_definitionContext<'input> = BaseParserRuleContext<'input,Class_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Class_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Class_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Class_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_definition(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_class_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Class_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_class_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_definition }
}
antlr_rust::tid!{Class_definitionContextExt<'a>}

impl<'input> Class_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_definitionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Class_definitionContextExt<'input>>{

fn class_prefixes(&self) -> Option<Rc<Class_prefixesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_specifier(&self) -> Option<Rc<Class_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Class_definitionContextAttrs<'input> for Class_definitionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn class_definition(&mut self,)
	-> Result<Rc<Class_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Class_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_class_definition);
        let mut _localctx: Rc<Class_definitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(180);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__3 {
				{
				recog.base.set_state(179);
				recog.base.match_token(T__3,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule class_prefixes*/
			recog.base.set_state(182);
			recog.class_prefixes()?;

			/*InvokeRule class_specifier*/
			recog.base.set_state(183);
			recog.class_specifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- class_specifier ----------------
pub type Class_specifierContextAll<'input> = Class_specifierContext<'input>;


pub type Class_specifierContext<'input> = BaseParserRuleContext<'input,Class_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Class_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Class_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Class_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_specifier(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_class_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Class_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_class_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_specifier }
}
antlr_rust::tid!{Class_specifierContextExt<'a>}

impl<'input> Class_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_specifierContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Class_specifierContextExt<'input>>{

fn long_class_specifier(&self) -> Option<Rc<Long_class_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn short_class_specifier(&self) -> Option<Rc<Short_class_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn der_class_specifier(&self) -> Option<Rc<Der_class_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Class_specifierContextAttrs<'input> for Class_specifierContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn class_specifier(&mut self,)
	-> Result<Rc<Class_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Class_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_class_specifier);
        let mut _localctx: Rc<Class_specifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(188);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule long_class_specifier*/
					recog.base.set_state(185);
					recog.long_class_specifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule short_class_specifier*/
					recog.base.set_state(186);
					recog.short_class_specifier()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule der_class_specifier*/
					recog.base.set_state(187);
					recog.der_class_specifier()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- class_prefixes ----------------
pub type Class_prefixesContextAll<'input> = Class_prefixesContext<'input>;


pub type Class_prefixesContext<'input> = BaseParserRuleContext<'input,Class_prefixesContextExt<'input>>;

#[derive(Clone)]
pub struct Class_prefixesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Class_prefixesContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Class_prefixesContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_prefixes(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_class_prefixes(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Class_prefixesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_class_prefixes }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_prefixes }
}
antlr_rust::tid!{Class_prefixesContextExt<'a>}

impl<'input> Class_prefixesContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_prefixesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_prefixesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_prefixesContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Class_prefixesContextExt<'input>>{


}

impl<'input> Class_prefixesContextAttrs<'input> for Class_prefixesContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn class_prefixes(&mut self,)
	-> Result<Rc<Class_prefixesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Class_prefixesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_class_prefixes);
        let mut _localctx: Rc<Class_prefixesContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(191);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__4 {
				{
				recog.base.set_state(190);
				recog.base.match_token(T__4,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(214);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(193);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(194);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(196);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__7 {
						{
						recog.base.set_state(195);
						recog.base.match_token(T__7,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(198);
					recog.base.match_token(T__8,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(199);
					recog.base.match_token(T__9,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(201);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__10 {
						{
						recog.base.set_state(200);
						recog.base.match_token(T__10,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(203);
					recog.base.match_token(T__11,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					recog.base.set_state(204);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					{
					recog.base.set_state(205);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					{
					recog.base.set_state(207);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__14 || _la==T__15 {
						{
						recog.base.set_state(206);
						_la = recog.base.input.la(1);
						if { !(_la==T__14 || _la==T__15) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(210);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__7 {
						{
						recog.base.set_state(209);
						recog.base.match_token(T__7,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(212);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					{
					recog.base.set_state(213);
					recog.base.match_token(T__7,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- long_class_specifier ----------------
pub type Long_class_specifierContextAll<'input> = Long_class_specifierContext<'input>;


pub type Long_class_specifierContext<'input> = BaseParserRuleContext<'input,Long_class_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Long_class_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Long_class_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Long_class_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_long_class_specifier(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_long_class_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Long_class_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_long_class_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_long_class_specifier }
}
antlr_rust::tid!{Long_class_specifierContextExt<'a>}

impl<'input> Long_class_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Long_class_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Long_class_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Long_class_specifierContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Long_class_specifierContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}
fn string_comment(&self) -> Option<Rc<String_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn composition(&self) -> Option<Rc<CompositionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Long_class_specifierContextAttrs<'input> for Long_class_specifierContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn long_class_specifier(&mut self,)
	-> Result<Rc<Long_class_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Long_class_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_long_class_specifier);
        let mut _localctx: Rc<Long_class_specifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(232);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(216);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					/*InvokeRule string_comment*/
					recog.base.set_state(217);
					recog.string_comment()?;

					/*InvokeRule composition*/
					recog.base.set_state(218);
					recog.composition()?;

					recog.base.set_state(219);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					recog.base.set_state(220);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(222);
					recog.base.match_token(T__18,&mut recog.err_handler)?;

					recog.base.set_state(223);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(225);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__21 {
						{
						/*InvokeRule class_modification*/
						recog.base.set_state(224);
						recog.class_modification()?;

						}
					}

					/*InvokeRule string_comment*/
					recog.base.set_state(227);
					recog.string_comment()?;

					/*InvokeRule composition*/
					recog.base.set_state(228);
					recog.composition()?;

					recog.base.set_state(229);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					recog.base.set_state(230);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- short_class_specifier ----------------
pub type Short_class_specifierContextAll<'input> = Short_class_specifierContext<'input>;


pub type Short_class_specifierContext<'input> = BaseParserRuleContext<'input,Short_class_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Short_class_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Short_class_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Short_class_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_short_class_specifier(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_short_class_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Short_class_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_short_class_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_short_class_specifier }
}
antlr_rust::tid!{Short_class_specifierContextExt<'a>}

impl<'input> Short_class_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Short_class_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Short_class_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Short_class_specifierContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Short_class_specifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn base_prefix(&self) -> Option<Rc<Base_prefixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array_subscripts(&self) -> Option<Rc<Array_subscriptsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enum_list(&self) -> Option<Rc<Enum_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Short_class_specifierContextAttrs<'input> for Short_class_specifierContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn short_class_specifier(&mut self,)
	-> Result<Rc<Short_class_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Short_class_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_short_class_specifier);
        let mut _localctx: Rc<Short_class_specifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(258);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(234);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(235);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					/*InvokeRule base_prefix*/
					recog.base.set_state(236);
					recog.base_prefix()?;

					/*InvokeRule name*/
					recog.base.set_state(237);
					recog.name()?;

					recog.base.set_state(239);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__83 {
						{
						/*InvokeRule array_subscripts*/
						recog.base.set_state(238);
						recog.array_subscripts()?;

						}
					}

					recog.base.set_state(242);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__21 {
						{
						/*InvokeRule class_modification*/
						recog.base.set_state(241);
						recog.class_modification()?;

						}
					}

					/*InvokeRule comment*/
					recog.base.set_state(244);
					recog.comment()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(246);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(247);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					recog.base.set_state(248);
					recog.base.match_token(T__20,&mut recog.err_handler)?;

					recog.base.set_state(249);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(254);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__23 | IDENT 
						=> {
							{
							recog.base.set_state(251);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==IDENT {
								{
								/*InvokeRule enum_list*/
								recog.base.set_state(250);
								recog.enum_list()?;

								}
							}

							}
						}

					 T__22 
						=> {
							{
							recog.base.set_state(253);
							recog.base.match_token(T__22,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(256);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					/*InvokeRule comment*/
					recog.base.set_state(257);
					recog.comment()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- der_class_specifier ----------------
pub type Der_class_specifierContextAll<'input> = Der_class_specifierContext<'input>;


pub type Der_class_specifierContext<'input> = BaseParserRuleContext<'input,Der_class_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Der_class_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Der_class_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Der_class_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_der_class_specifier(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_der_class_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Der_class_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_der_class_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_der_class_specifier }
}
antlr_rust::tid!{Der_class_specifierContextExt<'a>}

impl<'input> Der_class_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Der_class_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Der_class_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Der_class_specifierContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Der_class_specifierContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Der_class_specifierContextAttrs<'input> for Der_class_specifierContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn der_class_specifier(&mut self,)
	-> Result<Rc<Der_class_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Der_class_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_der_class_specifier);
        let mut _localctx: Rc<Der_class_specifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(260);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(261);
			recog.base.match_token(T__19,&mut recog.err_handler)?;

			recog.base.set_state(262);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			recog.base.set_state(263);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(264);
			recog.name()?;

			recog.base.set_state(265);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			recog.base.set_state(266);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(271);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(267);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				recog.base.set_state(268);
				recog.base.match_token(IDENT,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(273);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(274);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule comment*/
			recog.base.set_state(275);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- base_prefix ----------------
pub type Base_prefixContextAll<'input> = Base_prefixContext<'input>;


pub type Base_prefixContext<'input> = BaseParserRuleContext<'input,Base_prefixContextExt<'input>>;

#[derive(Clone)]
pub struct Base_prefixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Base_prefixContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Base_prefixContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_base_prefix(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_base_prefix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Base_prefixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_base_prefix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_base_prefix }
}
antlr_rust::tid!{Base_prefixContextExt<'a>}

impl<'input> Base_prefixContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Base_prefixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Base_prefixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Base_prefixContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Base_prefixContextExt<'input>>{

fn type_prefix(&self) -> Option<Rc<Type_prefixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Base_prefixContextAttrs<'input> for Base_prefixContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn base_prefix(&mut self,)
	-> Result<Rc<Base_prefixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Base_prefixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_base_prefix);
        let mut _localctx: Rc<Base_prefixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_prefix*/
			recog.base.set_state(277);
			recog.type_prefix()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enum_list ----------------
pub type Enum_listContextAll<'input> = Enum_listContext<'input>;


pub type Enum_listContext<'input> = BaseParserRuleContext<'input,Enum_listContextExt<'input>>;

#[derive(Clone)]
pub struct Enum_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Enum_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Enum_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enum_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_enum_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Enum_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enum_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enum_list }
}
antlr_rust::tid!{Enum_listContextExt<'a>}

impl<'input> Enum_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Enum_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Enum_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Enum_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Enum_listContextExt<'input>>{

fn enumeration_literal_all(&self) ->  Vec<Rc<Enumeration_literalContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumeration_literal(&self, i: usize) -> Option<Rc<Enumeration_literalContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Enum_listContextAttrs<'input> for Enum_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enum_list(&mut self,)
	-> Result<Rc<Enum_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Enum_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_enum_list);
        let mut _localctx: Rc<Enum_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule enumeration_literal*/
			recog.base.set_state(279);
			recog.enumeration_literal()?;

			recog.base.set_state(284);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(280);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule enumeration_literal*/
				recog.base.set_state(281);
				recog.enumeration_literal()?;

				}
				}
				recog.base.set_state(286);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumeration_literal ----------------
pub type Enumeration_literalContextAll<'input> = Enumeration_literalContext<'input>;


pub type Enumeration_literalContext<'input> = BaseParserRuleContext<'input,Enumeration_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Enumeration_literalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Enumeration_literalContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Enumeration_literalContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumeration_literal(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_enumeration_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Enumeration_literalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumeration_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumeration_literal }
}
antlr_rust::tid!{Enumeration_literalContextExt<'a>}

impl<'input> Enumeration_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Enumeration_literalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Enumeration_literalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Enumeration_literalContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Enumeration_literalContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Enumeration_literalContextAttrs<'input> for Enumeration_literalContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumeration_literal(&mut self,)
	-> Result<Rc<Enumeration_literalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Enumeration_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_enumeration_literal);
        let mut _localctx: Rc<Enumeration_literalContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(287);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			/*InvokeRule comment*/
			recog.base.set_state(288);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- composition ----------------
pub type CompositionContextAll<'input> = CompositionContext<'input>;


pub type CompositionContext<'input> = BaseParserRuleContext<'input,CompositionContextExt<'input>>;

#[derive(Clone)]
pub struct CompositionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for CompositionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for CompositionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_composition(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_composition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CompositionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_composition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_composition }
}
antlr_rust::tid!{CompositionContextExt<'a>}

impl<'input> CompositionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompositionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompositionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompositionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<CompositionContextExt<'input>>{

fn element_list_all(&self) ->  Vec<Rc<Element_listContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn element_list(&self, i: usize) -> Option<Rc<Element_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn equation_section_all(&self) ->  Vec<Rc<Equation_sectionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation_section(&self, i: usize) -> Option<Rc<Equation_sectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn algorithm_section_all(&self) ->  Vec<Rc<Algorithm_sectionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn algorithm_section(&self, i: usize) -> Option<Rc<Algorithm_sectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn language_specification(&self) -> Option<Rc<Language_specificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn external_function_call(&self) -> Option<Rc<External_function_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompositionContextAttrs<'input> for CompositionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn composition(&mut self,)
	-> Result<Rc<CompositionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompositionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_composition);
        let mut _localctx: Rc<CompositionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule element_list*/
			recog.base.set_state(290);
			recog.element_list()?;

			recog.base.set_state(299);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 27)) & !0x3f) == 0 && ((1usize << (_la - 27)) & ((1usize << (T__26 - 27)) | (1usize << (T__27 - 27)) | (1usize << (T__48 - 27)) | (1usize << (T__49 - 27)) | (1usize << (T__50 - 27)))) != 0) {
				{
				recog.base.set_state(297);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
					1 =>{
						{
						recog.base.set_state(291);
						recog.base.match_token(T__26,&mut recog.err_handler)?;

						/*InvokeRule element_list*/
						recog.base.set_state(292);
						recog.element_list()?;

						}
					}
				,
					2 =>{
						{
						recog.base.set_state(293);
						recog.base.match_token(T__27,&mut recog.err_handler)?;

						/*InvokeRule element_list*/
						recog.base.set_state(294);
						recog.element_list()?;

						}
					}
				,
					3 =>{
						{
						/*InvokeRule equation_section*/
						recog.base.set_state(295);
						recog.equation_section()?;

						}
					}
				,
					4 =>{
						{
						/*InvokeRule algorithm_section*/
						recog.base.set_state(296);
						recog.algorithm_section()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(301);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(313);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__28 {
				{
				recog.base.set_state(302);
				recog.base.match_token(T__28,&mut recog.err_handler)?;

				recog.base.set_state(304);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==STRING {
					{
					/*InvokeRule language_specification*/
					recog.base.set_state(303);
					recog.language_specification()?;

					}
				}

				recog.base.set_state(307);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__86 || _la==IDENT {
					{
					/*InvokeRule external_function_call*/
					recog.base.set_state(306);
					recog.external_function_call()?;

					}
				}

				recog.base.set_state(310);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__87 {
					{
					/*InvokeRule annotation*/
					recog.base.set_state(309);
					recog.annotation()?;

					}
				}

				recog.base.set_state(312);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(318);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__87 {
				{
				/*InvokeRule annotation*/
				recog.base.set_state(315);
				recog.annotation()?;

				recog.base.set_state(316);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- language_specification ----------------
pub type Language_specificationContextAll<'input> = Language_specificationContext<'input>;


pub type Language_specificationContext<'input> = BaseParserRuleContext<'input,Language_specificationContextExt<'input>>;

#[derive(Clone)]
pub struct Language_specificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Language_specificationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Language_specificationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_language_specification(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_language_specification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Language_specificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_language_specification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_language_specification }
}
antlr_rust::tid!{Language_specificationContextExt<'a>}

impl<'input> Language_specificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Language_specificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Language_specificationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Language_specificationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Language_specificationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> Language_specificationContextAttrs<'input> for Language_specificationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn language_specification(&mut self,)
	-> Result<Rc<Language_specificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Language_specificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_language_specification);
        let mut _localctx: Rc<Language_specificationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(320);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- external_function_call ----------------
pub type External_function_callContextAll<'input> = External_function_callContext<'input>;


pub type External_function_callContext<'input> = BaseParserRuleContext<'input,External_function_callContextExt<'input>>;

#[derive(Clone)]
pub struct External_function_callContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for External_function_callContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for External_function_callContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_external_function_call(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_external_function_call(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for External_function_callContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_external_function_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_external_function_call }
}
antlr_rust::tid!{External_function_callContextExt<'a>}

impl<'input> External_function_callContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<External_function_callContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,External_function_callContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait External_function_callContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<External_function_callContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn component_reference(&self) -> Option<Rc<Component_referenceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_list(&self) -> Option<Rc<Expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> External_function_callContextAttrs<'input> for External_function_callContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn external_function_call(&mut self,)
	-> Result<Rc<External_function_callContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = External_function_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_external_function_call);
        let mut _localctx: Rc<External_function_callContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(325);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule component_reference*/
					recog.base.set_state(322);
					recog.component_reference()?;

					recog.base.set_state(323);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(327);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(328);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 18)) & !0x3f) == 0 && ((1usize << (_la - 18)) & ((1usize << (T__17 - 18)) | (1usize << (T__21 - 18)) | (1usize << (T__24 - 18)) | (1usize << (T__45 - 18)) | (1usize << (T__48 - 18)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__72 - 66)) | (1usize << (T__73 - 66)) | (1usize << (T__74 - 66)) | (1usize << (T__75 - 66)) | (1usize << (T__81 - 66)) | (1usize << (T__82 - 66)) | (1usize << (T__83 - 66)) | (1usize << (T__85 - 66)) | (1usize << (T__86 - 66)) | (1usize << (IDENT - 66)) | (1usize << (STRING - 66)) | (1usize << (UNSIGNED_NUMBER - 66)))) != 0) {
				{
				/*InvokeRule expression_list*/
				recog.base.set_state(329);
				recog.expression_list()?;

				}
			}

			recog.base.set_state(332);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element_list ----------------
pub type Element_listContextAll<'input> = Element_listContext<'input>;


pub type Element_listContext<'input> = BaseParserRuleContext<'input,Element_listContextExt<'input>>;

#[derive(Clone)]
pub struct Element_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Element_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Element_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Element_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_list }
}
antlr_rust::tid!{Element_listContextExt<'a>}

impl<'input> Element_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Element_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Element_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Element_listContextExt<'input>>{

fn element_all(&self) ->  Vec<Rc<ElementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn element(&self, i: usize) -> Option<Rc<ElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Element_listContextAttrs<'input> for Element_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn element_list(&mut self,)
	-> Result<Rc<Element_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_element_list);
        let mut _localctx: Rc<Element_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(339);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__2) | (1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__14) | (1usize << T__15) | (1usize << T__16) | (1usize << T__18) | (1usize << T__29) | (1usize << T__30))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (T__31 - 32)) | (1usize << (T__32 - 32)) | (1usize << (T__33 - 32)) | (1usize << (T__38 - 32)) | (1usize << (T__39 - 32)) | (1usize << (T__40 - 32)) | (1usize << (T__41 - 32)) | (1usize << (T__42 - 32)) | (1usize << (T__43 - 32)) | (1usize << (T__44 - 32)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule element*/
				recog.base.set_state(334);
				recog.element()?;

				recog.base.set_state(335);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(341);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element ----------------
pub type ElementContextAll<'input> = ElementContext<'input>;


pub type ElementContext<'input> = BaseParserRuleContext<'input,ElementContextExt<'input>>;

#[derive(Clone)]
pub struct ElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for ElementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for ElementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element }
}
antlr_rust::tid!{ElementContextExt<'a>}

impl<'input> ElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<ElementContextExt<'input>>{

fn import_clause(&self) -> Option<Rc<Import_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn extends_clause(&self) -> Option<Rc<Extends_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_definition(&self) -> Option<Rc<Class_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_clause(&self) -> Option<Rc<Component_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constraining_clause(&self) -> Option<Rc<Constraining_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ElementContextAttrs<'input> for ElementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn element(&mut self,)
	-> Result<Rc<ElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_element);
        let mut _localctx: Rc<ElementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(372);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__33 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule import_clause*/
					recog.base.set_state(342);
					recog.import_clause()?;

					}
				}

			 T__18 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule extends_clause*/
					recog.base.set_state(343);
					recog.extends_clause()?;

					}
				}

			 T__2 | T__3 | T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 |
			 T__12 | T__13 | T__14 | T__15 | T__16 | T__29 | T__30 | T__31 | T__32 |
			 T__38 | T__39 | T__40 | T__41 | T__42 | T__43 | T__44 | T__86 | IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(345);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__29 {
						{
						recog.base.set_state(344);
						recog.base.match_token(T__29,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(348);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__2 {
						{
						recog.base.set_state(347);
						recog.base.match_token(T__2,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(351);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__30 {
						{
						recog.base.set_state(350);
						recog.base.match_token(T__30,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(354);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__31 {
						{
						recog.base.set_state(353);
						recog.base.match_token(T__31,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(370);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__3 | T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 |
					 T__13 | T__14 | T__15 | T__16 | T__38 | T__39 | T__40 | T__41 | T__42 |
					 T__43 | T__44 | T__86 | IDENT 
						=> {
							{
							recog.base.set_state(358);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 T__3 | T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 |
							 T__12 | T__13 | T__14 | T__15 | T__16 
								=> {
									{
									/*InvokeRule class_definition*/
									recog.base.set_state(356);
									recog.class_definition()?;

									}
								}

							 T__38 | T__39 | T__40 | T__41 | T__42 | T__43 | T__44 | T__86 | IDENT 
								=> {
									{
									/*InvokeRule component_clause*/
									recog.base.set_state(357);
									recog.component_clause()?;

									}
								}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							}
						}

					 T__32 
						=> {
							{
							recog.base.set_state(360);
							recog.base.match_token(T__32,&mut recog.err_handler)?;

							recog.base.set_state(363);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 T__3 | T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 |
							 T__12 | T__13 | T__14 | T__15 | T__16 
								=> {
									{
									/*InvokeRule class_definition*/
									recog.base.set_state(361);
									recog.class_definition()?;

									}
								}

							 T__38 | T__39 | T__40 | T__41 | T__42 | T__43 | T__44 | T__86 | IDENT 
								=> {
									{
									/*InvokeRule component_clause*/
									recog.base.set_state(362);
									recog.component_clause()?;

									}
								}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							recog.base.set_state(368);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==T__37 {
								{
								/*InvokeRule constraining_clause*/
								recog.base.set_state(365);
								recog.constraining_clause()?;

								/*InvokeRule comment*/
								recog.base.set_state(366);
								recog.comment()?;

								}
							}

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- import_clause ----------------
pub type Import_clauseContextAll<'input> = Import_clauseContext<'input>;


pub type Import_clauseContext<'input> = BaseParserRuleContext<'input,Import_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Import_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Import_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Import_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_import_clause(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_import_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Import_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_import_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_import_clause }
}
antlr_rust::tid!{Import_clauseContextExt<'a>}

impl<'input> Import_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Import_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Import_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Import_clauseContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Import_clauseContextExt<'input>>{

fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn import_list(&self) -> Option<Rc<Import_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Import_clauseContextAttrs<'input> for Import_clauseContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn import_clause(&mut self,)
	-> Result<Rc<Import_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Import_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_import_clause);
        let mut _localctx: Rc<Import_clauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(374);
			recog.base.match_token(T__33,&mut recog.err_handler)?;

			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(375);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(376);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					/*InvokeRule name*/
					recog.base.set_state(377);
					recog.name()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule name*/
					recog.base.set_state(378);
					recog.name()?;

					recog.base.set_state(379);
					recog.base.match_token(T__34,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule name*/
					recog.base.set_state(381);
					recog.name()?;

					recog.base.set_state(382);
					recog.base.match_token(T__35,&mut recog.err_handler)?;

					/*InvokeRule import_list*/
					recog.base.set_state(383);
					recog.import_list()?;

					recog.base.set_state(384);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule name*/
					recog.base.set_state(386);
					recog.name()?;

					}
				}

				_ => {}
			}
			/*InvokeRule comment*/
			recog.base.set_state(389);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- import_list ----------------
pub type Import_listContextAll<'input> = Import_listContext<'input>;


pub type Import_listContext<'input> = BaseParserRuleContext<'input,Import_listContextExt<'input>>;

#[derive(Clone)]
pub struct Import_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Import_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Import_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_import_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_import_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Import_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_import_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_import_list }
}
antlr_rust::tid!{Import_listContextExt<'a>}

impl<'input> Import_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Import_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Import_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Import_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Import_listContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}

}

impl<'input> Import_listContextAttrs<'input> for Import_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn import_list(&mut self,)
	-> Result<Rc<Import_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Import_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_import_list);
        let mut _localctx: Rc<Import_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(391);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(396);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(392);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				recog.base.set_state(393);
				recog.base.match_token(IDENT,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(398);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- extends_clause ----------------
pub type Extends_clauseContextAll<'input> = Extends_clauseContext<'input>;


pub type Extends_clauseContext<'input> = BaseParserRuleContext<'input,Extends_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Extends_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Extends_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Extends_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_extends_clause(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_extends_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Extends_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extends_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extends_clause }
}
antlr_rust::tid!{Extends_clauseContextExt<'a>}

impl<'input> Extends_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Extends_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Extends_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Extends_clauseContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Extends_clauseContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Extends_clauseContextAttrs<'input> for Extends_clauseContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn extends_clause(&mut self,)
	-> Result<Rc<Extends_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Extends_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_extends_clause);
        let mut _localctx: Rc<Extends_clauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(399);
			recog.base.match_token(T__18,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(400);
			recog.name()?;

			recog.base.set_state(402);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__21 {
				{
				/*InvokeRule class_modification*/
				recog.base.set_state(401);
				recog.class_modification()?;

				}
			}

			recog.base.set_state(405);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__87 {
				{
				/*InvokeRule annotation*/
				recog.base.set_state(404);
				recog.annotation()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constraining_clause ----------------
pub type Constraining_clauseContextAll<'input> = Constraining_clauseContext<'input>;


pub type Constraining_clauseContext<'input> = BaseParserRuleContext<'input,Constraining_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Constraining_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Constraining_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Constraining_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constraining_clause(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_constraining_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Constraining_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constraining_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constraining_clause }
}
antlr_rust::tid!{Constraining_clauseContextExt<'a>}

impl<'input> Constraining_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Constraining_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Constraining_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Constraining_clauseContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Constraining_clauseContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Constraining_clauseContextAttrs<'input> for Constraining_clauseContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constraining_clause(&mut self,)
	-> Result<Rc<Constraining_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Constraining_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_constraining_clause);
        let mut _localctx: Rc<Constraining_clauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(407);
			recog.base.match_token(T__37,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(408);
			recog.name()?;

			recog.base.set_state(410);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__21 {
				{
				/*InvokeRule class_modification*/
				recog.base.set_state(409);
				recog.class_modification()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_clause ----------------
pub type Component_clauseContextAll<'input> = Component_clauseContext<'input>;


pub type Component_clauseContext<'input> = BaseParserRuleContext<'input,Component_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Component_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_clause(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_clause }
}
antlr_rust::tid!{Component_clauseContextExt<'a>}

impl<'input> Component_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_clauseContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_clauseContextExt<'input>>{

fn type_prefix(&self) -> Option<Rc<Type_prefixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_specifier(&self) -> Option<Rc<Type_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_list(&self) -> Option<Rc<Component_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array_subscripts(&self) -> Option<Rc<Array_subscriptsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Component_clauseContextAttrs<'input> for Component_clauseContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_clause(&mut self,)
	-> Result<Rc<Component_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_component_clause);
        let mut _localctx: Rc<Component_clauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_prefix*/
			recog.base.set_state(412);
			recog.type_prefix()?;

			/*InvokeRule type_specifier*/
			recog.base.set_state(413);
			recog.type_specifier()?;

			recog.base.set_state(415);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__83 {
				{
				/*InvokeRule array_subscripts*/
				recog.base.set_state(414);
				recog.array_subscripts()?;

				}
			}

			/*InvokeRule component_list*/
			recog.base.set_state(417);
			recog.component_list()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_prefix ----------------
pub type Type_prefixContextAll<'input> = Type_prefixContext<'input>;


pub type Type_prefixContext<'input> = BaseParserRuleContext<'input,Type_prefixContextExt<'input>>;

#[derive(Clone)]
pub struct Type_prefixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Type_prefixContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Type_prefixContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_prefix(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_type_prefix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type_prefixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_prefix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_prefix }
}
antlr_rust::tid!{Type_prefixContextExt<'a>}

impl<'input> Type_prefixContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_prefixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_prefixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_prefixContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Type_prefixContextExt<'input>>{


}

impl<'input> Type_prefixContextAttrs<'input> for Type_prefixContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn type_prefix(&mut self,)
	-> Result<Rc<Type_prefixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_prefixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_type_prefix);
        let mut _localctx: Rc<Type_prefixContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(420);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__38 || _la==T__39 {
				{
				recog.base.set_state(419);
				_la = recog.base.input.la(1);
				if { !(_la==T__38 || _la==T__39) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(423);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & ((1usize << (T__40 - 41)) | (1usize << (T__41 - 41)) | (1usize << (T__42 - 41)))) != 0) {
				{
				recog.base.set_state(422);
				_la = recog.base.input.la(1);
				if { !(((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & ((1usize << (T__40 - 41)) | (1usize << (T__41 - 41)) | (1usize << (T__42 - 41)))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(426);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__43 || _la==T__44 {
				{
				recog.base.set_state(425);
				_la = recog.base.input.la(1);
				if { !(_la==T__43 || _la==T__44) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_specifier ----------------
pub type Type_specifierContextAll<'input> = Type_specifierContext<'input>;


pub type Type_specifierContext<'input> = BaseParserRuleContext<'input,Type_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Type_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Type_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Type_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_specifier(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_type_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_specifier }
}
antlr_rust::tid!{Type_specifierContextExt<'a>}

impl<'input> Type_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_specifierContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Type_specifierContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type_specifierContextAttrs<'input> for Type_specifierContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_specifier(&mut self,)
	-> Result<Rc<Type_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_type_specifier);
        let mut _localctx: Rc<Type_specifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule name*/
			recog.base.set_state(428);
			recog.name()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_list ----------------
pub type Component_listContextAll<'input> = Component_listContext<'input>;


pub type Component_listContext<'input> = BaseParserRuleContext<'input,Component_listContextExt<'input>>;

#[derive(Clone)]
pub struct Component_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_list }
}
antlr_rust::tid!{Component_listContextExt<'a>}

impl<'input> Component_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_listContextExt<'input>>{

fn component_declaration_all(&self) ->  Vec<Rc<Component_declarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn component_declaration(&self, i: usize) -> Option<Rc<Component_declarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Component_listContextAttrs<'input> for Component_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_list(&mut self,)
	-> Result<Rc<Component_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_component_list);
        let mut _localctx: Rc<Component_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule component_declaration*/
			recog.base.set_state(430);
			recog.component_declaration()?;

			recog.base.set_state(435);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(431);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule component_declaration*/
				recog.base.set_state(432);
				recog.component_declaration()?;

				}
				}
				recog.base.set_state(437);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_declaration ----------------
pub type Component_declarationContextAll<'input> = Component_declarationContext<'input>;


pub type Component_declarationContext<'input> = BaseParserRuleContext<'input,Component_declarationContextExt<'input>>;

#[derive(Clone)]
pub struct Component_declarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_declarationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_declarationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_declaration(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_declaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_declarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_declaration }
}
antlr_rust::tid!{Component_declarationContextExt<'a>}

impl<'input> Component_declarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_declarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_declarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_declarationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_declarationContextExt<'input>>{

fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn condition_attribute(&self) -> Option<Rc<Condition_attributeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Component_declarationContextAttrs<'input> for Component_declarationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_declaration(&mut self,)
	-> Result<Rc<Component_declarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_declarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_component_declaration);
        let mut _localctx: Rc<Component_declarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule declaration*/
			recog.base.set_state(438);
			recog.declaration()?;

			recog.base.set_state(440);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__45 {
				{
				/*InvokeRule condition_attribute*/
				recog.base.set_state(439);
				recog.condition_attribute()?;

				}
			}

			/*InvokeRule comment*/
			recog.base.set_state(442);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- condition_attribute ----------------
pub type Condition_attributeContextAll<'input> = Condition_attributeContext<'input>;


pub type Condition_attributeContext<'input> = BaseParserRuleContext<'input,Condition_attributeContextExt<'input>>;

#[derive(Clone)]
pub struct Condition_attributeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Condition_attributeContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Condition_attributeContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_condition_attribute(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_condition_attribute(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Condition_attributeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_condition_attribute }
	//fn type_rule_index() -> usize where Self: Sized { RULE_condition_attribute }
}
antlr_rust::tid!{Condition_attributeContextExt<'a>}

impl<'input> Condition_attributeContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Condition_attributeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Condition_attributeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Condition_attributeContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Condition_attributeContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Condition_attributeContextAttrs<'input> for Condition_attributeContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn condition_attribute(&mut self,)
	-> Result<Rc<Condition_attributeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Condition_attributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_condition_attribute);
        let mut _localctx: Rc<Condition_attributeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(444);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(445);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;


pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for DeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_declaration(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_declaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr_rust::tid!{DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn array_subscripts(&self) -> Option<Rc<Array_subscriptsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modification(&self) -> Option<Rc<ModificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn declaration(&mut self,)
	-> Result<Rc<DeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(447);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(449);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__83 {
				{
				/*InvokeRule array_subscripts*/
				recog.base.set_state(448);
				recog.array_subscripts()?;

				}
			}

			recog.base.set_state(452);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 20)) & !0x3f) == 0 && ((1usize << (_la - 20)) & ((1usize << (T__19 - 20)) | (1usize << (T__21 - 20)) | (1usize << (T__46 - 20)))) != 0) {
				{
				/*InvokeRule modification*/
				recog.base.set_state(451);
				recog.modification()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modification ----------------
pub type ModificationContextAll<'input> = ModificationContext<'input>;


pub type ModificationContext<'input> = BaseParserRuleContext<'input,ModificationContextExt<'input>>;

#[derive(Clone)]
pub struct ModificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for ModificationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for ModificationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modification(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_modification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modification }
}
antlr_rust::tid!{ModificationContextExt<'a>}

impl<'input> ModificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModificationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModificationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<ModificationContextExt<'input>>{

fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModificationContextAttrs<'input> for ModificationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modification(&mut self,)
	-> Result<Rc<ModificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_modification);
        let mut _localctx: Rc<ModificationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(463);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__21 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule class_modification*/
					recog.base.set_state(454);
					recog.class_modification()?;

					recog.base.set_state(457);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__19 {
						{
						recog.base.set_state(455);
						recog.base.match_token(T__19,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(456);
						recog.expression()?;

						}
					}

					}
				}

			 T__19 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(459);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(460);
					recog.expression()?;

					}
				}

			 T__46 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(461);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(462);
					recog.expression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- class_modification ----------------
pub type Class_modificationContextAll<'input> = Class_modificationContext<'input>;


pub type Class_modificationContext<'input> = BaseParserRuleContext<'input,Class_modificationContextExt<'input>>;

#[derive(Clone)]
pub struct Class_modificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Class_modificationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Class_modificationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_class_modification(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_class_modification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Class_modificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_class_modification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_class_modification }
}
antlr_rust::tid!{Class_modificationContextExt<'a>}

impl<'input> Class_modificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Class_modificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Class_modificationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Class_modificationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Class_modificationContextExt<'input>>{

fn argument_list(&self) -> Option<Rc<Argument_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Class_modificationContextAttrs<'input> for Class_modificationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn class_modification(&mut self,)
	-> Result<Rc<Class_modificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Class_modificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_class_modification);
        let mut _localctx: Rc<Class_modificationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(465);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			recog.base.set_state(467);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__2 || _la==T__29 || _la==T__32 || _la==T__47 || _la==T__86 || _la==IDENT {
				{
				/*InvokeRule argument_list*/
				recog.base.set_state(466);
				recog.argument_list()?;

				}
			}

			recog.base.set_state(469);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- argument_list ----------------
pub type Argument_listContextAll<'input> = Argument_listContext<'input>;


pub type Argument_listContext<'input> = BaseParserRuleContext<'input,Argument_listContextExt<'input>>;

#[derive(Clone)]
pub struct Argument_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Argument_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Argument_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_argument_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_argument_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Argument_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argument_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argument_list }
}
antlr_rust::tid!{Argument_listContextExt<'a>}

impl<'input> Argument_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Argument_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Argument_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Argument_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Argument_listContextExt<'input>>{

fn argument_all(&self) ->  Vec<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn argument(&self, i: usize) -> Option<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Argument_listContextAttrs<'input> for Argument_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn argument_list(&mut self,)
	-> Result<Rc<Argument_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Argument_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_argument_list);
        let mut _localctx: Rc<Argument_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule argument*/
			recog.base.set_state(471);
			recog.argument()?;

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(472);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule argument*/
				recog.base.set_state(473);
				recog.argument()?;

				}
				}
				recog.base.set_state(478);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- argument ----------------
pub type ArgumentContextAll<'input> = ArgumentContext<'input>;


pub type ArgumentContext<'input> = BaseParserRuleContext<'input,ArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for ArgumentContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for ArgumentContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_argument(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_argument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ArgumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argument }
}
antlr_rust::tid!{ArgumentContextExt<'a>}

impl<'input> ArgumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<ArgumentContextExt<'input>>{

fn element_modification_or_replaceable(&self) -> Option<Rc<Element_modification_or_replaceableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn element_redeclaration(&self) -> Option<Rc<Element_redeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArgumentContextAttrs<'input> for ArgumentContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn argument(&mut self,)
	-> Result<Rc<ArgumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_argument);
        let mut _localctx: Rc<ArgumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(481);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__2 | T__32 | T__47 | T__86 | IDENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule element_modification_or_replaceable*/
					recog.base.set_state(479);
					recog.element_modification_or_replaceable()?;

					}
				}

			 T__29 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule element_redeclaration*/
					recog.base.set_state(480);
					recog.element_redeclaration()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element_modification_or_replaceable ----------------
pub type Element_modification_or_replaceableContextAll<'input> = Element_modification_or_replaceableContext<'input>;


pub type Element_modification_or_replaceableContext<'input> = BaseParserRuleContext<'input,Element_modification_or_replaceableContextExt<'input>>;

#[derive(Clone)]
pub struct Element_modification_or_replaceableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Element_modification_or_replaceableContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Element_modification_or_replaceableContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element_modification_or_replaceable(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element_modification_or_replaceable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Element_modification_or_replaceableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_modification_or_replaceable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_modification_or_replaceable }
}
antlr_rust::tid!{Element_modification_or_replaceableContextExt<'a>}

impl<'input> Element_modification_or_replaceableContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Element_modification_or_replaceableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_modification_or_replaceableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Element_modification_or_replaceableContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Element_modification_or_replaceableContextExt<'input>>{

fn element_modification(&self) -> Option<Rc<Element_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn element_replaceable(&self) -> Option<Rc<Element_replaceableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Element_modification_or_replaceableContextAttrs<'input> for Element_modification_or_replaceableContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn element_modification_or_replaceable(&mut self,)
	-> Result<Rc<Element_modification_or_replaceableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_modification_or_replaceableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_element_modification_or_replaceable);
        let mut _localctx: Rc<Element_modification_or_replaceableContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(484);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__47 {
				{
				recog.base.set_state(483);
				recog.base.match_token(T__47,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(487);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__2 {
				{
				recog.base.set_state(486);
				recog.base.match_token(T__2,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(491);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__86 | IDENT 
				=> {
					{
					/*InvokeRule element_modification*/
					recog.base.set_state(489);
					recog.element_modification()?;

					}
				}

			 T__32 
				=> {
					{
					/*InvokeRule element_replaceable*/
					recog.base.set_state(490);
					recog.element_replaceable()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element_modification ----------------
pub type Element_modificationContextAll<'input> = Element_modificationContext<'input>;


pub type Element_modificationContext<'input> = BaseParserRuleContext<'input,Element_modificationContextExt<'input>>;

#[derive(Clone)]
pub struct Element_modificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Element_modificationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Element_modificationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element_modification(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element_modification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Element_modificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_modification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_modification }
}
antlr_rust::tid!{Element_modificationContextExt<'a>}

impl<'input> Element_modificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Element_modificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_modificationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Element_modificationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Element_modificationContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn string_comment(&self) -> Option<Rc<String_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modification(&self) -> Option<Rc<ModificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Element_modificationContextAttrs<'input> for Element_modificationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn element_modification(&mut self,)
	-> Result<Rc<Element_modificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_modificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_element_modification);
        let mut _localctx: Rc<Element_modificationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule name*/
			recog.base.set_state(493);
			recog.name()?;

			recog.base.set_state(495);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 20)) & !0x3f) == 0 && ((1usize << (_la - 20)) & ((1usize << (T__19 - 20)) | (1usize << (T__21 - 20)) | (1usize << (T__46 - 20)))) != 0) {
				{
				/*InvokeRule modification*/
				recog.base.set_state(494);
				recog.modification()?;

				}
			}

			/*InvokeRule string_comment*/
			recog.base.set_state(497);
			recog.string_comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element_redeclaration ----------------
pub type Element_redeclarationContextAll<'input> = Element_redeclarationContext<'input>;


pub type Element_redeclarationContext<'input> = BaseParserRuleContext<'input,Element_redeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct Element_redeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Element_redeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Element_redeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element_redeclaration(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element_redeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Element_redeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_redeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_redeclaration }
}
antlr_rust::tid!{Element_redeclarationContextExt<'a>}

impl<'input> Element_redeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Element_redeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_redeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Element_redeclarationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Element_redeclarationContextExt<'input>>{

fn element_replaceable(&self) -> Option<Rc<Element_replaceableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn short_class_definition(&self) -> Option<Rc<Short_class_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_clause1(&self) -> Option<Rc<Component_clause1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Element_redeclarationContextAttrs<'input> for Element_redeclarationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn element_redeclaration(&mut self,)
	-> Result<Rc<Element_redeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_redeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_element_redeclaration);
        let mut _localctx: Rc<Element_redeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(499);
			recog.base.match_token(T__29,&mut recog.err_handler)?;

			recog.base.set_state(501);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__47 {
				{
				recog.base.set_state(500);
				recog.base.match_token(T__47,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(504);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__2 {
				{
				recog.base.set_state(503);
				recog.base.match_token(T__2,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(511);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 | T__13 |
			 T__14 | T__15 | T__16 | T__38 | T__39 | T__40 | T__41 | T__42 | T__43 |
			 T__44 | T__86 | IDENT 
				=> {
					{
					recog.base.set_state(508);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 | T__13 |
					 T__14 | T__15 | T__16 
						=> {
							{
							/*InvokeRule short_class_definition*/
							recog.base.set_state(506);
							recog.short_class_definition()?;

							}
						}

					 T__38 | T__39 | T__40 | T__41 | T__42 | T__43 | T__44 | T__86 | IDENT 
						=> {
							{
							/*InvokeRule component_clause1*/
							recog.base.set_state(507);
							recog.component_clause1()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 T__32 
				=> {
					{
					/*InvokeRule element_replaceable*/
					recog.base.set_state(510);
					recog.element_replaceable()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- element_replaceable ----------------
pub type Element_replaceableContextAll<'input> = Element_replaceableContext<'input>;


pub type Element_replaceableContext<'input> = BaseParserRuleContext<'input,Element_replaceableContextExt<'input>>;

#[derive(Clone)]
pub struct Element_replaceableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Element_replaceableContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Element_replaceableContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_element_replaceable(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_element_replaceable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Element_replaceableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element_replaceable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element_replaceable }
}
antlr_rust::tid!{Element_replaceableContextExt<'a>}

impl<'input> Element_replaceableContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Element_replaceableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Element_replaceableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Element_replaceableContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Element_replaceableContextExt<'input>>{

fn short_class_definition(&self) -> Option<Rc<Short_class_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_clause1(&self) -> Option<Rc<Component_clause1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constraining_clause(&self) -> Option<Rc<Constraining_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Element_replaceableContextAttrs<'input> for Element_replaceableContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn element_replaceable(&mut self,)
	-> Result<Rc<Element_replaceableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Element_replaceableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_element_replaceable);
        let mut _localctx: Rc<Element_replaceableContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(513);
			recog.base.match_token(T__32,&mut recog.err_handler)?;

			recog.base.set_state(516);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__4 | T__5 | T__6 | T__7 | T__8 | T__9 | T__10 | T__11 | T__12 | T__13 |
			 T__14 | T__15 | T__16 
				=> {
					{
					/*InvokeRule short_class_definition*/
					recog.base.set_state(514);
					recog.short_class_definition()?;

					}
				}

			 T__38 | T__39 | T__40 | T__41 | T__42 | T__43 | T__44 | T__86 | IDENT 
				=> {
					{
					/*InvokeRule component_clause1*/
					recog.base.set_state(515);
					recog.component_clause1()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(519);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__37 {
				{
				/*InvokeRule constraining_clause*/
				recog.base.set_state(518);
				recog.constraining_clause()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_clause1 ----------------
pub type Component_clause1ContextAll<'input> = Component_clause1Context<'input>;


pub type Component_clause1Context<'input> = BaseParserRuleContext<'input,Component_clause1ContextExt<'input>>;

#[derive(Clone)]
pub struct Component_clause1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_clause1Context<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_clause1Context<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_clause1(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_clause1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_clause1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_clause1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_clause1 }
}
antlr_rust::tid!{Component_clause1ContextExt<'a>}

impl<'input> Component_clause1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_clause1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_clause1ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_clause1ContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_clause1ContextExt<'input>>{

fn type_prefix(&self) -> Option<Rc<Type_prefixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_specifier(&self) -> Option<Rc<Type_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_declaration1(&self) -> Option<Rc<Component_declaration1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Component_clause1ContextAttrs<'input> for Component_clause1Context<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_clause1(&mut self,)
	-> Result<Rc<Component_clause1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_clause1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_component_clause1);
        let mut _localctx: Rc<Component_clause1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type_prefix*/
			recog.base.set_state(521);
			recog.type_prefix()?;

			/*InvokeRule type_specifier*/
			recog.base.set_state(522);
			recog.type_specifier()?;

			/*InvokeRule component_declaration1*/
			recog.base.set_state(523);
			recog.component_declaration1()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_declaration1 ----------------
pub type Component_declaration1ContextAll<'input> = Component_declaration1Context<'input>;


pub type Component_declaration1Context<'input> = BaseParserRuleContext<'input,Component_declaration1ContextExt<'input>>;

#[derive(Clone)]
pub struct Component_declaration1ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_declaration1Context<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_declaration1Context<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_declaration1(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_declaration1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_declaration1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_declaration1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_declaration1 }
}
antlr_rust::tid!{Component_declaration1ContextExt<'a>}

impl<'input> Component_declaration1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_declaration1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_declaration1ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_declaration1ContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_declaration1ContextExt<'input>>{

fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Component_declaration1ContextAttrs<'input> for Component_declaration1Context<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_declaration1(&mut self,)
	-> Result<Rc<Component_declaration1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_declaration1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_component_declaration1);
        let mut _localctx: Rc<Component_declaration1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule declaration*/
			recog.base.set_state(525);
			recog.declaration()?;

			/*InvokeRule comment*/
			recog.base.set_state(526);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- short_class_definition ----------------
pub type Short_class_definitionContextAll<'input> = Short_class_definitionContext<'input>;


pub type Short_class_definitionContext<'input> = BaseParserRuleContext<'input,Short_class_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Short_class_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Short_class_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Short_class_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_short_class_definition(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_short_class_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Short_class_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_short_class_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_short_class_definition }
}
antlr_rust::tid!{Short_class_definitionContextExt<'a>}

impl<'input> Short_class_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Short_class_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Short_class_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Short_class_definitionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Short_class_definitionContextExt<'input>>{

fn class_prefixes(&self) -> Option<Rc<Class_prefixesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn short_class_specifier(&self) -> Option<Rc<Short_class_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Short_class_definitionContextAttrs<'input> for Short_class_definitionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn short_class_definition(&mut self,)
	-> Result<Rc<Short_class_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Short_class_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_short_class_definition);
        let mut _localctx: Rc<Short_class_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule class_prefixes*/
			recog.base.set_state(528);
			recog.class_prefixes()?;

			/*InvokeRule short_class_specifier*/
			recog.base.set_state(529);
			recog.short_class_specifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equation_section ----------------
pub type Equation_sectionContextAll<'input> = Equation_sectionContext<'input>;


pub type Equation_sectionContext<'input> = BaseParserRuleContext<'input,Equation_sectionContextExt<'input>>;

#[derive(Clone)]
pub struct Equation_sectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Equation_sectionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Equation_sectionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equation_section(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_equation_section(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Equation_sectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equation_section }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equation_section }
}
antlr_rust::tid!{Equation_sectionContextExt<'a>}

impl<'input> Equation_sectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Equation_sectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Equation_sectionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Equation_sectionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Equation_sectionContextExt<'input>>{

fn equation_all(&self) ->  Vec<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Equation_sectionContextAttrs<'input> for Equation_sectionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equation_section(&mut self,)
	-> Result<Rc<Equation_sectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Equation_sectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_equation_section);
        let mut _localctx: Rc<Equation_sectionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__48 {
				{
				recog.base.set_state(531);
				recog.base.match_token(T__48,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(534);
			recog.base.match_token(T__49,&mut recog.err_handler)?;

			recog.base.set_state(540);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule equation*/
					recog.base.set_state(535);
					recog.equation()?;

					recog.base.set_state(536);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(542);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(69,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- algorithm_section ----------------
pub type Algorithm_sectionContextAll<'input> = Algorithm_sectionContext<'input>;


pub type Algorithm_sectionContext<'input> = BaseParserRuleContext<'input,Algorithm_sectionContextExt<'input>>;

#[derive(Clone)]
pub struct Algorithm_sectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Algorithm_sectionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Algorithm_sectionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_algorithm_section(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_algorithm_section(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Algorithm_sectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_algorithm_section }
	//fn type_rule_index() -> usize where Self: Sized { RULE_algorithm_section }
}
antlr_rust::tid!{Algorithm_sectionContextExt<'a>}

impl<'input> Algorithm_sectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Algorithm_sectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Algorithm_sectionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Algorithm_sectionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Algorithm_sectionContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Algorithm_sectionContextAttrs<'input> for Algorithm_sectionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn algorithm_section(&mut self,)
	-> Result<Rc<Algorithm_sectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Algorithm_sectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_algorithm_section);
        let mut _localctx: Rc<Algorithm_sectionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(544);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__48 {
				{
				recog.base.set_state(543);
				recog.base.match_token(T__48,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(546);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			recog.base.set_state(552);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(547);
				recog.statement()?;

				recog.base.set_state(548);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(554);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equation ----------------
pub type EquationContextAll<'input> = EquationContext<'input>;


pub type EquationContext<'input> = BaseParserRuleContext<'input,EquationContextExt<'input>>;

#[derive(Clone)]
pub struct EquationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for EquationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for EquationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_equation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EquationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equation }
}
antlr_rust::tid!{EquationContextExt<'a>}

impl<'input> EquationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EquationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EquationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EquationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<EquationContextExt<'input>>{

fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn simple_expression(&self) -> Option<Rc<Simple_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn if_equation(&self) -> Option<Rc<If_equationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn for_equation(&self) -> Option<Rc<For_equationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn connect_clause(&self) -> Option<Rc<Connect_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn when_equation(&self) -> Option<Rc<When_equationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_call_args(&self) -> Option<Rc<Function_call_argsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EquationContextAttrs<'input> for EquationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equation(&mut self,)
	-> Result<Rc<EquationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EquationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_equation);
        let mut _localctx: Rc<EquationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(566);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(72,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule simple_expression*/
					recog.base.set_state(555);
					recog.simple_expression()?;

					recog.base.set_state(556);
					recog.base.match_token(T__19,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(557);
					recog.expression()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule if_equation*/
					recog.base.set_state(559);
					recog.if_equation()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule for_equation*/
					recog.base.set_state(560);
					recog.for_equation()?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule connect_clause*/
					recog.base.set_state(561);
					recog.connect_clause()?;

					}
				}
			,
				5 =>{
					{
					/*InvokeRule when_equation*/
					recog.base.set_state(562);
					recog.when_equation()?;

					}
				}
			,
				6 =>{
					{
					/*InvokeRule name*/
					recog.base.set_state(563);
					recog.name()?;

					/*InvokeRule function_call_args*/
					recog.base.set_state(564);
					recog.function_call_args()?;

					}
				}

				_ => {}
			}
			/*InvokeRule comment*/
			recog.base.set_state(568);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn comment(&self) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_reference(&self) -> Option<Rc<Component_referenceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn output_expression_list(&self) -> Option<Rc<Output_expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_call_args(&self) -> Option<Rc<Function_call_argsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn for_statement(&self) -> Option<Rc<For_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn while_statement(&self) -> Option<Rc<While_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn when_statement(&self) -> Option<Rc<When_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__86 | IDENT 
				=> {
					{
					/*InvokeRule component_reference*/
					recog.base.set_state(570);
					recog.component_reference()?;

					recog.base.set_state(574);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__46 
						=> {
							{
							recog.base.set_state(571);
							recog.base.match_token(T__46,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(572);
							recog.expression()?;

							}
						}

					 T__21 
						=> {
							{
							/*InvokeRule function_call_args*/
							recog.base.set_state(573);
							recog.function_call_args()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 T__21 
				=> {
					{
					recog.base.set_state(576);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					/*InvokeRule output_expression_list*/
					recog.base.set_state(577);
					recog.output_expression_list()?;

					recog.base.set_state(578);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					recog.base.set_state(579);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					/*InvokeRule component_reference*/
					recog.base.set_state(580);
					recog.component_reference()?;

					/*InvokeRule function_call_args*/
					recog.base.set_state(581);
					recog.function_call_args()?;

					}
				}

			 T__51 
				=> {
					{
					recog.base.set_state(583);
					recog.base.match_token(T__51,&mut recog.err_handler)?;

					}
				}

			 T__52 
				=> {
					{
					recog.base.set_state(584);
					recog.base.match_token(T__52,&mut recog.err_handler)?;

					}
				}

			 T__45 
				=> {
					{
					/*InvokeRule if_statement*/
					recog.base.set_state(585);
					recog.if_statement()?;

					}
				}

			 T__56 
				=> {
					{
					/*InvokeRule for_statement*/
					recog.base.set_state(586);
					recog.for_statement()?;

					}
				}

			 T__59 
				=> {
					{
					/*InvokeRule while_statement*/
					recog.base.set_state(587);
					recog.while_statement()?;

					}
				}

			 T__60 
				=> {
					{
					/*InvokeRule when_statement*/
					recog.base.set_state(588);
					recog.when_statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule comment*/
			recog.base.set_state(591);
			recog.comment()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_equation ----------------
pub type If_equationContextAll<'input> = If_equationContext<'input>;


pub type If_equationContext<'input> = BaseParserRuleContext<'input,If_equationContextExt<'input>>;

#[derive(Clone)]
pub struct If_equationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for If_equationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for If_equationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_equation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_if_equation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for If_equationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_equation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_equation }
}
antlr_rust::tid!{If_equationContextExt<'a>}

impl<'input> If_equationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_equationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_equationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait If_equationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<If_equationContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn equation_all(&self) ->  Vec<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> If_equationContextAttrs<'input> for If_equationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_equation(&mut self,)
	-> Result<Rc<If_equationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_equationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_if_equation);
        let mut _localctx: Rc<If_equationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(593);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(594);
			recog.expression()?;

			recog.base.set_state(595);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			recog.base.set_state(601);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(75,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule equation*/
					recog.base.set_state(596);
					recog.equation()?;

					recog.base.set_state(597);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(603);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(75,&mut recog.base)?;
			}
			recog.base.set_state(617);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__54 {
				{
				{
				recog.base.set_state(604);
				recog.base.match_token(T__54,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(605);
				recog.expression()?;

				recog.base.set_state(606);
				recog.base.match_token(T__53,&mut recog.err_handler)?;

				recog.base.set_state(612);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(76,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						/*InvokeRule equation*/
						recog.base.set_state(607);
						recog.equation()?;

						recog.base.set_state(608);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						}
						} 
					}
					recog.base.set_state(614);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(76,&mut recog.base)?;
				}
				}
				}
				recog.base.set_state(619);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(629);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__55 {
				{
				recog.base.set_state(620);
				recog.base.match_token(T__55,&mut recog.err_handler)?;

				recog.base.set_state(626);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(78,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						/*InvokeRule equation*/
						recog.base.set_state(621);
						recog.equation()?;

						recog.base.set_state(622);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						}
						} 
					}
					recog.base.set_state(628);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(78,&mut recog.base)?;
				}
				}
			}

			recog.base.set_state(631);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(632);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_statement ----------------
pub type If_statementContextAll<'input> = If_statementContext<'input>;


pub type If_statementContext<'input> = BaseParserRuleContext<'input,If_statementContextExt<'input>>;

#[derive(Clone)]
pub struct If_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for If_statementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for If_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_statement(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_if_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr_rust::tid!{If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait If_statementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<If_statementContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_statement(&mut self,)
	-> Result<Rc<If_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(634);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(635);
			recog.expression()?;

			recog.base.set_state(636);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			recog.base.set_state(642);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(637);
				recog.statement()?;

				recog.base.set_state(638);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(644);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(658);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__54 {
				{
				{
				recog.base.set_state(645);
				recog.base.match_token(T__54,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(646);
				recog.expression()?;

				recog.base.set_state(647);
				recog.base.match_token(T__53,&mut recog.err_handler)?;

				recog.base.set_state(653);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(648);
					recog.statement()?;

					recog.base.set_state(649);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(655);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
				}
				recog.base.set_state(660);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(670);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__55 {
				{
				recog.base.set_state(661);
				recog.base.match_token(T__55,&mut recog.err_handler)?;

				recog.base.set_state(667);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(662);
					recog.statement()?;

					recog.base.set_state(663);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(669);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(672);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(673);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- for_equation ----------------
pub type For_equationContextAll<'input> = For_equationContext<'input>;


pub type For_equationContext<'input> = BaseParserRuleContext<'input,For_equationContextExt<'input>>;

#[derive(Clone)]
pub struct For_equationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for For_equationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for For_equationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_for_equation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_for_equation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for For_equationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_for_equation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_for_equation }
}
antlr_rust::tid!{For_equationContextExt<'a>}

impl<'input> For_equationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<For_equationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,For_equationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait For_equationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<For_equationContextExt<'input>>{

fn for_indices(&self) -> Option<Rc<For_indicesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equation_all(&self) ->  Vec<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> For_equationContextAttrs<'input> for For_equationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn for_equation(&mut self,)
	-> Result<Rc<For_equationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = For_equationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_for_equation);
        let mut _localctx: Rc<For_equationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(675);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			/*InvokeRule for_indices*/
			recog.base.set_state(676);
			recog.for_indices()?;

			recog.base.set_state(677);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(683);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule equation*/
					recog.base.set_state(678);
					recog.equation()?;

					recog.base.set_state(679);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(685);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(85,&mut recog.base)?;
			}
			recog.base.set_state(686);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(687);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- for_statement ----------------
pub type For_statementContextAll<'input> = For_statementContext<'input>;


pub type For_statementContext<'input> = BaseParserRuleContext<'input,For_statementContextExt<'input>>;

#[derive(Clone)]
pub struct For_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for For_statementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for For_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_for_statement(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_for_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for For_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_for_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_for_statement }
}
antlr_rust::tid!{For_statementContextExt<'a>}

impl<'input> For_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<For_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,For_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait For_statementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<For_statementContextExt<'input>>{

fn for_indices(&self) -> Option<Rc<For_indicesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> For_statementContextAttrs<'input> for For_statementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn for_statement(&mut self,)
	-> Result<Rc<For_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = For_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_for_statement);
        let mut _localctx: Rc<For_statementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(689);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			/*InvokeRule for_indices*/
			recog.base.set_state(690);
			recog.for_indices()?;

			recog.base.set_state(691);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(697);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(692);
				recog.statement()?;

				recog.base.set_state(693);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(699);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(700);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(701);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- for_indices ----------------
pub type For_indicesContextAll<'input> = For_indicesContext<'input>;


pub type For_indicesContext<'input> = BaseParserRuleContext<'input,For_indicesContextExt<'input>>;

#[derive(Clone)]
pub struct For_indicesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for For_indicesContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for For_indicesContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_for_indices(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_for_indices(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for For_indicesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_for_indices }
	//fn type_rule_index() -> usize where Self: Sized { RULE_for_indices }
}
antlr_rust::tid!{For_indicesContextExt<'a>}

impl<'input> For_indicesContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<For_indicesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,For_indicesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait For_indicesContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<For_indicesContextExt<'input>>{

fn for_index_all(&self) ->  Vec<Rc<For_indexContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn for_index(&self, i: usize) -> Option<Rc<For_indexContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> For_indicesContextAttrs<'input> for For_indicesContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn for_indices(&mut self,)
	-> Result<Rc<For_indicesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = For_indicesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_for_indices);
        let mut _localctx: Rc<For_indicesContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule for_index*/
			recog.base.set_state(703);
			recog.for_index()?;

			recog.base.set_state(708);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(704);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule for_index*/
				recog.base.set_state(705);
				recog.for_index()?;

				}
				}
				recog.base.set_state(710);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- for_index ----------------
pub type For_indexContextAll<'input> = For_indexContext<'input>;


pub type For_indexContext<'input> = BaseParserRuleContext<'input,For_indexContextExt<'input>>;

#[derive(Clone)]
pub struct For_indexContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for For_indexContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for For_indexContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_for_index(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_for_index(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for For_indexContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_for_index }
	//fn type_rule_index() -> usize where Self: Sized { RULE_for_index }
}
antlr_rust::tid!{For_indexContextExt<'a>}

impl<'input> For_indexContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<For_indexContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,For_indexContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait For_indexContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<For_indexContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> For_indexContextAttrs<'input> for For_indexContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn for_index(&mut self,)
	-> Result<Rc<For_indexContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = For_indexContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_for_index);
        let mut _localctx: Rc<For_indexContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(711);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(714);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__58 {
				{
				recog.base.set_state(712);
				recog.base.match_token(T__58,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(713);
				recog.expression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- while_statement ----------------
pub type While_statementContextAll<'input> = While_statementContext<'input>;


pub type While_statementContext<'input> = BaseParserRuleContext<'input,While_statementContextExt<'input>>;

#[derive(Clone)]
pub struct While_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for While_statementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for While_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_while_statement(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_while_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for While_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_while_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_while_statement }
}
antlr_rust::tid!{While_statementContextExt<'a>}

impl<'input> While_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<While_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,While_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait While_statementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<While_statementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> While_statementContextAttrs<'input> for While_statementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn while_statement(&mut self,)
	-> Result<Rc<While_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = While_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_while_statement);
        let mut _localctx: Rc<While_statementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(716);
			recog.base.match_token(T__59,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(717);
			recog.expression()?;

			recog.base.set_state(718);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			recog.base.set_state(724);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(719);
				recog.statement()?;

				recog.base.set_state(720);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(726);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(727);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(728);
			recog.base.match_token(T__59,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- when_equation ----------------
pub type When_equationContextAll<'input> = When_equationContext<'input>;


pub type When_equationContext<'input> = BaseParserRuleContext<'input,When_equationContextExt<'input>>;

#[derive(Clone)]
pub struct When_equationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for When_equationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for When_equationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_when_equation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_when_equation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for When_equationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_when_equation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_when_equation }
}
antlr_rust::tid!{When_equationContextExt<'a>}

impl<'input> When_equationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<When_equationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,When_equationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait When_equationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<When_equationContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn equation_all(&self) ->  Vec<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> When_equationContextAttrs<'input> for When_equationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn when_equation(&mut self,)
	-> Result<Rc<When_equationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = When_equationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_when_equation);
        let mut _localctx: Rc<When_equationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(730);
			recog.base.match_token(T__60,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(731);
			recog.expression()?;

			recog.base.set_state(732);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			recog.base.set_state(738);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule equation*/
					recog.base.set_state(733);
					recog.equation()?;

					recog.base.set_state(734);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(740);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(90,&mut recog.base)?;
			}
			recog.base.set_state(754);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__61 {
				{
				{
				recog.base.set_state(741);
				recog.base.match_token(T__61,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(742);
				recog.expression()?;

				recog.base.set_state(743);
				recog.base.match_token(T__53,&mut recog.err_handler)?;

				recog.base.set_state(749);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(91,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						/*InvokeRule equation*/
						recog.base.set_state(744);
						recog.equation()?;

						recog.base.set_state(745);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						}
						} 
					}
					recog.base.set_state(751);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(91,&mut recog.base)?;
				}
				}
				}
				recog.base.set_state(756);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(757);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(758);
			recog.base.match_token(T__60,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- when_statement ----------------
pub type When_statementContextAll<'input> = When_statementContext<'input>;


pub type When_statementContext<'input> = BaseParserRuleContext<'input,When_statementContextExt<'input>>;

#[derive(Clone)]
pub struct When_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for When_statementContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for When_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_when_statement(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_when_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for When_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_when_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_when_statement }
}
antlr_rust::tid!{When_statementContextExt<'a>}

impl<'input> When_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<When_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,When_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait When_statementContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<When_statementContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> When_statementContextAttrs<'input> for When_statementContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn when_statement(&mut self,)
	-> Result<Rc<When_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = When_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_when_statement);
        let mut _localctx: Rc<When_statementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(760);
			recog.base.match_token(T__60,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(761);
			recog.expression()?;

			recog.base.set_state(762);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			recog.base.set_state(768);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(763);
				recog.statement()?;

				recog.base.set_state(764);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(770);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(784);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__61 {
				{
				{
				recog.base.set_state(771);
				recog.base.match_token(T__61,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(772);
				recog.expression()?;

				recog.base.set_state(773);
				recog.base.match_token(T__53,&mut recog.err_handler)?;

				recog.base.set_state(779);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__21 || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__51 - 46)) | (1usize << (T__52 - 46)) | (1usize << (T__56 - 46)) | (1usize << (T__59 - 46)) | (1usize << (T__60 - 46)))) != 0) || _la==T__86 || _la==IDENT {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(774);
					recog.statement()?;

					recog.base.set_state(775);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(781);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
				}
				recog.base.set_state(786);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(787);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			recog.base.set_state(788);
			recog.base.match_token(T__60,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- connect_clause ----------------
pub type Connect_clauseContextAll<'input> = Connect_clauseContext<'input>;


pub type Connect_clauseContext<'input> = BaseParserRuleContext<'input,Connect_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Connect_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Connect_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Connect_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_connect_clause(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_connect_clause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Connect_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_connect_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_connect_clause }
}
antlr_rust::tid!{Connect_clauseContextExt<'a>}

impl<'input> Connect_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Connect_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Connect_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Connect_clauseContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Connect_clauseContextExt<'input>>{

fn component_reference_all(&self) ->  Vec<Rc<Component_referenceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn component_reference(&self, i: usize) -> Option<Rc<Component_referenceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Connect_clauseContextAttrs<'input> for Connect_clauseContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn connect_clause(&mut self,)
	-> Result<Rc<Connect_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Connect_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_connect_clause);
        let mut _localctx: Rc<Connect_clauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(790);
			recog.base.match_token(T__62,&mut recog.err_handler)?;

			recog.base.set_state(791);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule component_reference*/
			recog.base.set_state(792);
			recog.component_reference()?;

			recog.base.set_state(793);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			/*InvokeRule component_reference*/
			recog.base.set_state(794);
			recog.component_reference()?;

			recog.base.set_state(795);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn simple_expression(&self) -> Option<Rc<Simple_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(815);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__17 | T__21 | T__24 | T__48 | T__65 | T__72 | T__73 | T__74 | T__75 |
			 T__81 | T__82 | T__83 | T__85 | T__86 | IDENT | STRING | UNSIGNED_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule simple_expression*/
					recog.base.set_state(797);
					recog.simple_expression()?;

					}
				}

			 T__45 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(798);
					recog.base.match_token(T__45,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(799);
					recog.expression()?;

					recog.base.set_state(800);
					recog.base.match_token(T__53,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(801);
					recog.expression()?;

					recog.base.set_state(809);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__54 {
						{
						{
						recog.base.set_state(802);
						recog.base.match_token(T__54,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(803);
						recog.expression()?;

						recog.base.set_state(804);
						recog.base.match_token(T__53,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(805);
						recog.expression()?;

						}
						}
						recog.base.set_state(811);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(812);
					recog.base.match_token(T__55,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(813);
					recog.expression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- simple_expression ----------------
pub type Simple_expressionContextAll<'input> = Simple_expressionContext<'input>;


pub type Simple_expressionContext<'input> = BaseParserRuleContext<'input,Simple_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Simple_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Simple_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_simple_expression(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_simple_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Simple_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_simple_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_simple_expression }
}
antlr_rust::tid!{Simple_expressionContextExt<'a>}

impl<'input> Simple_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Simple_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Simple_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Simple_expressionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Simple_expressionContextExt<'input>>{

fn logical_expression_all(&self) ->  Vec<Rc<Logical_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logical_expression(&self, i: usize) -> Option<Rc<Logical_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Simple_expressionContextAttrs<'input> for Simple_expressionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn simple_expression(&mut self,)
	-> Result<Rc<Simple_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Simple_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_simple_expression);
        let mut _localctx: Rc<Simple_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logical_expression*/
			recog.base.set_state(817);
			recog.logical_expression()?;

			recog.base.set_state(824);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(818);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				/*InvokeRule logical_expression*/
				recog.base.set_state(819);
				recog.logical_expression()?;

				recog.base.set_state(822);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__22 {
					{
					recog.base.set_state(820);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule logical_expression*/
					recog.base.set_state(821);
					recog.logical_expression()?;

					}
				}

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logical_expression ----------------
pub type Logical_expressionContextAll<'input> = Logical_expressionContext<'input>;


pub type Logical_expressionContext<'input> = BaseParserRuleContext<'input,Logical_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Logical_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Logical_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_expression(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_logical_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Logical_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_expression }
}
antlr_rust::tid!{Logical_expressionContextExt<'a>}

impl<'input> Logical_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_expressionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Logical_expressionContextExt<'input>>{

fn logical_term_all(&self) ->  Vec<Rc<Logical_termContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logical_term(&self, i: usize) -> Option<Rc<Logical_termContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Logical_expressionContextAttrs<'input> for Logical_expressionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_expression(&mut self,)
	-> Result<Rc<Logical_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_logical_expression);
        let mut _localctx: Rc<Logical_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logical_term*/
			recog.base.set_state(826);
			recog.logical_term()?;

			recog.base.set_state(831);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__63 {
				{
				{
				recog.base.set_state(827);
				recog.base.match_token(T__63,&mut recog.err_handler)?;

				/*InvokeRule logical_term*/
				recog.base.set_state(828);
				recog.logical_term()?;

				}
				}
				recog.base.set_state(833);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logical_term ----------------
pub type Logical_termContextAll<'input> = Logical_termContext<'input>;


pub type Logical_termContext<'input> = BaseParserRuleContext<'input,Logical_termContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_termContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Logical_termContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Logical_termContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_term(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_logical_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Logical_termContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_term }
}
antlr_rust::tid!{Logical_termContextExt<'a>}

impl<'input> Logical_termContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_termContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_termContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_termContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Logical_termContextExt<'input>>{

fn logical_factor_all(&self) ->  Vec<Rc<Logical_factorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logical_factor(&self, i: usize) -> Option<Rc<Logical_factorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Logical_termContextAttrs<'input> for Logical_termContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_term(&mut self,)
	-> Result<Rc<Logical_termContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_termContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_logical_term);
        let mut _localctx: Rc<Logical_termContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logical_factor*/
			recog.base.set_state(834);
			recog.logical_factor()?;

			recog.base.set_state(839);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__64 {
				{
				{
				recog.base.set_state(835);
				recog.base.match_token(T__64,&mut recog.err_handler)?;

				/*InvokeRule logical_factor*/
				recog.base.set_state(836);
				recog.logical_factor()?;

				}
				}
				recog.base.set_state(841);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logical_factor ----------------
pub type Logical_factorContextAll<'input> = Logical_factorContext<'input>;


pub type Logical_factorContext<'input> = BaseParserRuleContext<'input,Logical_factorContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_factorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Logical_factorContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Logical_factorContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_factor(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_logical_factor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Logical_factorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_factor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_factor }
}
antlr_rust::tid!{Logical_factorContextExt<'a>}

impl<'input> Logical_factorContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_factorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_factorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_factorContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Logical_factorContextExt<'input>>{

fn relation(&self) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Logical_factorContextAttrs<'input> for Logical_factorContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_factor(&mut self,)
	-> Result<Rc<Logical_factorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_factorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_logical_factor);
        let mut _localctx: Rc<Logical_factorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(843);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__65 {
				{
				recog.base.set_state(842);
				recog.base.match_token(T__65,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule relation*/
			recog.base.set_state(845);
			recog.relation()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation ----------------
pub type RelationContextAll<'input> = RelationContext<'input>;


pub type RelationContext<'input> = BaseParserRuleContext<'input,RelationContextExt<'input>>;

#[derive(Clone)]
pub struct RelationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for RelationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for RelationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_relation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RelationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation }
}
antlr_rust::tid!{RelationContextExt<'a>}

impl<'input> RelationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<RelationContextExt<'input>>{

fn arithmetic_expression_all(&self) ->  Vec<Rc<Arithmetic_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn arithmetic_expression(&self, i: usize) -> Option<Rc<Arithmetic_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn rel_op(&self) -> Option<Rc<Rel_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RelationContextAttrs<'input> for RelationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn relation(&mut self,)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_relation);
        let mut _localctx: Rc<RelationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule arithmetic_expression*/
			recog.base.set_state(847);
			recog.arithmetic_expression()?;

			recog.base.set_state(851);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__68 - 67)) | (1usize << (T__69 - 67)) | (1usize << (T__70 - 67)) | (1usize << (T__71 - 67)))) != 0) {
				{
				/*InvokeRule rel_op*/
				recog.base.set_state(848);
				recog.rel_op()?;

				/*InvokeRule arithmetic_expression*/
				recog.base.set_state(849);
				recog.arithmetic_expression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- rel_op ----------------
pub type Rel_opContextAll<'input> = Rel_opContext<'input>;


pub type Rel_opContext<'input> = BaseParserRuleContext<'input,Rel_opContextExt<'input>>;

#[derive(Clone)]
pub struct Rel_opContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Rel_opContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Rel_opContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_rel_op(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_rel_op(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Rel_opContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rel_op }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rel_op }
}
antlr_rust::tid!{Rel_opContextExt<'a>}

impl<'input> Rel_opContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Rel_opContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Rel_opContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Rel_opContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Rel_opContextExt<'input>>{


}

impl<'input> Rel_opContextAttrs<'input> for Rel_opContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn rel_op(&mut self,)
	-> Result<Rc<Rel_opContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Rel_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_rel_op);
        let mut _localctx: Rc<Rel_opContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(853);
			_la = recog.base.input.la(1);
			if { !(((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (T__66 - 67)) | (1usize << (T__67 - 67)) | (1usize << (T__68 - 67)) | (1usize << (T__69 - 67)) | (1usize << (T__70 - 67)) | (1usize << (T__71 - 67)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arithmetic_expression ----------------
pub type Arithmetic_expressionContextAll<'input> = Arithmetic_expressionContext<'input>;


pub type Arithmetic_expressionContext<'input> = BaseParserRuleContext<'input,Arithmetic_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Arithmetic_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Arithmetic_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Arithmetic_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arithmetic_expression(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_arithmetic_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Arithmetic_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithmetic_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithmetic_expression }
}
antlr_rust::tid!{Arithmetic_expressionContextExt<'a>}

impl<'input> Arithmetic_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Arithmetic_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Arithmetic_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Arithmetic_expressionContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Arithmetic_expressionContextExt<'input>>{

fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn add_op_all(&self) ->  Vec<Rc<Add_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn add_op(&self, i: usize) -> Option<Rc<Add_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Arithmetic_expressionContextAttrs<'input> for Arithmetic_expressionContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	#[allow(unused_parens)]
	pub fn arithmetic_expression(&mut self,)
	-> Result<Rc<Arithmetic_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Arithmetic_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_arithmetic_expression);
        let mut _localctx: Rc<Arithmetic_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(856);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (T__72 - 73)) | (1usize << (T__73 - 73)) | (1usize << (T__74 - 73)) | (1usize << (T__75 - 73)))) != 0) {
				{
				/*InvokeRule add_op*/
				recog.base.set_state(855);
				recog.add_op()?;

				}
			}

			/*InvokeRule term*/
			recog.base.set_state(858);
			recog.term()?;

			recog.base.set_state(864);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (T__72 - 73)) | (1usize << (T__73 - 73)) | (1usize << (T__74 - 73)) | (1usize << (T__75 - 73)))) != 0) {
				{
				{
				/*InvokeRule add_op*/
				recog.base.set_state(859);
				recog.add_op()?;

				/*InvokeRule term*/
				recog.base.set_state(860);
				recog.term()?;

				}
				}
				recog.base.set_state(866);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- add_op ----------------
pub type Add_opContextAll<'input> = Add_opContext<'input>;


pub type Add_opContext<'input> = BaseParserRuleContext<'input,Add_opContextExt<'input>>;

#[derive(Clone)]
pub struct Add_opContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Add_opContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Add_opContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_add_op(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_add_op(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Add_opContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_add_op }
	//fn type_rule_index() -> usize where Self: Sized { RULE_add_op }
}
antlr_rust::tid!{Add_opContextExt<'a>}

impl<'input> Add_opContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Add_opContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Add_opContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Add_opContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Add_opContextExt<'input>>{


}

impl<'input> Add_opContextAttrs<'input> for Add_opContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn add_op(&mut self,)
	-> Result<Rc<Add_opContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Add_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_add_op);
        let mut _localctx: Rc<Add_opContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(867);
			_la = recog.base.input.la(1);
			if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (T__72 - 73)) | (1usize << (T__73 - 73)) | (1usize << (T__74 - 73)) | (1usize << (T__75 - 73)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- term ----------------
pub type TermContextAll<'input> = TermContext<'input>;


pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for TermContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_term(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_term(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TermContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<TermContextExt<'input>>{

fn factor_all(&self) ->  Vec<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn factor(&self, i: usize) -> Option<Rc<FactorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn mul_op_all(&self) ->  Vec<Rc<Mul_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mul_op(&self, i: usize) -> Option<Rc<Mul_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_term);
        let mut _localctx: Rc<TermContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule factor*/
			recog.base.set_state(869);
			recog.factor()?;

			recog.base.set_state(875);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__34 || ((((_la - 77)) & !0x3f) == 0 && ((1usize << (_la - 77)) & ((1usize << (T__76 - 77)) | (1usize << (T__77 - 77)) | (1usize << (T__78 - 77)))) != 0) {
				{
				{
				/*InvokeRule mul_op*/
				recog.base.set_state(870);
				recog.mul_op()?;

				/*InvokeRule factor*/
				recog.base.set_state(871);
				recog.factor()?;

				}
				}
				recog.base.set_state(877);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mul_op ----------------
pub type Mul_opContextAll<'input> = Mul_opContext<'input>;


pub type Mul_opContext<'input> = BaseParserRuleContext<'input,Mul_opContextExt<'input>>;

#[derive(Clone)]
pub struct Mul_opContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Mul_opContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Mul_opContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mul_op(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_mul_op(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Mul_opContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mul_op }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mul_op }
}
antlr_rust::tid!{Mul_opContextExt<'a>}

impl<'input> Mul_opContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mul_opContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mul_opContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mul_opContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Mul_opContextExt<'input>>{


}

impl<'input> Mul_opContextAttrs<'input> for Mul_opContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mul_op(&mut self,)
	-> Result<Rc<Mul_opContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mul_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_mul_op);
        let mut _localctx: Rc<Mul_opContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(878);
			_la = recog.base.input.la(1);
			if { !(_la==T__34 || ((((_la - 77)) & !0x3f) == 0 && ((1usize << (_la - 77)) & ((1usize << (T__76 - 77)) | (1usize << (T__77 - 77)) | (1usize << (T__78 - 77)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- factor ----------------
pub type FactorContextAll<'input> = FactorContext<'input>;


pub type FactorContext<'input> = BaseParserRuleContext<'input,FactorContextExt<'input>>;

#[derive(Clone)]
pub struct FactorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for FactorContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for FactorContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_factor(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_factor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FactorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_factor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_factor }
}
antlr_rust::tid!{FactorContextExt<'a>}

impl<'input> FactorContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FactorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FactorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FactorContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<FactorContextExt<'input>>{

fn primary_all(&self) ->  Vec<Rc<PrimaryContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn primary(&self, i: usize) -> Option<Rc<PrimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FactorContextAttrs<'input> for FactorContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn factor(&mut self,)
	-> Result<Rc<FactorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FactorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_factor);
        let mut _localctx: Rc<FactorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule primary*/
			recog.base.set_state(880);
			recog.primary()?;

			recog.base.set_state(883);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__79 || _la==T__80 {
				{
				recog.base.set_state(881);
				_la = recog.base.input.la(1);
				if { !(_la==T__79 || _la==T__80) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule primary*/
				recog.base.set_state(882);
				recog.primary()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primary ----------------
pub type PrimaryContextAll<'input> = PrimaryContext<'input>;


pub type PrimaryContext<'input> = BaseParserRuleContext<'input,PrimaryContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for PrimaryContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for PrimaryContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primary(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_primary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PrimaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}
antlr_rust::tid!{PrimaryContextExt<'a>}

impl<'input> PrimaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<PrimaryContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNSIGNED_NUMBER
/// Returns `None` if there is no child corresponding to token UNSIGNED_NUMBER
fn UNSIGNED_NUMBER(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(UNSIGNED_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn function_call_args(&self) -> Option<Rc<Function_call_argsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_reference(&self) -> Option<Rc<Component_referenceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn output_expression_list(&self) -> Option<Rc<Output_expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_list_all(&self) ->  Vec<Rc<Expression_listContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression_list(&self, i: usize) -> Option<Rc<Expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn function_arguments(&self) -> Option<Rc<Function_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryContextAttrs<'input> for PrimaryContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary(&mut self,)
	-> Result<Rc<PrimaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_primary);
        let mut _localctx: Rc<PrimaryContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(916);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(110,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(885);
					recog.base.match_token(UNSIGNED_NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(886);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(887);
					recog.base.match_token(T__81,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(888);
					recog.base.match_token(T__82,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(892);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__86 | IDENT 
						=> {
							{
							/*InvokeRule name*/
							recog.base.set_state(889);
							recog.name()?;

							}
						}

					 T__24 
						=> {
							{
							recog.base.set_state(890);
							recog.base.match_token(T__24,&mut recog.err_handler)?;

							}
						}

					 T__48 
						=> {
							{
							recog.base.set_state(891);
							recog.base.match_token(T__48,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					/*InvokeRule function_call_args*/
					recog.base.set_state(894);
					recog.function_call_args()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule component_reference*/
					recog.base.set_state(895);
					recog.component_reference()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(896);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					/*InvokeRule output_expression_list*/
					recog.base.set_state(897);
					recog.output_expression_list()?;

					recog.base.set_state(898);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(900);
					recog.base.match_token(T__83,&mut recog.err_handler)?;

					/*InvokeRule expression_list*/
					recog.base.set_state(901);
					recog.expression_list()?;

					recog.base.set_state(906);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 {
						{
						{
						recog.base.set_state(902);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule expression_list*/
						recog.base.set_state(903);
						recog.expression_list()?;

						}
						}
						recog.base.set_state(908);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(909);
					recog.base.match_token(T__84,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(911);
					recog.base.match_token(T__85,&mut recog.err_handler)?;

					/*InvokeRule function_arguments*/
					recog.base.set_state(912);
					recog.function_arguments()?;

					recog.base.set_state(913);
					recog.base.match_token(T__36,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(915);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;


pub type NameContext<'input> = BaseParserRuleContext<'input,NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for NameContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for NameContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_name(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr_rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NameContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<NameContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}

}

impl<'input> NameContextAttrs<'input> for NameContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn name(&mut self,)
	-> Result<Rc<NameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(919);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__86 {
				{
				recog.base.set_state(918);
				recog.base.match_token(T__86,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(921);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(926);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__86 {
				{
				{
				recog.base.set_state(922);
				recog.base.match_token(T__86,&mut recog.err_handler)?;

				recog.base.set_state(923);
				recog.base.match_token(IDENT,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(928);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_reference ----------------
pub type Component_referenceContextAll<'input> = Component_referenceContext<'input>;


pub type Component_referenceContext<'input> = BaseParserRuleContext<'input,Component_referenceContextExt<'input>>;

#[derive(Clone)]
pub struct Component_referenceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Component_referenceContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Component_referenceContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_component_reference(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_component_reference(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Component_referenceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_reference }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_reference }
}
antlr_rust::tid!{Component_referenceContextExt<'a>}

impl<'input> Component_referenceContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_referenceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_referenceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_referenceContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Component_referenceContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}
fn array_subscripts_all(&self) ->  Vec<Rc<Array_subscriptsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn array_subscripts(&self, i: usize) -> Option<Rc<Array_subscriptsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Component_referenceContextAttrs<'input> for Component_referenceContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_reference(&mut self,)
	-> Result<Rc<Component_referenceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_referenceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_component_reference);
        let mut _localctx: Rc<Component_referenceContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(930);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__86 {
				{
				recog.base.set_state(929);
				recog.base.match_token(T__86,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(932);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(934);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__83 {
				{
				/*InvokeRule array_subscripts*/
				recog.base.set_state(933);
				recog.array_subscripts()?;

				}
			}

			recog.base.set_state(943);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__86 {
				{
				{
				recog.base.set_state(936);
				recog.base.match_token(T__86,&mut recog.err_handler)?;

				recog.base.set_state(937);
				recog.base.match_token(IDENT,&mut recog.err_handler)?;

				recog.base.set_state(939);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__83 {
					{
					/*InvokeRule array_subscripts*/
					recog.base.set_state(938);
					recog.array_subscripts()?;

					}
				}

				}
				}
				recog.base.set_state(945);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_call_args ----------------
pub type Function_call_argsContextAll<'input> = Function_call_argsContext<'input>;


pub type Function_call_argsContext<'input> = BaseParserRuleContext<'input,Function_call_argsContextExt<'input>>;

#[derive(Clone)]
pub struct Function_call_argsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Function_call_argsContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Function_call_argsContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_call_args(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_function_call_args(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Function_call_argsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_call_args }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_call_args }
}
antlr_rust::tid!{Function_call_argsContextExt<'a>}

impl<'input> Function_call_argsContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_call_argsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_call_argsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_call_argsContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Function_call_argsContextExt<'input>>{

fn function_arguments(&self) -> Option<Rc<Function_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Function_call_argsContextAttrs<'input> for Function_call_argsContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_call_args(&mut self,)
	-> Result<Rc<Function_call_argsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_call_argsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_function_call_args);
        let mut _localctx: Rc<Function_call_argsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(946);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			recog.base.set_state(948);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__16) | (1usize << T__17) | (1usize << T__21) | (1usize << T__24))) != 0) || ((((_la - 46)) & !0x3f) == 0 && ((1usize << (_la - 46)) & ((1usize << (T__45 - 46)) | (1usize << (T__48 - 46)) | (1usize << (T__65 - 46)) | (1usize << (T__72 - 46)) | (1usize << (T__73 - 46)) | (1usize << (T__74 - 46)) | (1usize << (T__75 - 46)))) != 0) || ((((_la - 82)) & !0x3f) == 0 && ((1usize << (_la - 82)) & ((1usize << (T__81 - 82)) | (1usize << (T__82 - 82)) | (1usize << (T__83 - 82)) | (1usize << (T__85 - 82)) | (1usize << (T__86 - 82)) | (1usize << (IDENT - 82)) | (1usize << (STRING - 82)) | (1usize << (UNSIGNED_NUMBER - 82)))) != 0) {
				{
				/*InvokeRule function_arguments*/
				recog.base.set_state(947);
				recog.function_arguments()?;

				}
			}

			recog.base.set_state(950);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_arguments ----------------
pub type Function_argumentsContextAll<'input> = Function_argumentsContext<'input>;


pub type Function_argumentsContext<'input> = BaseParserRuleContext<'input,Function_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Function_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Function_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Function_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_arguments(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_function_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Function_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_arguments }
}
antlr_rust::tid!{Function_argumentsContextExt<'a>}

impl<'input> Function_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_argumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_argumentsContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Function_argumentsContextExt<'input>>{

fn function_argument(&self) -> Option<Rc<Function_argumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_arguments(&self) -> Option<Rc<Function_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn for_indices(&self) -> Option<Rc<For_indicesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn named_arguments(&self) -> Option<Rc<Named_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Function_argumentsContextAttrs<'input> for Function_argumentsContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_arguments(&mut self,)
	-> Result<Rc<Function_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_function_arguments);
        let mut _localctx: Rc<Function_argumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(960);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(119,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule function_argument*/
					recog.base.set_state(952);
					recog.function_argument()?;

					recog.base.set_state(957);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__25 
						=> {
					    	{
					    	recog.base.set_state(953);
					    	recog.base.match_token(T__25,&mut recog.err_handler)?;

					    	/*InvokeRule function_arguments*/
					    	recog.base.set_state(954);
					    	recog.function_arguments()?;

					    	}
					    }

					 T__56 
						=> {
					    	{
					    	recog.base.set_state(955);
					    	recog.base.match_token(T__56,&mut recog.err_handler)?;

					    	/*InvokeRule for_indices*/
					    	recog.base.set_state(956);
					    	recog.for_indices()?;

					    	}
					    }

					 T__23 | T__36 
						=> {
					    }

						_ => {}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule named_arguments*/
					recog.base.set_state(959);
					recog.named_arguments()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- named_arguments ----------------
pub type Named_argumentsContextAll<'input> = Named_argumentsContext<'input>;


pub type Named_argumentsContext<'input> = BaseParserRuleContext<'input,Named_argumentsContextExt<'input>>;

#[derive(Clone)]
pub struct Named_argumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Named_argumentsContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Named_argumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_named_arguments(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_named_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Named_argumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_named_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_named_arguments }
}
antlr_rust::tid!{Named_argumentsContextExt<'a>}

impl<'input> Named_argumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Named_argumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Named_argumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Named_argumentsContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Named_argumentsContextExt<'input>>{

fn named_argument(&self) -> Option<Rc<Named_argumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn named_arguments(&self) -> Option<Rc<Named_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Named_argumentsContextAttrs<'input> for Named_argumentsContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn named_arguments(&mut self,)
	-> Result<Rc<Named_argumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Named_argumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_named_arguments);
        let mut _localctx: Rc<Named_argumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule named_argument*/
			recog.base.set_state(962);
			recog.named_argument()?;

			recog.base.set_state(965);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__25 {
				{
				recog.base.set_state(963);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule named_arguments*/
				recog.base.set_state(964);
				recog.named_arguments()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- named_argument ----------------
pub type Named_argumentContextAll<'input> = Named_argumentContext<'input>;


pub type Named_argumentContext<'input> = BaseParserRuleContext<'input,Named_argumentContextExt<'input>>;

#[derive(Clone)]
pub struct Named_argumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Named_argumentContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Named_argumentContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_named_argument(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_named_argument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Named_argumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_named_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_named_argument }
}
antlr_rust::tid!{Named_argumentContextExt<'a>}

impl<'input> Named_argumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Named_argumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Named_argumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Named_argumentContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Named_argumentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
fn function_argument(&self) -> Option<Rc<Function_argumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Named_argumentContextAttrs<'input> for Named_argumentContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn named_argument(&mut self,)
	-> Result<Rc<Named_argumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Named_argumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_named_argument);
        let mut _localctx: Rc<Named_argumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(967);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(968);
			recog.base.match_token(T__19,&mut recog.err_handler)?;

			/*InvokeRule function_argument*/
			recog.base.set_state(969);
			recog.function_argument()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_argument ----------------
pub type Function_argumentContextAll<'input> = Function_argumentContext<'input>;


pub type Function_argumentContext<'input> = BaseParserRuleContext<'input,Function_argumentContextExt<'input>>;

#[derive(Clone)]
pub struct Function_argumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Function_argumentContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Function_argumentContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_argument(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_function_argument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Function_argumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_argument }
}
antlr_rust::tid!{Function_argumentContextExt<'a>}

impl<'input> Function_argumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_argumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_argumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_argumentContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Function_argumentContextExt<'input>>{

fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn named_arguments(&self) -> Option<Rc<Named_argumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Function_argumentContextAttrs<'input> for Function_argumentContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_argument(&mut self,)
	-> Result<Rc<Function_argumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_argumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_function_argument);
        let mut _localctx: Rc<Function_argumentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(980);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(971);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					/*InvokeRule name*/
					recog.base.set_state(972);
					recog.name()?;

					recog.base.set_state(973);
					recog.base.match_token(T__21,&mut recog.err_handler)?;

					recog.base.set_state(975);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IDENT {
						{
						/*InvokeRule named_arguments*/
						recog.base.set_state(974);
						recog.named_arguments()?;

						}
					}

					recog.base.set_state(977);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

			 T__17 | T__21 | T__24 | T__45 | T__48 | T__65 | T__72 | T__73 | T__74 |
			 T__75 | T__81 | T__82 | T__83 | T__85 | T__86 | IDENT | STRING | UNSIGNED_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(979);
					recog.expression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- output_expression_list ----------------
pub type Output_expression_listContextAll<'input> = Output_expression_listContext<'input>;


pub type Output_expression_listContext<'input> = BaseParserRuleContext<'input,Output_expression_listContextExt<'input>>;

#[derive(Clone)]
pub struct Output_expression_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Output_expression_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Output_expression_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_output_expression_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_output_expression_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Output_expression_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_output_expression_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_output_expression_list }
}
antlr_rust::tid!{Output_expression_listContextExt<'a>}

impl<'input> Output_expression_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Output_expression_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Output_expression_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Output_expression_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Output_expression_listContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Output_expression_listContextAttrs<'input> for Output_expression_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn output_expression_list(&mut self,)
	-> Result<Rc<Output_expression_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Output_expression_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_output_expression_list);
        let mut _localctx: Rc<Output_expression_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(983);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 18)) & !0x3f) == 0 && ((1usize << (_la - 18)) & ((1usize << (T__17 - 18)) | (1usize << (T__21 - 18)) | (1usize << (T__24 - 18)) | (1usize << (T__45 - 18)) | (1usize << (T__48 - 18)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__72 - 66)) | (1usize << (T__73 - 66)) | (1usize << (T__74 - 66)) | (1usize << (T__75 - 66)) | (1usize << (T__81 - 66)) | (1usize << (T__82 - 66)) | (1usize << (T__83 - 66)) | (1usize << (T__85 - 66)) | (1usize << (T__86 - 66)) | (1usize << (IDENT - 66)) | (1usize << (STRING - 66)) | (1usize << (UNSIGNED_NUMBER - 66)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(982);
				recog.expression()?;

				}
			}

			recog.base.set_state(991);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(985);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				recog.base.set_state(987);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 18)) & !0x3f) == 0 && ((1usize << (_la - 18)) & ((1usize << (T__17 - 18)) | (1usize << (T__21 - 18)) | (1usize << (T__24 - 18)) | (1usize << (T__45 - 18)) | (1usize << (T__48 - 18)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__72 - 66)) | (1usize << (T__73 - 66)) | (1usize << (T__74 - 66)) | (1usize << (T__75 - 66)) | (1usize << (T__81 - 66)) | (1usize << (T__82 - 66)) | (1usize << (T__83 - 66)) | (1usize << (T__85 - 66)) | (1usize << (T__86 - 66)) | (1usize << (IDENT - 66)) | (1usize << (STRING - 66)) | (1usize << (UNSIGNED_NUMBER - 66)))) != 0) {
					{
					/*InvokeRule expression*/
					recog.base.set_state(986);
					recog.expression()?;

					}
				}

				}
				}
				recog.base.set_state(993);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression_list ----------------
pub type Expression_listContextAll<'input> = Expression_listContext<'input>;


pub type Expression_listContext<'input> = BaseParserRuleContext<'input,Expression_listContextExt<'input>>;

#[derive(Clone)]
pub struct Expression_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Expression_listContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Expression_listContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression_list(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_expression_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expression_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression_list }
}
antlr_rust::tid!{Expression_listContextExt<'a>}

impl<'input> Expression_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expression_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expression_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Expression_listContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Expression_listContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Expression_listContextAttrs<'input> for Expression_listContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression_list(&mut self,)
	-> Result<Rc<Expression_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expression_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_expression_list);
        let mut _localctx: Rc<Expression_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(994);
			recog.expression()?;

			recog.base.set_state(999);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(995);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(996);
				recog.expression()?;

				}
				}
				recog.base.set_state(1001);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array_subscripts ----------------
pub type Array_subscriptsContextAll<'input> = Array_subscriptsContext<'input>;


pub type Array_subscriptsContext<'input> = BaseParserRuleContext<'input,Array_subscriptsContextExt<'input>>;

#[derive(Clone)]
pub struct Array_subscriptsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Array_subscriptsContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Array_subscriptsContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_array_subscripts(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_array_subscripts(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Array_subscriptsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array_subscripts }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array_subscripts }
}
antlr_rust::tid!{Array_subscriptsContextExt<'a>}

impl<'input> Array_subscriptsContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Array_subscriptsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Array_subscriptsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Array_subscriptsContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Array_subscriptsContextExt<'input>>{

fn subscript__all(&self) ->  Vec<Rc<Subscript_ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn subscript_(&self, i: usize) -> Option<Rc<Subscript_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Array_subscriptsContextAttrs<'input> for Array_subscriptsContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array_subscripts(&mut self,)
	-> Result<Rc<Array_subscriptsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Array_subscriptsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_array_subscripts);
        let mut _localctx: Rc<Array_subscriptsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1002);
			recog.base.match_token(T__83,&mut recog.err_handler)?;

			/*InvokeRule subscript_*/
			recog.base.set_state(1003);
			recog.subscript_()?;

			recog.base.set_state(1008);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(1004);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule subscript_*/
				recog.base.set_state(1005);
				recog.subscript_()?;

				}
				}
				recog.base.set_state(1010);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1011);
			recog.base.match_token(T__84,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subscript_ ----------------
pub type Subscript_ContextAll<'input> = Subscript_Context<'input>;


pub type Subscript_Context<'input> = BaseParserRuleContext<'input,Subscript_ContextExt<'input>>;

#[derive(Clone)]
pub struct Subscript_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for Subscript_Context<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for Subscript_Context<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_subscript_(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_subscript_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Subscript_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subscript_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subscript_ }
}
antlr_rust::tid!{Subscript_ContextExt<'a>}

impl<'input> Subscript_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Subscript_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Subscript_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Subscript_ContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<Subscript_ContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Subscript_ContextAttrs<'input> for Subscript_Context<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subscript_(&mut self,)
	-> Result<Rc<Subscript_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Subscript_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_subscript_);
        let mut _localctx: Rc<Subscript_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1015);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__22 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1013);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					}
				}

			 T__17 | T__21 | T__24 | T__45 | T__48 | T__65 | T__72 | T__73 | T__74 |
			 T__75 | T__81 | T__82 | T__83 | T__85 | T__86 | IDENT | STRING | UNSIGNED_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1014);
					recog.expression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comment ----------------
pub type CommentContextAll<'input> = CommentContext<'input>;


pub type CommentContext<'input> = BaseParserRuleContext<'input,CommentContextExt<'input>>;

#[derive(Clone)]
pub struct CommentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for CommentContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for CommentContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comment(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_comment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CommentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comment }
}
antlr_rust::tid!{CommentContextExt<'a>}

impl<'input> CommentContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommentContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<CommentContextExt<'input>>{

fn string_comment(&self) -> Option<Rc<String_commentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CommentContextAttrs<'input> for CommentContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comment(&mut self,)
	-> Result<Rc<CommentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_comment);
        let mut _localctx: Rc<CommentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule string_comment*/
			recog.base.set_state(1017);
			recog.string_comment()?;

			recog.base.set_state(1019);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__87 {
				{
				/*InvokeRule annotation*/
				recog.base.set_state(1018);
				recog.annotation()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- string_comment ----------------
pub type String_commentContextAll<'input> = String_commentContext<'input>;


pub type String_commentContext<'input> = BaseParserRuleContext<'input,String_commentContextExt<'input>>;

#[derive(Clone)]
pub struct String_commentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for String_commentContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for String_commentContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_string_comment(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_string_comment(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for String_commentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_string_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_string_comment }
}
antlr_rust::tid!{String_commentContextExt<'a>}

impl<'input> String_commentContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<String_commentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,String_commentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait String_commentContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<String_commentContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,modelicaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,modelicaParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}

}

impl<'input> String_commentContextAttrs<'input> for String_commentContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn string_comment(&mut self,)
	-> Result<Rc<String_commentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = String_commentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_string_comment);
        let mut _localctx: Rc<String_commentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1029);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STRING {
				{
				recog.base.set_state(1021);
				recog.base.match_token(STRING,&mut recog.err_handler)?;

				recog.base.set_state(1026);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__72 {
					{
					{
					recog.base.set_state(1022);
					recog.base.match_token(T__72,&mut recog.err_handler)?;

					recog.base.set_state(1023);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(1028);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;


pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> modelicaParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn modelicaListener<'input> + 'a> for AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotation(self);
		}fn exit(&self,listener: &mut (dyn modelicaListener<'input> + 'a)) {
			listener.exit_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = modelicaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid!{AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn modelicaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationContextAttrs<'input>: modelicaParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{

fn class_modification(&self) -> Option<Rc<Class_modificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

impl<'input, I, H> modelicaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotation(&mut self,)
	-> Result<Rc<AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1031);
			recog.base.match_token(T__87,&mut recog.err_handler)?;

			/*InvokeRule class_modification*/
			recog.base.set_state(1032);
			recog.class_modification()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x60\u{40d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x03\x02\x03\x02\x05\x02\u{a1}\x0a\x02\
	\x03\x02\x07\x02\u{a4}\x0a\x02\x0c\x02\x0e\x02\u{a7}\x0b\x02\x03\x02\x05\
	\x02\u{aa}\x0a\x02\x03\x02\x03\x02\x03\x02\x07\x02\u{af}\x0a\x02\x0c\x02\
	\x0e\x02\u{b2}\x0b\x02\x03\x02\x03\x02\x03\x03\x05\x03\u{b7}\x0a\x03\x03\
	\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x05\x04\u{bf}\x0a\x04\x03\x05\
	\x05\x05\u{c2}\x0a\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{c7}\x0a\x05\x03\
	\x05\x03\x05\x03\x05\x05\x05\u{cc}\x0a\x05\x03\x05\x03\x05\x03\x05\x03\x05\
	\x05\x05\u{d2}\x0a\x05\x03\x05\x05\x05\u{d5}\x0a\x05\x03\x05\x03\x05\x05\
	\x05\u{d9}\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x05\x06\u{e4}\x0a\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\
	\x06\x05\x06\u{eb}\x0a\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\
	\u{f2}\x0a\x07\x03\x07\x05\x07\u{f5}\x0a\x07\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{fe}\x0a\x07\x03\x07\x05\x07\u{101}\
	\x0a\x07\x03\x07\x03\x07\x05\x07\u{105}\x0a\x07\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x07\x08\u{110}\x0a\x08\
	\x0c\x08\x0e\x08\u{113}\x0b\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\
	\x03\x0a\x03\x0a\x03\x0a\x07\x0a\u{11d}\x0a\x0a\x0c\x0a\x0e\x0a\u{120}\x0b\
	\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x07\x0c\u{12c}\x0a\x0c\x0c\x0c\x0e\x0c\u{12f}\x0b\x0c\x03\x0c\
	\x03\x0c\x05\x0c\u{133}\x0a\x0c\x03\x0c\x05\x0c\u{136}\x0a\x0c\x03\x0c\x05\
	\x0c\u{139}\x0a\x0c\x03\x0c\x05\x0c\u{13c}\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x05\x0c\u{141}\x0a\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x05\x0e\
	\u{148}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{14d}\x0a\x0e\x03\x0e\x03\
	\x0e\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{154}\x0a\x0f\x0c\x0f\x0e\x0f\u{157}\
	\x0b\x0f\x03\x10\x03\x10\x03\x10\x05\x10\u{15c}\x0a\x10\x03\x10\x05\x10\
	\u{15f}\x0a\x10\x03\x10\x05\x10\u{162}\x0a\x10\x03\x10\x05\x10\u{165}\x0a\
	\x10\x03\x10\x03\x10\x05\x10\u{169}\x0a\x10\x03\x10\x03\x10\x03\x10\x05\
	\x10\u{16e}\x0a\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{173}\x0a\x10\x05\x10\
	\u{175}\x0a\x10\x05\x10\u{177}\x0a\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x05\
	\x11\u{186}\x0a\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x07\x12\u{18d}\
	\x0a\x12\x0c\x12\x0e\x12\u{190}\x0b\x12\x03\x13\x03\x13\x03\x13\x05\x13\
	\u{195}\x0a\x13\x03\x13\x05\x13\u{198}\x0a\x13\x03\x14\x03\x14\x03\x14\x05\
	\x14\u{19d}\x0a\x14\x03\x15\x03\x15\x03\x15\x05\x15\u{1a2}\x0a\x15\x03\x15\
	\x03\x15\x03\x16\x05\x16\u{1a7}\x0a\x16\x03\x16\x05\x16\u{1aa}\x0a\x16\x03\
	\x16\x05\x16\u{1ad}\x0a\x16\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x07\
	\x18\u{1b4}\x0a\x18\x0c\x18\x0e\x18\u{1b7}\x0b\x18\x03\x19\x03\x19\x05\x19\
	\u{1bb}\x0a\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\
	\x05\x1b\u{1c4}\x0a\x1b\x03\x1b\x05\x1b\u{1c7}\x0a\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x05\x1c\u{1cc}\x0a\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x05\x1c\u{1d2}\
	\x0a\x1c\x03\x1d\x03\x1d\x05\x1d\u{1d6}\x0a\x1d\x03\x1d\x03\x1d\x03\x1e\
	\x03\x1e\x03\x1e\x07\x1e\u{1dd}\x0a\x1e\x0c\x1e\x0e\x1e\u{1e0}\x0b\x1e\x03\
	\x1f\x03\x1f\x05\x1f\u{1e4}\x0a\x1f\x03\x20\x05\x20\u{1e7}\x0a\x20\x03\x20\
	\x05\x20\u{1ea}\x0a\x20\x03\x20\x03\x20\x05\x20\u{1ee}\x0a\x20\x03\x21\x03\
	\x21\x05\x21\u{1f2}\x0a\x21\x03\x21\x03\x21\x03\x22\x03\x22\x05\x22\u{1f8}\
	\x0a\x22\x03\x22\x05\x22\u{1fb}\x0a\x22\x03\x22\x03\x22\x05\x22\u{1ff}\x0a\
	\x22\x03\x22\x05\x22\u{202}\x0a\x22\x03\x23\x03\x23\x03\x23\x05\x23\u{207}\
	\x0a\x23\x03\x23\x05\x23\u{20a}\x0a\x23\x03\x24\x03\x24\x03\x24\x03\x24\
	\x03\x25\x03\x25\x03\x25\x03\x26\x03\x26\x03\x26\x03\x27\x05\x27\u{217}\
	\x0a\x27\x03\x27\x03\x27\x03\x27\x03\x27\x07\x27\u{21d}\x0a\x27\x0c\x27\
	\x0e\x27\u{220}\x0b\x27\x03\x28\x05\x28\u{223}\x0a\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x07\x28\u{229}\x0a\x28\x0c\x28\x0e\x28\u{22c}\x0b\x28\x03\x29\
	\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\x03\x29\
	\x03\x29\x05\x29\u{239}\x0a\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x05\x2a\u{241}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\
	\u{250}\x0a\x2a\x03\x2a\x03\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\
	\x03\x2b\x07\x2b\u{25a}\x0a\x2b\x0c\x2b\x0e\x2b\u{25d}\x0b\x2b\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x07\x2b\u{265}\x0a\x2b\x0c\x2b\x0e\
	\x2b\u{268}\x0b\x2b\x07\x2b\u{26a}\x0a\x2b\x0c\x2b\x0e\x2b\u{26d}\x0b\x2b\
	\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x07\x2b\u{273}\x0a\x2b\x0c\x2b\x0e\x2b\
	\u{276}\x0b\x2b\x05\x2b\u{278}\x0a\x2b\x03\x2b\x03\x2b\x03\x2b\x03\x2c\x03\
	\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x07\x2c\u{283}\x0a\x2c\x0c\x2c\x0e\
	\x2c\u{286}\x0b\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x07\
	\x2c\u{28e}\x0a\x2c\x0c\x2c\x0e\x2c\u{291}\x0b\x2c\x07\x2c\u{293}\x0a\x2c\
	\x0c\x2c\x0e\x2c\u{296}\x0b\x2c\x03\x2c\x03\x2c\x03\x2c\x03\x2c\x07\x2c\
	\u{29c}\x0a\x2c\x0c\x2c\x0e\x2c\u{29f}\x0b\x2c\x05\x2c\u{2a1}\x0a\x2c\x03\
	\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x03\x2d\x07\
	\x2d\u{2ac}\x0a\x2d\x0c\x2d\x0e\x2d\u{2af}\x0b\x2d\x03\x2d\x03\x2d\x03\x2d\
	\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x07\x2e\u{2ba}\x0a\x2e\
	\x0c\x2e\x0e\x2e\u{2bd}\x0b\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2f\x03\x2f\
	\x03\x2f\x07\x2f\u{2c5}\x0a\x2f\x0c\x2f\x0e\x2f\u{2c8}\x0b\x2f\x03\x30\x03\
	\x30\x03\x30\x05\x30\u{2cd}\x0a\x30\x03\x31\x03\x31\x03\x31\x03\x31\x03\
	\x31\x03\x31\x07\x31\u{2d5}\x0a\x31\x0c\x31\x0e\x31\u{2d8}\x0b\x31\x03\x31\
	\x03\x31\x03\x31\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x07\x32\
	\u{2e3}\x0a\x32\x0c\x32\x0e\x32\u{2e6}\x0b\x32\x03\x32\x03\x32\x03\x32\x03\
	\x32\x03\x32\x03\x32\x07\x32\u{2ee}\x0a\x32\x0c\x32\x0e\x32\u{2f1}\x0b\x32\
	\x07\x32\u{2f3}\x0a\x32\x0c\x32\x0e\x32\u{2f6}\x0b\x32\x03\x32\x03\x32\x03\
	\x32\x03\x33\x03\x33\x03\x33\x03\x33\x03\x33\x03\x33\x07\x33\u{301}\x0a\
	\x33\x0c\x33\x0e\x33\u{304}\x0b\x33\x03\x33\x03\x33\x03\x33\x03\x33\x03\
	\x33\x03\x33\x07\x33\u{30c}\x0a\x33\x0c\x33\x0e\x33\u{30f}\x0b\x33\x07\x33\
	\u{311}\x0a\x33\x0c\x33\x0e\x33\u{314}\x0b\x33\x03\x33\x03\x33\x03\x33\x03\
	\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x35\x03\x35\x03\
	\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x07\x35\u{32a}\
	\x0a\x35\x0c\x35\x0e\x35\u{32d}\x0b\x35\x03\x35\x03\x35\x03\x35\x05\x35\
	\u{332}\x0a\x35\x03\x36\x03\x36\x03\x36\x03\x36\x03\x36\x05\x36\u{339}\x0a\
	\x36\x05\x36\u{33b}\x0a\x36\x03\x37\x03\x37\x03\x37\x07\x37\u{340}\x0a\x37\
	\x0c\x37\x0e\x37\u{343}\x0b\x37\x03\x38\x03\x38\x03\x38\x07\x38\u{348}\x0a\
	\x38\x0c\x38\x0e\x38\u{34b}\x0b\x38\x03\x39\x05\x39\u{34e}\x0a\x39\x03\x39\
	\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x05\x3a\u{356}\x0a\x3a\x03\x3b\
	\x03\x3b\x03\x3c\x05\x3c\u{35b}\x0a\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\
	\x07\x3c\u{361}\x0a\x3c\x0c\x3c\x0e\x3c\u{364}\x0b\x3c\x03\x3d\x03\x3d\x03\
	\x3e\x03\x3e\x03\x3e\x03\x3e\x07\x3e\u{36c}\x0a\x3e\x0c\x3e\x0e\x3e\u{36f}\
	\x0b\x3e\x03\x3f\x03\x3f\x03\x40\x03\x40\x03\x40\x05\x40\u{376}\x0a\x40\
	\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x05\x41\u{37f}\
	\x0a\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\
	\x03\x41\x03\x41\x07\x41\u{38b}\x0a\x41\x0c\x41\x0e\x41\u{38e}\x0b\x41\x03\
	\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x03\x41\x05\x41\u{397}\x0a\
	\x41\x03\x42\x05\x42\u{39a}\x0a\x42\x03\x42\x03\x42\x03\x42\x07\x42\u{39f}\
	\x0a\x42\x0c\x42\x0e\x42\u{3a2}\x0b\x42\x03\x43\x05\x43\u{3a5}\x0a\x43\x03\
	\x43\x03\x43\x05\x43\u{3a9}\x0a\x43\x03\x43\x03\x43\x03\x43\x05\x43\u{3ae}\
	\x0a\x43\x07\x43\u{3b0}\x0a\x43\x0c\x43\x0e\x43\u{3b3}\x0b\x43\x03\x44\x03\
	\x44\x05\x44\u{3b7}\x0a\x44\x03\x44\x03\x44\x03\x45\x03\x45\x03\x45\x03\
	\x45\x03\x45\x05\x45\u{3c0}\x0a\x45\x03\x45\x05\x45\u{3c3}\x0a\x45\x03\x46\
	\x03\x46\x03\x46\x05\x46\u{3c8}\x0a\x46\x03\x47\x03\x47\x03\x47\x03\x47\
	\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\u{3d2}\x0a\x48\x03\x48\x03\x48\
	\x03\x48\x05\x48\u{3d7}\x0a\x48\x03\x49\x05\x49\u{3da}\x0a\x49\x03\x49\x03\
	\x49\x05\x49\u{3de}\x0a\x49\x07\x49\u{3e0}\x0a\x49\x0c\x49\x0e\x49\u{3e3}\
	\x0b\x49\x03\x4a\x03\x4a\x03\x4a\x07\x4a\u{3e8}\x0a\x4a\x0c\x4a\x0e\x4a\
	\u{3eb}\x0b\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x07\x4b\u{3f1}\x0a\x4b\x0c\
	\x4b\x0e\x4b\u{3f4}\x0b\x4b\x03\x4b\x03\x4b\x03\x4c\x03\x4c\x05\x4c\u{3fa}\
	\x0a\x4c\x03\x4d\x03\x4d\x05\x4d\u{3fe}\x0a\x4d\x03\x4e\x03\x4e\x03\x4e\
	\x07\x4e\u{403}\x0a\x4e\x0c\x4e\x0e\x4e\u{406}\x0b\x4e\x05\x4e\u{408}\x0a\
	\x4e\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x02\x02\x50\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\
	\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\
	\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\
	\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\
	\u{94}\u{96}\u{98}\u{9a}\u{9c}\x02\x0a\x03\x02\x11\x12\x03\x02\x29\x2a\x03\
	\x02\x2b\x2d\x03\x02\x2e\x2f\x03\x02\x45\x4a\x03\x02\x4b\x4e\x04\x02\x25\
	\x25\x4f\x51\x03\x02\x52\x53\x02\u{464}\x02\u{a5}\x03\x02\x02\x02\x04\u{b6}\
	\x03\x02\x02\x02\x06\u{be}\x03\x02\x02\x02\x08\u{c1}\x03\x02\x02\x02\x0a\
	\u{ea}\x03\x02\x02\x02\x0c\u{104}\x03\x02\x02\x02\x0e\u{106}\x03\x02\x02\
	\x02\x10\u{117}\x03\x02\x02\x02\x12\u{119}\x03\x02\x02\x02\x14\u{121}\x03\
	\x02\x02\x02\x16\u{124}\x03\x02\x02\x02\x18\u{142}\x03\x02\x02\x02\x1a\u{147}\
	\x03\x02\x02\x02\x1c\u{155}\x03\x02\x02\x02\x1e\u{176}\x03\x02\x02\x02\x20\
	\u{178}\x03\x02\x02\x02\x22\u{189}\x03\x02\x02\x02\x24\u{191}\x03\x02\x02\
	\x02\x26\u{199}\x03\x02\x02\x02\x28\u{19e}\x03\x02\x02\x02\x2a\u{1a6}\x03\
	\x02\x02\x02\x2c\u{1ae}\x03\x02\x02\x02\x2e\u{1b0}\x03\x02\x02\x02\x30\u{1b8}\
	\x03\x02\x02\x02\x32\u{1be}\x03\x02\x02\x02\x34\u{1c1}\x03\x02\x02\x02\x36\
	\u{1d1}\x03\x02\x02\x02\x38\u{1d3}\x03\x02\x02\x02\x3a\u{1d9}\x03\x02\x02\
	\x02\x3c\u{1e3}\x03\x02\x02\x02\x3e\u{1e6}\x03\x02\x02\x02\x40\u{1ef}\x03\
	\x02\x02\x02\x42\u{1f5}\x03\x02\x02\x02\x44\u{203}\x03\x02\x02\x02\x46\u{20b}\
	\x03\x02\x02\x02\x48\u{20f}\x03\x02\x02\x02\x4a\u{212}\x03\x02\x02\x02\x4c\
	\u{216}\x03\x02\x02\x02\x4e\u{222}\x03\x02\x02\x02\x50\u{238}\x03\x02\x02\
	\x02\x52\u{24f}\x03\x02\x02\x02\x54\u{253}\x03\x02\x02\x02\x56\u{27c}\x03\
	\x02\x02\x02\x58\u{2a5}\x03\x02\x02\x02\x5a\u{2b3}\x03\x02\x02\x02\x5c\u{2c1}\
	\x03\x02\x02\x02\x5e\u{2c9}\x03\x02\x02\x02\x60\u{2ce}\x03\x02\x02\x02\x62\
	\u{2dc}\x03\x02\x02\x02\x64\u{2fa}\x03\x02\x02\x02\x66\u{318}\x03\x02\x02\
	\x02\x68\u{331}\x03\x02\x02\x02\x6a\u{333}\x03\x02\x02\x02\x6c\u{33c}\x03\
	\x02\x02\x02\x6e\u{344}\x03\x02\x02\x02\x70\u{34d}\x03\x02\x02\x02\x72\u{351}\
	\x03\x02\x02\x02\x74\u{357}\x03\x02\x02\x02\x76\u{35a}\x03\x02\x02\x02\x78\
	\u{365}\x03\x02\x02\x02\x7a\u{367}\x03\x02\x02\x02\x7c\u{370}\x03\x02\x02\
	\x02\x7e\u{372}\x03\x02\x02\x02\u{80}\u{396}\x03\x02\x02\x02\u{82}\u{399}\
	\x03\x02\x02\x02\u{84}\u{3a4}\x03\x02\x02\x02\u{86}\u{3b4}\x03\x02\x02\x02\
	\u{88}\u{3c2}\x03\x02\x02\x02\u{8a}\u{3c4}\x03\x02\x02\x02\u{8c}\u{3c9}\
	\x03\x02\x02\x02\u{8e}\u{3d6}\x03\x02\x02\x02\u{90}\u{3d9}\x03\x02\x02\x02\
	\u{92}\u{3e4}\x03\x02\x02\x02\u{94}\u{3ec}\x03\x02\x02\x02\u{96}\u{3f9}\
	\x03\x02\x02\x02\u{98}\u{3fb}\x03\x02\x02\x02\u{9a}\u{407}\x03\x02\x02\x02\
	\u{9c}\u{409}\x03\x02\x02\x02\u{9e}\u{a0}\x07\x03\x02\x02\u{9f}\u{a1}\x05\
	\u{82}\x42\x02\u{a0}\u{9f}\x03\x02\x02\x02\u{a0}\u{a1}\x03\x02\x02\x02\u{a1}\
	\u{a2}\x03\x02\x02\x02\u{a2}\u{a4}\x07\x04\x02\x02\u{a3}\u{9e}\x03\x02\x02\
	\x02\u{a4}\u{a7}\x03\x02\x02\x02\u{a5}\u{a3}\x03\x02\x02\x02\u{a5}\u{a6}\
	\x03\x02\x02\x02\u{a6}\u{b0}\x03\x02\x02\x02\u{a7}\u{a5}\x03\x02\x02\x02\
	\u{a8}\u{aa}\x07\x05\x02\x02\u{a9}\u{a8}\x03\x02\x02\x02\u{a9}\u{aa}\x03\
	\x02\x02\x02\u{aa}\u{ab}\x03\x02\x02\x02\u{ab}\u{ac}\x05\x04\x03\x02\u{ac}\
	\u{ad}\x07\x04\x02\x02\u{ad}\u{af}\x03\x02\x02\x02\u{ae}\u{a9}\x03\x02\x02\
	\x02\u{af}\u{b2}\x03\x02\x02\x02\u{b0}\u{ae}\x03\x02\x02\x02\u{b0}\u{b1}\
	\x03\x02\x02\x02\u{b1}\u{b3}\x03\x02\x02\x02\u{b2}\u{b0}\x03\x02\x02\x02\
	\u{b3}\u{b4}\x07\x02\x02\x03\u{b4}\x03\x03\x02\x02\x02\u{b5}\u{b7}\x07\x06\
	\x02\x02\u{b6}\u{b5}\x03\x02\x02\x02\u{b6}\u{b7}\x03\x02\x02\x02\u{b7}\u{b8}\
	\x03\x02\x02\x02\u{b8}\u{b9}\x05\x08\x05\x02\u{b9}\u{ba}\x05\x06\x04\x02\
	\u{ba}\x05\x03\x02\x02\x02\u{bb}\u{bf}\x05\x0a\x06\x02\u{bc}\u{bf}\x05\x0c\
	\x07\x02\u{bd}\u{bf}\x05\x0e\x08\x02\u{be}\u{bb}\x03\x02\x02\x02\u{be}\u{bc}\
	\x03\x02\x02\x02\u{be}\u{bd}\x03\x02\x02\x02\u{bf}\x07\x03\x02\x02\x02\u{c0}\
	\u{c2}\x07\x07\x02\x02\u{c1}\u{c0}\x03\x02\x02\x02\u{c1}\u{c2}\x03\x02\x02\
	\x02\u{c2}\u{d8}\x03\x02\x02\x02\u{c3}\u{d9}\x07\x08\x02\x02\u{c4}\u{d9}\
	\x07\x09\x02\x02\u{c5}\u{c7}\x07\x0a\x02\x02\u{c6}\u{c5}\x03\x02\x02\x02\
	\u{c6}\u{c7}\x03\x02\x02\x02\u{c7}\u{c8}\x03\x02\x02\x02\u{c8}\u{d9}\x07\
	\x0b\x02\x02\u{c9}\u{d9}\x07\x0c\x02\x02\u{ca}\u{cc}\x07\x0d\x02\x02\u{cb}\
	\u{ca}\x03\x02\x02\x02\u{cb}\u{cc}\x03\x02\x02\x02\u{cc}\u{cd}\x03\x02\x02\
	\x02\u{cd}\u{d9}\x07\x0e\x02\x02\u{ce}\u{d9}\x07\x0f\x02\x02\u{cf}\u{d9}\
	\x07\x10\x02\x02\u{d0}\u{d2}\x09\x02\x02\x02\u{d1}\u{d0}\x03\x02\x02\x02\
	\u{d1}\u{d2}\x03\x02\x02\x02\u{d2}\u{d4}\x03\x02\x02\x02\u{d3}\u{d5}\x07\
	\x0a\x02\x02\u{d4}\u{d3}\x03\x02\x02\x02\u{d4}\u{d5}\x03\x02\x02\x02\u{d5}\
	\u{d6}\x03\x02\x02\x02\u{d6}\u{d9}\x07\x13\x02\x02\u{d7}\u{d9}\x07\x0a\x02\
	\x02\u{d8}\u{c3}\x03\x02\x02\x02\u{d8}\u{c4}\x03\x02\x02\x02\u{d8}\u{c6}\
	\x03\x02\x02\x02\u{d8}\u{c9}\x03\x02\x02\x02\u{d8}\u{cb}\x03\x02\x02\x02\
	\u{d8}\u{ce}\x03\x02\x02\x02\u{d8}\u{cf}\x03\x02\x02\x02\u{d8}\u{d1}\x03\
	\x02\x02\x02\u{d8}\u{d7}\x03\x02\x02\x02\u{d9}\x09\x03\x02\x02\x02\u{da}\
	\u{db}\x07\x5b\x02\x02\u{db}\u{dc}\x05\u{9a}\x4e\x02\u{dc}\u{dd}\x05\x16\
	\x0c\x02\u{dd}\u{de}\x07\x14\x02\x02\u{de}\u{df}\x07\x5b\x02\x02\u{df}\u{eb}\
	\x03\x02\x02\x02\u{e0}\u{e1}\x07\x15\x02\x02\u{e1}\u{e3}\x07\x5b\x02\x02\
	\u{e2}\u{e4}\x05\x38\x1d\x02\u{e3}\u{e2}\x03\x02\x02\x02\u{e3}\u{e4}\x03\
	\x02\x02\x02\u{e4}\u{e5}\x03\x02\x02\x02\u{e5}\u{e6}\x05\u{9a}\x4e\x02\u{e6}\
	\u{e7}\x05\x16\x0c\x02\u{e7}\u{e8}\x07\x14\x02\x02\u{e8}\u{e9}\x07\x5b\x02\
	\x02\u{e9}\u{eb}\x03\x02\x02\x02\u{ea}\u{da}\x03\x02\x02\x02\u{ea}\u{e0}\
	\x03\x02\x02\x02\u{eb}\x0b\x03\x02\x02\x02\u{ec}\u{ed}\x07\x5b\x02\x02\u{ed}\
	\u{ee}\x07\x16\x02\x02\u{ee}\u{ef}\x05\x10\x09\x02\u{ef}\u{f1}\x05\u{82}\
	\x42\x02\u{f0}\u{f2}\x05\u{94}\x4b\x02\u{f1}\u{f0}\x03\x02\x02\x02\u{f1}\
	\u{f2}\x03\x02\x02\x02\u{f2}\u{f4}\x03\x02\x02\x02\u{f3}\u{f5}\x05\x38\x1d\
	\x02\u{f4}\u{f3}\x03\x02\x02\x02\u{f4}\u{f5}\x03\x02\x02\x02\u{f5}\u{f6}\
	\x03\x02\x02\x02\u{f6}\u{f7}\x05\u{98}\x4d\x02\u{f7}\u{105}\x03\x02\x02\
	\x02\u{f8}\u{f9}\x07\x5b\x02\x02\u{f9}\u{fa}\x07\x16\x02\x02\u{fa}\u{fb}\
	\x07\x17\x02\x02\u{fb}\u{100}\x07\x18\x02\x02\u{fc}\u{fe}\x05\x12\x0a\x02\
	\u{fd}\u{fc}\x03\x02\x02\x02\u{fd}\u{fe}\x03\x02\x02\x02\u{fe}\u{101}\x03\
	\x02\x02\x02\u{ff}\u{101}\x07\x19\x02\x02\u{100}\u{fd}\x03\x02\x02\x02\u{100}\
	\u{ff}\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\u{103}\x07\x1a\
	\x02\x02\u{103}\u{105}\x05\u{98}\x4d\x02\u{104}\u{ec}\x03\x02\x02\x02\u{104}\
	\u{f8}\x03\x02\x02\x02\u{105}\x0d\x03\x02\x02\x02\u{106}\u{107}\x07\x5b\
	\x02\x02\u{107}\u{108}\x07\x16\x02\x02\u{108}\u{109}\x07\x1b\x02\x02\u{109}\
	\u{10a}\x07\x18\x02\x02\u{10a}\u{10b}\x05\u{82}\x42\x02\u{10b}\u{10c}\x07\
	\x1c\x02\x02\u{10c}\u{111}\x07\x5b\x02\x02\u{10d}\u{10e}\x07\x1c\x02\x02\
	\u{10e}\u{110}\x07\x5b\x02\x02\u{10f}\u{10d}\x03\x02\x02\x02\u{110}\u{113}\
	\x03\x02\x02\x02\u{111}\u{10f}\x03\x02\x02\x02\u{111}\u{112}\x03\x02\x02\
	\x02\u{112}\u{114}\x03\x02\x02\x02\u{113}\u{111}\x03\x02\x02\x02\u{114}\
	\u{115}\x07\x1a\x02\x02\u{115}\u{116}\x05\u{98}\x4d\x02\u{116}\x0f\x03\x02\
	\x02\x02\u{117}\u{118}\x05\x2a\x16\x02\u{118}\x11\x03\x02\x02\x02\u{119}\
	\u{11e}\x05\x14\x0b\x02\u{11a}\u{11b}\x07\x1c\x02\x02\u{11b}\u{11d}\x05\
	\x14\x0b\x02\u{11c}\u{11a}\x03\x02\x02\x02\u{11d}\u{120}\x03\x02\x02\x02\
	\u{11e}\u{11c}\x03\x02\x02\x02\u{11e}\u{11f}\x03\x02\x02\x02\u{11f}\x13\
	\x03\x02\x02\x02\u{120}\u{11e}\x03\x02\x02\x02\u{121}\u{122}\x07\x5b\x02\
	\x02\u{122}\u{123}\x05\u{98}\x4d\x02\u{123}\x15\x03\x02\x02\x02\u{124}\u{12d}\
	\x05\x1c\x0f\x02\u{125}\u{126}\x07\x1d\x02\x02\u{126}\u{12c}\x05\x1c\x0f\
	\x02\u{127}\u{128}\x07\x1e\x02\x02\u{128}\u{12c}\x05\x1c\x0f\x02\u{129}\
	\u{12c}\x05\x4c\x27\x02\u{12a}\u{12c}\x05\x4e\x28\x02\u{12b}\u{125}\x03\
	\x02\x02\x02\u{12b}\u{127}\x03\x02\x02\x02\u{12b}\u{129}\x03\x02\x02\x02\
	\u{12b}\u{12a}\x03\x02\x02\x02\u{12c}\u{12f}\x03\x02\x02\x02\u{12d}\u{12b}\
	\x03\x02\x02\x02\u{12d}\u{12e}\x03\x02\x02\x02\u{12e}\u{13b}\x03\x02\x02\
	\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{130}\u{132}\x07\x1f\x02\x02\u{131}\
	\u{133}\x05\x18\x0d\x02\u{132}\u{131}\x03\x02\x02\x02\u{132}\u{133}\x03\
	\x02\x02\x02\u{133}\u{135}\x03\x02\x02\x02\u{134}\u{136}\x05\x1a\x0e\x02\
	\u{135}\u{134}\x03\x02\x02\x02\u{135}\u{136}\x03\x02\x02\x02\u{136}\u{138}\
	\x03\x02\x02\x02\u{137}\u{139}\x05\u{9c}\x4f\x02\u{138}\u{137}\x03\x02\x02\
	\x02\u{138}\u{139}\x03\x02\x02\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\
	\u{13c}\x07\x04\x02\x02\u{13b}\u{130}\x03\x02\x02\x02\u{13b}\u{13c}\x03\
	\x02\x02\x02\u{13c}\u{140}\x03\x02\x02\x02\u{13d}\u{13e}\x05\u{9c}\x4f\x02\
	\u{13e}\u{13f}\x07\x04\x02\x02\u{13f}\u{141}\x03\x02\x02\x02\u{140}\u{13d}\
	\x03\x02\x02\x02\u{140}\u{141}\x03\x02\x02\x02\u{141}\x17\x03\x02\x02\x02\
	\u{142}\u{143}\x07\x5c\x02\x02\u{143}\x19\x03\x02\x02\x02\u{144}\u{145}\
	\x05\u{84}\x43\x02\u{145}\u{146}\x07\x16\x02\x02\u{146}\u{148}\x03\x02\x02\
	\x02\u{147}\u{144}\x03\x02\x02\x02\u{147}\u{148}\x03\x02\x02\x02\u{148}\
	\u{149}\x03\x02\x02\x02\u{149}\u{14a}\x07\x5b\x02\x02\u{14a}\u{14c}\x07\
	\x18\x02\x02\u{14b}\u{14d}\x05\u{92}\x4a\x02\u{14c}\u{14b}\x03\x02\x02\x02\
	\u{14c}\u{14d}\x03\x02\x02\x02\u{14d}\u{14e}\x03\x02\x02\x02\u{14e}\u{14f}\
	\x07\x1a\x02\x02\u{14f}\x1b\x03\x02\x02\x02\u{150}\u{151}\x05\x1e\x10\x02\
	\u{151}\u{152}\x07\x04\x02\x02\u{152}\u{154}\x03\x02\x02\x02\u{153}\u{150}\
	\x03\x02\x02\x02\u{154}\u{157}\x03\x02\x02\x02\u{155}\u{153}\x03\x02\x02\
	\x02\u{155}\u{156}\x03\x02\x02\x02\u{156}\x1d\x03\x02\x02\x02\u{157}\u{155}\
	\x03\x02\x02\x02\u{158}\u{177}\x05\x20\x11\x02\u{159}\u{177}\x05\x24\x13\
	\x02\u{15a}\u{15c}\x07\x20\x02\x02\u{15b}\u{15a}\x03\x02\x02\x02\u{15b}\
	\u{15c}\x03\x02\x02\x02\u{15c}\u{15e}\x03\x02\x02\x02\u{15d}\u{15f}\x07\
	\x05\x02\x02\u{15e}\u{15d}\x03\x02\x02\x02\u{15e}\u{15f}\x03\x02\x02\x02\
	\u{15f}\u{161}\x03\x02\x02\x02\u{160}\u{162}\x07\x21\x02\x02\u{161}\u{160}\
	\x03\x02\x02\x02\u{161}\u{162}\x03\x02\x02\x02\u{162}\u{164}\x03\x02\x02\
	\x02\u{163}\u{165}\x07\x22\x02\x02\u{164}\u{163}\x03\x02\x02\x02\u{164}\
	\u{165}\x03\x02\x02\x02\u{165}\u{174}\x03\x02\x02\x02\u{166}\u{169}\x05\
	\x04\x03\x02\u{167}\u{169}\x05\x28\x15\x02\u{168}\u{166}\x03\x02\x02\x02\
	\u{168}\u{167}\x03\x02\x02\x02\u{169}\u{175}\x03\x02\x02\x02\u{16a}\u{16d}\
	\x07\x23\x02\x02\u{16b}\u{16e}\x05\x04\x03\x02\u{16c}\u{16e}\x05\x28\x15\
	\x02\u{16d}\u{16b}\x03\x02\x02\x02\u{16d}\u{16c}\x03\x02\x02\x02\u{16e}\
	\u{172}\x03\x02\x02\x02\u{16f}\u{170}\x05\x26\x14\x02\u{170}\u{171}\x05\
	\u{98}\x4d\x02\u{171}\u{173}\x03\x02\x02\x02\u{172}\u{16f}\x03\x02\x02\x02\
	\u{172}\u{173}\x03\x02\x02\x02\u{173}\u{175}\x03\x02\x02\x02\u{174}\u{168}\
	\x03\x02\x02\x02\u{174}\u{16a}\x03\x02\x02\x02\u{175}\u{177}\x03\x02\x02\
	\x02\u{176}\u{158}\x03\x02\x02\x02\u{176}\u{159}\x03\x02\x02\x02\u{176}\
	\u{15b}\x03\x02\x02\x02\u{177}\x1f\x03\x02\x02\x02\u{178}\u{185}\x07\x24\
	\x02\x02\u{179}\u{17a}\x07\x5b\x02\x02\u{17a}\u{17b}\x07\x16\x02\x02\u{17b}\
	\u{186}\x05\u{82}\x42\x02\u{17c}\u{17d}\x05\u{82}\x42\x02\u{17d}\u{17e}\
	\x07\x25\x02\x02\u{17e}\u{186}\x03\x02\x02\x02\u{17f}\u{180}\x05\u{82}\x42\
	\x02\u{180}\u{181}\x07\x26\x02\x02\u{181}\u{182}\x05\x22\x12\x02\u{182}\
	\u{183}\x07\x27\x02\x02\u{183}\u{186}\x03\x02\x02\x02\u{184}\u{186}\x05\
	\u{82}\x42\x02\u{185}\u{179}\x03\x02\x02\x02\u{185}\u{17c}\x03\x02\x02\x02\
	\u{185}\u{17f}\x03\x02\x02\x02\u{185}\u{184}\x03\x02\x02\x02\u{186}\u{187}\
	\x03\x02\x02\x02\u{187}\u{188}\x05\u{98}\x4d\x02\u{188}\x21\x03\x02\x02\
	\x02\u{189}\u{18e}\x07\x5b\x02\x02\u{18a}\u{18b}\x07\x1c\x02\x02\u{18b}\
	\u{18d}\x07\x5b\x02\x02\u{18c}\u{18a}\x03\x02\x02\x02\u{18d}\u{190}\x03\
	\x02\x02\x02\u{18e}\u{18c}\x03\x02\x02\x02\u{18e}\u{18f}\x03\x02\x02\x02\
	\u{18f}\x23\x03\x02\x02\x02\u{190}\u{18e}\x03\x02\x02\x02\u{191}\u{192}\
	\x07\x15\x02\x02\u{192}\u{194}\x05\u{82}\x42\x02\u{193}\u{195}\x05\x38\x1d\
	\x02\u{194}\u{193}\x03\x02\x02\x02\u{194}\u{195}\x03\x02\x02\x02\u{195}\
	\u{197}\x03\x02\x02\x02\u{196}\u{198}\x05\u{9c}\x4f\x02\u{197}\u{196}\x03\
	\x02\x02\x02\u{197}\u{198}\x03\x02\x02\x02\u{198}\x25\x03\x02\x02\x02\u{199}\
	\u{19a}\x07\x28\x02\x02\u{19a}\u{19c}\x05\u{82}\x42\x02\u{19b}\u{19d}\x05\
	\x38\x1d\x02\u{19c}\u{19b}\x03\x02\x02\x02\u{19c}\u{19d}\x03\x02\x02\x02\
	\u{19d}\x27\x03\x02\x02\x02\u{19e}\u{19f}\x05\x2a\x16\x02\u{19f}\u{1a1}\
	\x05\x2c\x17\x02\u{1a0}\u{1a2}\x05\u{94}\x4b\x02\u{1a1}\u{1a0}\x03\x02\x02\
	\x02\u{1a1}\u{1a2}\x03\x02\x02\x02\u{1a2}\u{1a3}\x03\x02\x02\x02\u{1a3}\
	\u{1a4}\x05\x2e\x18\x02\u{1a4}\x29\x03\x02\x02\x02\u{1a5}\u{1a7}\x09\x03\
	\x02\x02\u{1a6}\u{1a5}\x03\x02\x02\x02\u{1a6}\u{1a7}\x03\x02\x02\x02\u{1a7}\
	\u{1a9}\x03\x02\x02\x02\u{1a8}\u{1aa}\x09\x04\x02\x02\u{1a9}\u{1a8}\x03\
	\x02\x02\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\u{1ac}\x03\x02\x02\x02\
	\u{1ab}\u{1ad}\x09\x05\x02\x02\u{1ac}\u{1ab}\x03\x02\x02\x02\u{1ac}\u{1ad}\
	\x03\x02\x02\x02\u{1ad}\x2b\x03\x02\x02\x02\u{1ae}\u{1af}\x05\u{82}\x42\
	\x02\u{1af}\x2d\x03\x02\x02\x02\u{1b0}\u{1b5}\x05\x30\x19\x02\u{1b1}\u{1b2}\
	\x07\x1c\x02\x02\u{1b2}\u{1b4}\x05\x30\x19\x02\u{1b3}\u{1b1}\x03\x02\x02\
	\x02\u{1b4}\u{1b7}\x03\x02\x02\x02\u{1b5}\u{1b3}\x03\x02\x02\x02\u{1b5}\
	\u{1b6}\x03\x02\x02\x02\u{1b6}\x2f\x03\x02\x02\x02\u{1b7}\u{1b5}\x03\x02\
	\x02\x02\u{1b8}\u{1ba}\x05\x34\x1b\x02\u{1b9}\u{1bb}\x05\x32\x1a\x02\u{1ba}\
	\u{1b9}\x03\x02\x02\x02\u{1ba}\u{1bb}\x03\x02\x02\x02\u{1bb}\u{1bc}\x03\
	\x02\x02\x02\u{1bc}\u{1bd}\x05\u{98}\x4d\x02\u{1bd}\x31\x03\x02\x02\x02\
	\u{1be}\u{1bf}\x07\x30\x02\x02\u{1bf}\u{1c0}\x05\x68\x35\x02\u{1c0}\x33\
	\x03\x02\x02\x02\u{1c1}\u{1c3}\x07\x5b\x02\x02\u{1c2}\u{1c4}\x05\u{94}\x4b\
	\x02\u{1c3}\u{1c2}\x03\x02\x02\x02\u{1c3}\u{1c4}\x03\x02\x02\x02\u{1c4}\
	\u{1c6}\x03\x02\x02\x02\u{1c5}\u{1c7}\x05\x36\x1c\x02\u{1c6}\u{1c5}\x03\
	\x02\x02\x02\u{1c6}\u{1c7}\x03\x02\x02\x02\u{1c7}\x35\x03\x02\x02\x02\u{1c8}\
	\u{1cb}\x05\x38\x1d\x02\u{1c9}\u{1ca}\x07\x16\x02\x02\u{1ca}\u{1cc}\x05\
	\x68\x35\x02\u{1cb}\u{1c9}\x03\x02\x02\x02\u{1cb}\u{1cc}\x03\x02\x02\x02\
	\u{1cc}\u{1d2}\x03\x02\x02\x02\u{1cd}\u{1ce}\x07\x16\x02\x02\u{1ce}\u{1d2}\
	\x05\x68\x35\x02\u{1cf}\u{1d0}\x07\x31\x02\x02\u{1d0}\u{1d2}\x05\x68\x35\
	\x02\u{1d1}\u{1c8}\x03\x02\x02\x02\u{1d1}\u{1cd}\x03\x02\x02\x02\u{1d1}\
	\u{1cf}\x03\x02\x02\x02\u{1d2}\x37\x03\x02\x02\x02\u{1d3}\u{1d5}\x07\x18\
	\x02\x02\u{1d4}\u{1d6}\x05\x3a\x1e\x02\u{1d5}\u{1d4}\x03\x02\x02\x02\u{1d5}\
	\u{1d6}\x03\x02\x02\x02\u{1d6}\u{1d7}\x03\x02\x02\x02\u{1d7}\u{1d8}\x07\
	\x1a\x02\x02\u{1d8}\x39\x03\x02\x02\x02\u{1d9}\u{1de}\x05\x3c\x1f\x02\u{1da}\
	\u{1db}\x07\x1c\x02\x02\u{1db}\u{1dd}\x05\x3c\x1f\x02\u{1dc}\u{1da}\x03\
	\x02\x02\x02\u{1dd}\u{1e0}\x03\x02\x02\x02\u{1de}\u{1dc}\x03\x02\x02\x02\
	\u{1de}\u{1df}\x03\x02\x02\x02\u{1df}\x3b\x03\x02\x02\x02\u{1e0}\u{1de}\
	\x03\x02\x02\x02\u{1e1}\u{1e4}\x05\x3e\x20\x02\u{1e2}\u{1e4}\x05\x42\x22\
	\x02\u{1e3}\u{1e1}\x03\x02\x02\x02\u{1e3}\u{1e2}\x03\x02\x02\x02\u{1e4}\
	\x3d\x03\x02\x02\x02\u{1e5}\u{1e7}\x07\x32\x02\x02\u{1e6}\u{1e5}\x03\x02\
	\x02\x02\u{1e6}\u{1e7}\x03\x02\x02\x02\u{1e7}\u{1e9}\x03\x02\x02\x02\u{1e8}\
	\u{1ea}\x07\x05\x02\x02\u{1e9}\u{1e8}\x03\x02\x02\x02\u{1e9}\u{1ea}\x03\
	\x02\x02\x02\u{1ea}\u{1ed}\x03\x02\x02\x02\u{1eb}\u{1ee}\x05\x40\x21\x02\
	\u{1ec}\u{1ee}\x05\x44\x23\x02\u{1ed}\u{1eb}\x03\x02\x02\x02\u{1ed}\u{1ec}\
	\x03\x02\x02\x02\u{1ee}\x3f\x03\x02\x02\x02\u{1ef}\u{1f1}\x05\u{82}\x42\
	\x02\u{1f0}\u{1f2}\x05\x36\x1c\x02\u{1f1}\u{1f0}\x03\x02\x02\x02\u{1f1}\
	\u{1f2}\x03\x02\x02\x02\u{1f2}\u{1f3}\x03\x02\x02\x02\u{1f3}\u{1f4}\x05\
	\u{9a}\x4e\x02\u{1f4}\x41\x03\x02\x02\x02\u{1f5}\u{1f7}\x07\x20\x02\x02\
	\u{1f6}\u{1f8}\x07\x32\x02\x02\u{1f7}\u{1f6}\x03\x02\x02\x02\u{1f7}\u{1f8}\
	\x03\x02\x02\x02\u{1f8}\u{1fa}\x03\x02\x02\x02\u{1f9}\u{1fb}\x07\x05\x02\
	\x02\u{1fa}\u{1f9}\x03\x02\x02\x02\u{1fa}\u{1fb}\x03\x02\x02\x02\u{1fb}\
	\u{201}\x03\x02\x02\x02\u{1fc}\u{1ff}\x05\x4a\x26\x02\u{1fd}\u{1ff}\x05\
	\x46\x24\x02\u{1fe}\u{1fc}\x03\x02\x02\x02\u{1fe}\u{1fd}\x03\x02\x02\x02\
	\u{1ff}\u{202}\x03\x02\x02\x02\u{200}\u{202}\x05\x44\x23\x02\u{201}\u{1fe}\
	\x03\x02\x02\x02\u{201}\u{200}\x03\x02\x02\x02\u{202}\x43\x03\x02\x02\x02\
	\u{203}\u{206}\x07\x23\x02\x02\u{204}\u{207}\x05\x4a\x26\x02\u{205}\u{207}\
	\x05\x46\x24\x02\u{206}\u{204}\x03\x02\x02\x02\u{206}\u{205}\x03\x02\x02\
	\x02\u{207}\u{209}\x03\x02\x02\x02\u{208}\u{20a}\x05\x26\x14\x02\u{209}\
	\u{208}\x03\x02\x02\x02\u{209}\u{20a}\x03\x02\x02\x02\u{20a}\x45\x03\x02\
	\x02\x02\u{20b}\u{20c}\x05\x2a\x16\x02\u{20c}\u{20d}\x05\x2c\x17\x02\u{20d}\
	\u{20e}\x05\x48\x25\x02\u{20e}\x47\x03\x02\x02\x02\u{20f}\u{210}\x05\x34\
	\x1b\x02\u{210}\u{211}\x05\u{98}\x4d\x02\u{211}\x49\x03\x02\x02\x02\u{212}\
	\u{213}\x05\x08\x05\x02\u{213}\u{214}\x05\x0c\x07\x02\u{214}\x4b\x03\x02\
	\x02\x02\u{215}\u{217}\x07\x33\x02\x02\u{216}\u{215}\x03\x02\x02\x02\u{216}\
	\u{217}\x03\x02\x02\x02\u{217}\u{218}\x03\x02\x02\x02\u{218}\u{21e}\x07\
	\x34\x02\x02\u{219}\u{21a}\x05\x50\x29\x02\u{21a}\u{21b}\x07\x04\x02\x02\
	\u{21b}\u{21d}\x03\x02\x02\x02\u{21c}\u{219}\x03\x02\x02\x02\u{21d}\u{220}\
	\x03\x02\x02\x02\u{21e}\u{21c}\x03\x02\x02\x02\u{21e}\u{21f}\x03\x02\x02\
	\x02\u{21f}\x4d\x03\x02\x02\x02\u{220}\u{21e}\x03\x02\x02\x02\u{221}\u{223}\
	\x07\x33\x02\x02\u{222}\u{221}\x03\x02\x02\x02\u{222}\u{223}\x03\x02\x02\
	\x02\u{223}\u{224}\x03\x02\x02\x02\u{224}\u{22a}\x07\x35\x02\x02\u{225}\
	\u{226}\x05\x52\x2a\x02\u{226}\u{227}\x07\x04\x02\x02\u{227}\u{229}\x03\
	\x02\x02\x02\u{228}\u{225}\x03\x02\x02\x02\u{229}\u{22c}\x03\x02\x02\x02\
	\u{22a}\u{228}\x03\x02\x02\x02\u{22a}\u{22b}\x03\x02\x02\x02\u{22b}\x4f\
	\x03\x02\x02\x02\u{22c}\u{22a}\x03\x02\x02\x02\u{22d}\u{22e}\x05\x6a\x36\
	\x02\u{22e}\u{22f}\x07\x16\x02\x02\u{22f}\u{230}\x05\x68\x35\x02\u{230}\
	\u{239}\x03\x02\x02\x02\u{231}\u{239}\x05\x54\x2b\x02\u{232}\u{239}\x05\
	\x58\x2d\x02\u{233}\u{239}\x05\x66\x34\x02\u{234}\u{239}\x05\x62\x32\x02\
	\u{235}\u{236}\x05\u{82}\x42\x02\u{236}\u{237}\x05\u{86}\x44\x02\u{237}\
	\u{239}\x03\x02\x02\x02\u{238}\u{22d}\x03\x02\x02\x02\u{238}\u{231}\x03\
	\x02\x02\x02\u{238}\u{232}\x03\x02\x02\x02\u{238}\u{233}\x03\x02\x02\x02\
	\u{238}\u{234}\x03\x02\x02\x02\u{238}\u{235}\x03\x02\x02\x02\u{239}\u{23a}\
	\x03\x02\x02\x02\u{23a}\u{23b}\x05\u{98}\x4d\x02\u{23b}\x51\x03\x02\x02\
	\x02\u{23c}\u{240}\x05\u{84}\x43\x02\u{23d}\u{23e}\x07\x31\x02\x02\u{23e}\
	\u{241}\x05\x68\x35\x02\u{23f}\u{241}\x05\u{86}\x44\x02\u{240}\u{23d}\x03\
	\x02\x02\x02\u{240}\u{23f}\x03\x02\x02\x02\u{241}\u{250}\x03\x02\x02\x02\
	\u{242}\u{243}\x07\x18\x02\x02\u{243}\u{244}\x05\u{90}\x49\x02\u{244}\u{245}\
	\x07\x1a\x02\x02\u{245}\u{246}\x07\x31\x02\x02\u{246}\u{247}\x05\u{84}\x43\
	\x02\u{247}\u{248}\x05\u{86}\x44\x02\u{248}\u{250}\x03\x02\x02\x02\u{249}\
	\u{250}\x07\x36\x02\x02\u{24a}\u{250}\x07\x37\x02\x02\u{24b}\u{250}\x05\
	\x56\x2c\x02\u{24c}\u{250}\x05\x5a\x2e\x02\u{24d}\u{250}\x05\x60\x31\x02\
	\u{24e}\u{250}\x05\x64\x33\x02\u{24f}\u{23c}\x03\x02\x02\x02\u{24f}\u{242}\
	\x03\x02\x02\x02\u{24f}\u{249}\x03\x02\x02\x02\u{24f}\u{24a}\x03\x02\x02\
	\x02\u{24f}\u{24b}\x03\x02\x02\x02\u{24f}\u{24c}\x03\x02\x02\x02\u{24f}\
	\u{24d}\x03\x02\x02\x02\u{24f}\u{24e}\x03\x02\x02\x02\u{250}\u{251}\x03\
	\x02\x02\x02\u{251}\u{252}\x05\u{98}\x4d\x02\u{252}\x53\x03\x02\x02\x02\
	\u{253}\u{254}\x07\x30\x02\x02\u{254}\u{255}\x05\x68\x35\x02\u{255}\u{25b}\
	\x07\x38\x02\x02\u{256}\u{257}\x05\x50\x29\x02\u{257}\u{258}\x07\x04\x02\
	\x02\u{258}\u{25a}\x03\x02\x02\x02\u{259}\u{256}\x03\x02\x02\x02\u{25a}\
	\u{25d}\x03\x02\x02\x02\u{25b}\u{259}\x03\x02\x02\x02\u{25b}\u{25c}\x03\
	\x02\x02\x02\u{25c}\u{26b}\x03\x02\x02\x02\u{25d}\u{25b}\x03\x02\x02\x02\
	\u{25e}\u{25f}\x07\x39\x02\x02\u{25f}\u{260}\x05\x68\x35\x02\u{260}\u{266}\
	\x07\x38\x02\x02\u{261}\u{262}\x05\x50\x29\x02\u{262}\u{263}\x07\x04\x02\
	\x02\u{263}\u{265}\x03\x02\x02\x02\u{264}\u{261}\x03\x02\x02\x02\u{265}\
	\u{268}\x03\x02\x02\x02\u{266}\u{264}\x03\x02\x02\x02\u{266}\u{267}\x03\
	\x02\x02\x02\u{267}\u{26a}\x03\x02\x02\x02\u{268}\u{266}\x03\x02\x02\x02\
	\u{269}\u{25e}\x03\x02\x02\x02\u{26a}\u{26d}\x03\x02\x02\x02\u{26b}\u{269}\
	\x03\x02\x02\x02\u{26b}\u{26c}\x03\x02\x02\x02\u{26c}\u{277}\x03\x02\x02\
	\x02\u{26d}\u{26b}\x03\x02\x02\x02\u{26e}\u{274}\x07\x3a\x02\x02\u{26f}\
	\u{270}\x05\x50\x29\x02\u{270}\u{271}\x07\x04\x02\x02\u{271}\u{273}\x03\
	\x02\x02\x02\u{272}\u{26f}\x03\x02\x02\x02\u{273}\u{276}\x03\x02\x02\x02\
	\u{274}\u{272}\x03\x02\x02\x02\u{274}\u{275}\x03\x02\x02\x02\u{275}\u{278}\
	\x03\x02\x02\x02\u{276}\u{274}\x03\x02\x02\x02\u{277}\u{26e}\x03\x02\x02\
	\x02\u{277}\u{278}\x03\x02\x02\x02\u{278}\u{279}\x03\x02\x02\x02\u{279}\
	\u{27a}\x07\x14\x02\x02\u{27a}\u{27b}\x07\x30\x02\x02\u{27b}\x55\x03\x02\
	\x02\x02\u{27c}\u{27d}\x07\x30\x02\x02\u{27d}\u{27e}\x05\x68\x35\x02\u{27e}\
	\u{284}\x07\x38\x02\x02\u{27f}\u{280}\x05\x52\x2a\x02\u{280}\u{281}\x07\
	\x04\x02\x02\u{281}\u{283}\x03\x02\x02\x02\u{282}\u{27f}\x03\x02\x02\x02\
	\u{283}\u{286}\x03\x02\x02\x02\u{284}\u{282}\x03\x02\x02\x02\u{284}\u{285}\
	\x03\x02\x02\x02\u{285}\u{294}\x03\x02\x02\x02\u{286}\u{284}\x03\x02\x02\
	\x02\u{287}\u{288}\x07\x39\x02\x02\u{288}\u{289}\x05\x68\x35\x02\u{289}\
	\u{28f}\x07\x38\x02\x02\u{28a}\u{28b}\x05\x52\x2a\x02\u{28b}\u{28c}\x07\
	\x04\x02\x02\u{28c}\u{28e}\x03\x02\x02\x02\u{28d}\u{28a}\x03\x02\x02\x02\
	\u{28e}\u{291}\x03\x02\x02\x02\u{28f}\u{28d}\x03\x02\x02\x02\u{28f}\u{290}\
	\x03\x02\x02\x02\u{290}\u{293}\x03\x02\x02\x02\u{291}\u{28f}\x03\x02\x02\
	\x02\u{292}\u{287}\x03\x02\x02\x02\u{293}\u{296}\x03\x02\x02\x02\u{294}\
	\u{292}\x03\x02\x02\x02\u{294}\u{295}\x03\x02\x02\x02\u{295}\u{2a0}\x03\
	\x02\x02\x02\u{296}\u{294}\x03\x02\x02\x02\u{297}\u{29d}\x07\x3a\x02\x02\
	\u{298}\u{299}\x05\x52\x2a\x02\u{299}\u{29a}\x07\x04\x02\x02\u{29a}\u{29c}\
	\x03\x02\x02\x02\u{29b}\u{298}\x03\x02\x02\x02\u{29c}\u{29f}\x03\x02\x02\
	\x02\u{29d}\u{29b}\x03\x02\x02\x02\u{29d}\u{29e}\x03\x02\x02\x02\u{29e}\
	\u{2a1}\x03\x02\x02\x02\u{29f}\u{29d}\x03\x02\x02\x02\u{2a0}\u{297}\x03\
	\x02\x02\x02\u{2a0}\u{2a1}\x03\x02\x02\x02\u{2a1}\u{2a2}\x03\x02\x02\x02\
	\u{2a2}\u{2a3}\x07\x14\x02\x02\u{2a3}\u{2a4}\x07\x30\x02\x02\u{2a4}\x57\
	\x03\x02\x02\x02\u{2a5}\u{2a6}\x07\x3b\x02\x02\u{2a6}\u{2a7}\x05\x5c\x2f\
	\x02\u{2a7}\u{2ad}\x07\x3c\x02\x02\u{2a8}\u{2a9}\x05\x50\x29\x02\u{2a9}\
	\u{2aa}\x07\x04\x02\x02\u{2aa}\u{2ac}\x03\x02\x02\x02\u{2ab}\u{2a8}\x03\
	\x02\x02\x02\u{2ac}\u{2af}\x03\x02\x02\x02\u{2ad}\u{2ab}\x03\x02\x02\x02\
	\u{2ad}\u{2ae}\x03\x02\x02\x02\u{2ae}\u{2b0}\x03\x02\x02\x02\u{2af}\u{2ad}\
	\x03\x02\x02\x02\u{2b0}\u{2b1}\x07\x14\x02\x02\u{2b1}\u{2b2}\x07\x3b\x02\
	\x02\u{2b2}\x59\x03\x02\x02\x02\u{2b3}\u{2b4}\x07\x3b\x02\x02\u{2b4}\u{2b5}\
	\x05\x5c\x2f\x02\u{2b5}\u{2bb}\x07\x3c\x02\x02\u{2b6}\u{2b7}\x05\x52\x2a\
	\x02\u{2b7}\u{2b8}\x07\x04\x02\x02\u{2b8}\u{2ba}\x03\x02\x02\x02\u{2b9}\
	\u{2b6}\x03\x02\x02\x02\u{2ba}\u{2bd}\x03\x02\x02\x02\u{2bb}\u{2b9}\x03\
	\x02\x02\x02\u{2bb}\u{2bc}\x03\x02\x02\x02\u{2bc}\u{2be}\x03\x02\x02\x02\
	\u{2bd}\u{2bb}\x03\x02\x02\x02\u{2be}\u{2bf}\x07\x14\x02\x02\u{2bf}\u{2c0}\
	\x07\x3b\x02\x02\u{2c0}\x5b\x03\x02\x02\x02\u{2c1}\u{2c6}\x05\x5e\x30\x02\
	\u{2c2}\u{2c3}\x07\x1c\x02\x02\u{2c3}\u{2c5}\x05\x5e\x30\x02\u{2c4}\u{2c2}\
	\x03\x02\x02\x02\u{2c5}\u{2c8}\x03\x02\x02\x02\u{2c6}\u{2c4}\x03\x02\x02\
	\x02\u{2c6}\u{2c7}\x03\x02\x02\x02\u{2c7}\x5d\x03\x02\x02\x02\u{2c8}\u{2c6}\
	\x03\x02\x02\x02\u{2c9}\u{2cc}\x07\x5b\x02\x02\u{2ca}\u{2cb}\x07\x3d\x02\
	\x02\u{2cb}\u{2cd}\x05\x68\x35\x02\u{2cc}\u{2ca}\x03\x02\x02\x02\u{2cc}\
	\u{2cd}\x03\x02\x02\x02\u{2cd}\x5f\x03\x02\x02\x02\u{2ce}\u{2cf}\x07\x3e\
	\x02\x02\u{2cf}\u{2d0}\x05\x68\x35\x02\u{2d0}\u{2d6}\x07\x3c\x02\x02\u{2d1}\
	\u{2d2}\x05\x52\x2a\x02\u{2d2}\u{2d3}\x07\x04\x02\x02\u{2d3}\u{2d5}\x03\
	\x02\x02\x02\u{2d4}\u{2d1}\x03\x02\x02\x02\u{2d5}\u{2d8}\x03\x02\x02\x02\
	\u{2d6}\u{2d4}\x03\x02\x02\x02\u{2d6}\u{2d7}\x03\x02\x02\x02\u{2d7}\u{2d9}\
	\x03\x02\x02\x02\u{2d8}\u{2d6}\x03\x02\x02\x02\u{2d9}\u{2da}\x07\x14\x02\
	\x02\u{2da}\u{2db}\x07\x3e\x02\x02\u{2db}\x61\x03\x02\x02\x02\u{2dc}\u{2dd}\
	\x07\x3f\x02\x02\u{2dd}\u{2de}\x05\x68\x35\x02\u{2de}\u{2e4}\x07\x38\x02\
	\x02\u{2df}\u{2e0}\x05\x50\x29\x02\u{2e0}\u{2e1}\x07\x04\x02\x02\u{2e1}\
	\u{2e3}\x03\x02\x02\x02\u{2e2}\u{2df}\x03\x02\x02\x02\u{2e3}\u{2e6}\x03\
	\x02\x02\x02\u{2e4}\u{2e2}\x03\x02\x02\x02\u{2e4}\u{2e5}\x03\x02\x02\x02\
	\u{2e5}\u{2f4}\x03\x02\x02\x02\u{2e6}\u{2e4}\x03\x02\x02\x02\u{2e7}\u{2e8}\
	\x07\x40\x02\x02\u{2e8}\u{2e9}\x05\x68\x35\x02\u{2e9}\u{2ef}\x07\x38\x02\
	\x02\u{2ea}\u{2eb}\x05\x50\x29\x02\u{2eb}\u{2ec}\x07\x04\x02\x02\u{2ec}\
	\u{2ee}\x03\x02\x02\x02\u{2ed}\u{2ea}\x03\x02\x02\x02\u{2ee}\u{2f1}\x03\
	\x02\x02\x02\u{2ef}\u{2ed}\x03\x02\x02\x02\u{2ef}\u{2f0}\x03\x02\x02\x02\
	\u{2f0}\u{2f3}\x03\x02\x02\x02\u{2f1}\u{2ef}\x03\x02\x02\x02\u{2f2}\u{2e7}\
	\x03\x02\x02\x02\u{2f3}\u{2f6}\x03\x02\x02\x02\u{2f4}\u{2f2}\x03\x02\x02\
	\x02\u{2f4}\u{2f5}\x03\x02\x02\x02\u{2f5}\u{2f7}\x03\x02\x02\x02\u{2f6}\
	\u{2f4}\x03\x02\x02\x02\u{2f7}\u{2f8}\x07\x14\x02\x02\u{2f8}\u{2f9}\x07\
	\x3f\x02\x02\u{2f9}\x63\x03\x02\x02\x02\u{2fa}\u{2fb}\x07\x3f\x02\x02\u{2fb}\
	\u{2fc}\x05\x68\x35\x02\u{2fc}\u{302}\x07\x38\x02\x02\u{2fd}\u{2fe}\x05\
	\x52\x2a\x02\u{2fe}\u{2ff}\x07\x04\x02\x02\u{2ff}\u{301}\x03\x02\x02\x02\
	\u{300}\u{2fd}\x03\x02\x02\x02\u{301}\u{304}\x03\x02\x02\x02\u{302}\u{300}\
	\x03\x02\x02\x02\u{302}\u{303}\x03\x02\x02\x02\u{303}\u{312}\x03\x02\x02\
	\x02\u{304}\u{302}\x03\x02\x02\x02\u{305}\u{306}\x07\x40\x02\x02\u{306}\
	\u{307}\x05\x68\x35\x02\u{307}\u{30d}\x07\x38\x02\x02\u{308}\u{309}\x05\
	\x52\x2a\x02\u{309}\u{30a}\x07\x04\x02\x02\u{30a}\u{30c}\x03\x02\x02\x02\
	\u{30b}\u{308}\x03\x02\x02\x02\u{30c}\u{30f}\x03\x02\x02\x02\u{30d}\u{30b}\
	\x03\x02\x02\x02\u{30d}\u{30e}\x03\x02\x02\x02\u{30e}\u{311}\x03\x02\x02\
	\x02\u{30f}\u{30d}\x03\x02\x02\x02\u{310}\u{305}\x03\x02\x02\x02\u{311}\
	\u{314}\x03\x02\x02\x02\u{312}\u{310}\x03\x02\x02\x02\u{312}\u{313}\x03\
	\x02\x02\x02\u{313}\u{315}\x03\x02\x02\x02\u{314}\u{312}\x03\x02\x02\x02\
	\u{315}\u{316}\x07\x14\x02\x02\u{316}\u{317}\x07\x3f\x02\x02\u{317}\x65\
	\x03\x02\x02\x02\u{318}\u{319}\x07\x41\x02\x02\u{319}\u{31a}\x07\x18\x02\
	\x02\u{31a}\u{31b}\x05\u{84}\x43\x02\u{31b}\u{31c}\x07\x1c\x02\x02\u{31c}\
	\u{31d}\x05\u{84}\x43\x02\u{31d}\u{31e}\x07\x1a\x02\x02\u{31e}\x67\x03\x02\
	\x02\x02\u{31f}\u{332}\x05\x6a\x36\x02\u{320}\u{321}\x07\x30\x02\x02\u{321}\
	\u{322}\x05\x68\x35\x02\u{322}\u{323}\x07\x38\x02\x02\u{323}\u{32b}\x05\
	\x68\x35\x02\u{324}\u{325}\x07\x39\x02\x02\u{325}\u{326}\x05\x68\x35\x02\
	\u{326}\u{327}\x07\x38\x02\x02\u{327}\u{328}\x05\x68\x35\x02\u{328}\u{32a}\
	\x03\x02\x02\x02\u{329}\u{324}\x03\x02\x02\x02\u{32a}\u{32d}\x03\x02\x02\
	\x02\u{32b}\u{329}\x03\x02\x02\x02\u{32b}\u{32c}\x03\x02\x02\x02\u{32c}\
	\u{32e}\x03\x02\x02\x02\u{32d}\u{32b}\x03\x02\x02\x02\u{32e}\u{32f}\x07\
	\x3a\x02\x02\u{32f}\u{330}\x05\x68\x35\x02\u{330}\u{332}\x03\x02\x02\x02\
	\u{331}\u{31f}\x03\x02\x02\x02\u{331}\u{320}\x03\x02\x02\x02\u{332}\x69\
	\x03\x02\x02\x02\u{333}\u{33a}\x05\x6c\x37\x02\u{334}\u{335}\x07\x19\x02\
	\x02\u{335}\u{338}\x05\x6c\x37\x02\u{336}\u{337}\x07\x19\x02\x02\u{337}\
	\u{339}\x05\x6c\x37\x02\u{338}\u{336}\x03\x02\x02\x02\u{338}\u{339}\x03\
	\x02\x02\x02\u{339}\u{33b}\x03\x02\x02\x02\u{33a}\u{334}\x03\x02\x02\x02\
	\u{33a}\u{33b}\x03\x02\x02\x02\u{33b}\x6b\x03\x02\x02\x02\u{33c}\u{341}\
	\x05\x6e\x38\x02\u{33d}\u{33e}\x07\x42\x02\x02\u{33e}\u{340}\x05\x6e\x38\
	\x02\u{33f}\u{33d}\x03\x02\x02\x02\u{340}\u{343}\x03\x02\x02\x02\u{341}\
	\u{33f}\x03\x02\x02\x02\u{341}\u{342}\x03\x02\x02\x02\u{342}\x6d\x03\x02\
	\x02\x02\u{343}\u{341}\x03\x02\x02\x02\u{344}\u{349}\x05\x70\x39\x02\u{345}\
	\u{346}\x07\x43\x02\x02\u{346}\u{348}\x05\x70\x39\x02\u{347}\u{345}\x03\
	\x02\x02\x02\u{348}\u{34b}\x03\x02\x02\x02\u{349}\u{347}\x03\x02\x02\x02\
	\u{349}\u{34a}\x03\x02\x02\x02\u{34a}\x6f\x03\x02\x02\x02\u{34b}\u{349}\
	\x03\x02\x02\x02\u{34c}\u{34e}\x07\x44\x02\x02\u{34d}\u{34c}\x03\x02\x02\
	\x02\u{34d}\u{34e}\x03\x02\x02\x02\u{34e}\u{34f}\x03\x02\x02\x02\u{34f}\
	\u{350}\x05\x72\x3a\x02\u{350}\x71\x03\x02\x02\x02\u{351}\u{355}\x05\x76\
	\x3c\x02\u{352}\u{353}\x05\x74\x3b\x02\u{353}\u{354}\x05\x76\x3c\x02\u{354}\
	\u{356}\x03\x02\x02\x02\u{355}\u{352}\x03\x02\x02\x02\u{355}\u{356}\x03\
	\x02\x02\x02\u{356}\x73\x03\x02\x02\x02\u{357}\u{358}\x09\x06\x02\x02\u{358}\
	\x75\x03\x02\x02\x02\u{359}\u{35b}\x05\x78\x3d\x02\u{35a}\u{359}\x03\x02\
	\x02\x02\u{35a}\u{35b}\x03\x02\x02\x02\u{35b}\u{35c}\x03\x02\x02\x02\u{35c}\
	\u{362}\x05\x7a\x3e\x02\u{35d}\u{35e}\x05\x78\x3d\x02\u{35e}\u{35f}\x05\
	\x7a\x3e\x02\u{35f}\u{361}\x03\x02\x02\x02\u{360}\u{35d}\x03\x02\x02\x02\
	\u{361}\u{364}\x03\x02\x02\x02\u{362}\u{360}\x03\x02\x02\x02\u{362}\u{363}\
	\x03\x02\x02\x02\u{363}\x77\x03\x02\x02\x02\u{364}\u{362}\x03\x02\x02\x02\
	\u{365}\u{366}\x09\x07\x02\x02\u{366}\x79\x03\x02\x02\x02\u{367}\u{36d}\
	\x05\x7e\x40\x02\u{368}\u{369}\x05\x7c\x3f\x02\u{369}\u{36a}\x05\x7e\x40\
	\x02\u{36a}\u{36c}\x03\x02\x02\x02\u{36b}\u{368}\x03\x02\x02\x02\u{36c}\
	\u{36f}\x03\x02\x02\x02\u{36d}\u{36b}\x03\x02\x02\x02\u{36d}\u{36e}\x03\
	\x02\x02\x02\u{36e}\x7b\x03\x02\x02\x02\u{36f}\u{36d}\x03\x02\x02\x02\u{370}\
	\u{371}\x09\x08\x02\x02\u{371}\x7d\x03\x02\x02\x02\u{372}\u{375}\x05\u{80}\
	\x41\x02\u{373}\u{374}\x09\x09\x02\x02\u{374}\u{376}\x05\u{80}\x41\x02\u{375}\
	\u{373}\x03\x02\x02\x02\u{375}\u{376}\x03\x02\x02\x02\u{376}\x7f\x03\x02\
	\x02\x02\u{377}\u{397}\x07\x5d\x02\x02\u{378}\u{397}\x07\x5c\x02\x02\u{379}\
	\u{397}\x07\x54\x02\x02\u{37a}\u{397}\x07\x55\x02\x02\u{37b}\u{37f}\x05\
	\u{82}\x42\x02\u{37c}\u{37f}\x07\x1b\x02\x02\u{37d}\u{37f}\x07\x33\x02\x02\
	\u{37e}\u{37b}\x03\x02\x02\x02\u{37e}\u{37c}\x03\x02\x02\x02\u{37e}\u{37d}\
	\x03\x02\x02\x02\u{37f}\u{380}\x03\x02\x02\x02\u{380}\u{397}\x05\u{86}\x44\
	\x02\u{381}\u{397}\x05\u{84}\x43\x02\u{382}\u{383}\x07\x18\x02\x02\u{383}\
	\u{384}\x05\u{90}\x49\x02\u{384}\u{385}\x07\x1a\x02\x02\u{385}\u{397}\x03\
	\x02\x02\x02\u{386}\u{387}\x07\x56\x02\x02\u{387}\u{38c}\x05\u{92}\x4a\x02\
	\u{388}\u{389}\x07\x04\x02\x02\u{389}\u{38b}\x05\u{92}\x4a\x02\u{38a}\u{388}\
	\x03\x02\x02\x02\u{38b}\u{38e}\x03\x02\x02\x02\u{38c}\u{38a}\x03\x02\x02\
	\x02\u{38c}\u{38d}\x03\x02\x02\x02\u{38d}\u{38f}\x03\x02\x02\x02\u{38e}\
	\u{38c}\x03\x02\x02\x02\u{38f}\u{390}\x07\x57\x02\x02\u{390}\u{397}\x03\
	\x02\x02\x02\u{391}\u{392}\x07\x58\x02\x02\u{392}\u{393}\x05\u{88}\x45\x02\
	\u{393}\u{394}\x07\x27\x02\x02\u{394}\u{397}\x03\x02\x02\x02\u{395}\u{397}\
	\x07\x14\x02\x02\u{396}\u{377}\x03\x02\x02\x02\u{396}\u{378}\x03\x02\x02\
	\x02\u{396}\u{379}\x03\x02\x02\x02\u{396}\u{37a}\x03\x02\x02\x02\u{396}\
	\u{37e}\x03\x02\x02\x02\u{396}\u{381}\x03\x02\x02\x02\u{396}\u{382}\x03\
	\x02\x02\x02\u{396}\u{386}\x03\x02\x02\x02\u{396}\u{391}\x03\x02\x02\x02\
	\u{396}\u{395}\x03\x02\x02\x02\u{397}\u{81}\x03\x02\x02\x02\u{398}\u{39a}\
	\x07\x59\x02\x02\u{399}\u{398}\x03\x02\x02\x02\u{399}\u{39a}\x03\x02\x02\
	\x02\u{39a}\u{39b}\x03\x02\x02\x02\u{39b}\u{3a0}\x07\x5b\x02\x02\u{39c}\
	\u{39d}\x07\x59\x02\x02\u{39d}\u{39f}\x07\x5b\x02\x02\u{39e}\u{39c}\x03\
	\x02\x02\x02\u{39f}\u{3a2}\x03\x02\x02\x02\u{3a0}\u{39e}\x03\x02\x02\x02\
	\u{3a0}\u{3a1}\x03\x02\x02\x02\u{3a1}\u{83}\x03\x02\x02\x02\u{3a2}\u{3a0}\
	\x03\x02\x02\x02\u{3a3}\u{3a5}\x07\x59\x02\x02\u{3a4}\u{3a3}\x03\x02\x02\
	\x02\u{3a4}\u{3a5}\x03\x02\x02\x02\u{3a5}\u{3a6}\x03\x02\x02\x02\u{3a6}\
	\u{3a8}\x07\x5b\x02\x02\u{3a7}\u{3a9}\x05\u{94}\x4b\x02\u{3a8}\u{3a7}\x03\
	\x02\x02\x02\u{3a8}\u{3a9}\x03\x02\x02\x02\u{3a9}\u{3b1}\x03\x02\x02\x02\
	\u{3aa}\u{3ab}\x07\x59\x02\x02\u{3ab}\u{3ad}\x07\x5b\x02\x02\u{3ac}\u{3ae}\
	\x05\u{94}\x4b\x02\u{3ad}\u{3ac}\x03\x02\x02\x02\u{3ad}\u{3ae}\x03\x02\x02\
	\x02\u{3ae}\u{3b0}\x03\x02\x02\x02\u{3af}\u{3aa}\x03\x02\x02\x02\u{3b0}\
	\u{3b3}\x03\x02\x02\x02\u{3b1}\u{3af}\x03\x02\x02\x02\u{3b1}\u{3b2}\x03\
	\x02\x02\x02\u{3b2}\u{85}\x03\x02\x02\x02\u{3b3}\u{3b1}\x03\x02\x02\x02\
	\u{3b4}\u{3b6}\x07\x18\x02\x02\u{3b5}\u{3b7}\x05\u{88}\x45\x02\u{3b6}\u{3b5}\
	\x03\x02\x02\x02\u{3b6}\u{3b7}\x03\x02\x02\x02\u{3b7}\u{3b8}\x03\x02\x02\
	\x02\u{3b8}\u{3b9}\x07\x1a\x02\x02\u{3b9}\u{87}\x03\x02\x02\x02\u{3ba}\u{3bf}\
	\x05\u{8e}\x48\x02\u{3bb}\u{3bc}\x07\x1c\x02\x02\u{3bc}\u{3c0}\x05\u{88}\
	\x45\x02\u{3bd}\u{3be}\x07\x3b\x02\x02\u{3be}\u{3c0}\x05\x5c\x2f\x02\u{3bf}\
	\u{3bb}\x03\x02\x02\x02\u{3bf}\u{3bd}\x03\x02\x02\x02\u{3bf}\u{3c0}\x03\
	\x02\x02\x02\u{3c0}\u{3c3}\x03\x02\x02\x02\u{3c1}\u{3c3}\x05\u{8a}\x46\x02\
	\u{3c2}\u{3ba}\x03\x02\x02\x02\u{3c2}\u{3c1}\x03\x02\x02\x02\u{3c3}\u{89}\
	\x03\x02\x02\x02\u{3c4}\u{3c7}\x05\u{8c}\x47\x02\u{3c5}\u{3c6}\x07\x1c\x02\
	\x02\u{3c6}\u{3c8}\x05\u{8a}\x46\x02\u{3c7}\u{3c5}\x03\x02\x02\x02\u{3c7}\
	\u{3c8}\x03\x02\x02\x02\u{3c8}\u{8b}\x03\x02\x02\x02\u{3c9}\u{3ca}\x07\x5b\
	\x02\x02\u{3ca}\u{3cb}\x07\x16\x02\x02\u{3cb}\u{3cc}\x05\u{8e}\x48\x02\u{3cc}\
	\u{8d}\x03\x02\x02\x02\u{3cd}\u{3ce}\x07\x13\x02\x02\u{3ce}\u{3cf}\x05\u{82}\
	\x42\x02\u{3cf}\u{3d1}\x07\x18\x02\x02\u{3d0}\u{3d2}\x05\u{8a}\x46\x02\u{3d1}\
	\u{3d0}\x03\x02\x02\x02\u{3d1}\u{3d2}\x03\x02\x02\x02\u{3d2}\u{3d3}\x03\
	\x02\x02\x02\u{3d3}\u{3d4}\x07\x1a\x02\x02\u{3d4}\u{3d7}\x03\x02\x02\x02\
	\u{3d5}\u{3d7}\x05\x68\x35\x02\u{3d6}\u{3cd}\x03\x02\x02\x02\u{3d6}\u{3d5}\
	\x03\x02\x02\x02\u{3d7}\u{8f}\x03\x02\x02\x02\u{3d8}\u{3da}\x05\x68\x35\
	\x02\u{3d9}\u{3d8}\x03\x02\x02\x02\u{3d9}\u{3da}\x03\x02\x02\x02\u{3da}\
	\u{3e1}\x03\x02\x02\x02\u{3db}\u{3dd}\x07\x1c\x02\x02\u{3dc}\u{3de}\x05\
	\x68\x35\x02\u{3dd}\u{3dc}\x03\x02\x02\x02\u{3dd}\u{3de}\x03\x02\x02\x02\
	\u{3de}\u{3e0}\x03\x02\x02\x02\u{3df}\u{3db}\x03\x02\x02\x02\u{3e0}\u{3e3}\
	\x03\x02\x02\x02\u{3e1}\u{3df}\x03\x02\x02\x02\u{3e1}\u{3e2}\x03\x02\x02\
	\x02\u{3e2}\u{91}\x03\x02\x02\x02\u{3e3}\u{3e1}\x03\x02\x02\x02\u{3e4}\u{3e9}\
	\x05\x68\x35\x02\u{3e5}\u{3e6}\x07\x1c\x02\x02\u{3e6}\u{3e8}\x05\x68\x35\
	\x02\u{3e7}\u{3e5}\x03\x02\x02\x02\u{3e8}\u{3eb}\x03\x02\x02\x02\u{3e9}\
	\u{3e7}\x03\x02\x02\x02\u{3e9}\u{3ea}\x03\x02\x02\x02\u{3ea}\u{93}\x03\x02\
	\x02\x02\u{3eb}\u{3e9}\x03\x02\x02\x02\u{3ec}\u{3ed}\x07\x56\x02\x02\u{3ed}\
	\u{3f2}\x05\u{96}\x4c\x02\u{3ee}\u{3ef}\x07\x1c\x02\x02\u{3ef}\u{3f1}\x05\
	\u{96}\x4c\x02\u{3f0}\u{3ee}\x03\x02\x02\x02\u{3f1}\u{3f4}\x03\x02\x02\x02\
	\u{3f2}\u{3f0}\x03\x02\x02\x02\u{3f2}\u{3f3}\x03\x02\x02\x02\u{3f3}\u{3f5}\
	\x03\x02\x02\x02\u{3f4}\u{3f2}\x03\x02\x02\x02\u{3f5}\u{3f6}\x07\x57\x02\
	\x02\u{3f6}\u{95}\x03\x02\x02\x02\u{3f7}\u{3fa}\x07\x19\x02\x02\u{3f8}\u{3fa}\
	\x05\x68\x35\x02\u{3f9}\u{3f7}\x03\x02\x02\x02\u{3f9}\u{3f8}\x03\x02\x02\
	\x02\u{3fa}\u{97}\x03\x02\x02\x02\u{3fb}\u{3fd}\x05\u{9a}\x4e\x02\u{3fc}\
	\u{3fe}\x05\u{9c}\x4f\x02\u{3fd}\u{3fc}\x03\x02\x02\x02\u{3fd}\u{3fe}\x03\
	\x02\x02\x02\u{3fe}\u{99}\x03\x02\x02\x02\u{3ff}\u{404}\x07\x5c\x02\x02\
	\u{400}\u{401}\x07\x4b\x02\x02\u{401}\u{403}\x07\x5c\x02\x02\u{402}\u{400}\
	\x03\x02\x02\x02\u{403}\u{406}\x03\x02\x02\x02\u{404}\u{402}\x03\x02\x02\
	\x02\u{404}\u{405}\x03\x02\x02\x02\u{405}\u{408}\x03\x02\x02\x02\u{406}\
	\u{404}\x03\x02\x02\x02\u{407}\u{3ff}\x03\x02\x02\x02\u{407}\u{408}\x03\
	\x02\x02\x02\u{408}\u{9b}\x03\x02\x02\x02\u{409}\u{40a}\x07\x5a\x02\x02\
	\u{40a}\u{40b}\x05\x38\x1d\x02\u{40b}\u{9d}\x03\x02\x02\x02\u{86}\u{a0}\
	\u{a5}\u{a9}\u{b0}\u{b6}\u{be}\u{c1}\u{c6}\u{cb}\u{d1}\u{d4}\u{d8}\u{e3}\
	\u{ea}\u{f1}\u{f4}\u{fd}\u{100}\u{104}\u{111}\u{11e}\u{12b}\u{12d}\u{132}\
	\u{135}\u{138}\u{13b}\u{140}\u{147}\u{14c}\u{155}\u{15b}\u{15e}\u{161}\u{164}\
	\u{168}\u{16d}\u{172}\u{174}\u{176}\u{185}\u{18e}\u{194}\u{197}\u{19c}\u{1a1}\
	\u{1a6}\u{1a9}\u{1ac}\u{1b5}\u{1ba}\u{1c3}\u{1c6}\u{1cb}\u{1d1}\u{1d5}\u{1de}\
	\u{1e3}\u{1e6}\u{1e9}\u{1ed}\u{1f1}\u{1f7}\u{1fa}\u{1fe}\u{201}\u{206}\u{209}\
	\u{216}\u{21e}\u{222}\u{22a}\u{238}\u{240}\u{24f}\u{25b}\u{266}\u{26b}\u{274}\
	\u{277}\u{284}\u{28f}\u{294}\u{29d}\u{2a0}\u{2ad}\u{2bb}\u{2c6}\u{2cc}\u{2d6}\
	\u{2e4}\u{2ef}\u{2f4}\u{302}\u{30d}\u{312}\u{32b}\u{331}\u{338}\u{33a}\u{341}\
	\u{349}\u{34d}\u{355}\u{35a}\u{362}\u{36d}\u{375}\u{37e}\u{38c}\u{396}\u{399}\
	\u{3a0}\u{3a4}\u{3a8}\u{3ad}\u{3b1}\u{3b6}\u{3bf}\u{3c2}\u{3c7}\u{3d1}\u{3d6}\
	\u{3d9}\u{3dd}\u{3e1}\u{3e9}\u{3f2}\u{3f9}\u{3fd}\u{404}\u{407}";

