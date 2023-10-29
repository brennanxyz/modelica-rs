//! # Wrapping the Modelica language in Rust
//! 
//! This crate is a wrapper for the Modelica language, used primarily for
//! modeling physical systems. It is **not** ready for use by anyone for any reason.


/// An element defined by the production component-clause in the Modelica grammar
pub struct ModelicaComponent {
    pub name: String,
}

/// Content ignored by the Modelica translator
/// 
/// Text on a line following the // character pair is ignored by the Modelica translator.
/// Also, text located between the character pairs /* and */ is ignored by the Modelica translator.
pub struct ModelicaComment {
    pub comment: String,
}

/// Located at the end of a declaration, equation, or statement or at the beginning of a class definition
pub struct ModelicaDescription {
    pub comment: String,
}

/// A combination of Modelica values, variables, and operators
pub struct ModelicaExpression {
    pub expression: String,
}

/// Reserved Modelica keywords
#[allow(dead_code)]
pub enum ModelicaKeyword {
    Algorithm, And, Annotation,
    Block, Break,
    Class, Connect, Connector, Constant, ConstrainedBy,
    Der, Discrete,
    Each, Else, ElseIf, ElseWhen, Encapsulated, End, Enumeration, Equation, Expandable, Extends, External,
    False, Final, Flow, For, Function,
    If, Import, Impure, In, Initial, Inner, Input,
    Loop,
    Model,
    Not,
    Operator, Or, Outer, Output,
    Package, Parameter, Partial, Protected, Public, Pure,
    Record, Redeclare, Replaceable, Return,
    Stream,
    Time, Then, True, Type,
    When, While, Within,
}

#[allow(dead_code)]
fn get_keyword_str(keyword: ModelicaKeyword) -> &'static str {
    match keyword {
        ModelicaKeyword::Algorithm => "algorithm",
        ModelicaKeyword::And => "and",
        ModelicaKeyword::Annotation => "annotation",
        ModelicaKeyword::Block => "block",
        ModelicaKeyword::Break => "break",
        ModelicaKeyword::Class => "class",
        ModelicaKeyword::Connect => "connect",
        ModelicaKeyword::Connector => "connector",
        ModelicaKeyword::Constant => "constant",
        ModelicaKeyword::ConstrainedBy => "constrainedby",
        ModelicaKeyword::Der => "der",
        ModelicaKeyword::Discrete => "discrete",
        ModelicaKeyword::Each => "each",
        ModelicaKeyword::Else => "else",
        ModelicaKeyword::ElseIf => "elseif",
        ModelicaKeyword::ElseWhen => "elsewhen",
        ModelicaKeyword::Encapsulated => "encapsulated",
        ModelicaKeyword::End => "end",
        ModelicaKeyword::Enumeration => "enumeration",
        ModelicaKeyword::Equation => "equation",
        ModelicaKeyword::Expandable => "expandable",
        ModelicaKeyword::Extends => "extends",
        ModelicaKeyword::External => "external",
        ModelicaKeyword::False => "false",
        ModelicaKeyword::Final => "final",
        ModelicaKeyword::Flow => "flow",
        ModelicaKeyword::For => "for",
        ModelicaKeyword::Function => "function",
        ModelicaKeyword::If => "if",
        ModelicaKeyword::Import => "import",
        ModelicaKeyword::Impure => "impure",
        ModelicaKeyword::In => "in",
        ModelicaKeyword::Initial => "initial",
        ModelicaKeyword::Inner => "inner",
        ModelicaKeyword::Input => "input",
        ModelicaKeyword::Loop => "loop",
        ModelicaKeyword::Model => "model",
        ModelicaKeyword::Not => "not",
        ModelicaKeyword::Operator => "operator",
        ModelicaKeyword::Or => "or",
        ModelicaKeyword::Outer => "outer",
        ModelicaKeyword::Output => "output",
        ModelicaKeyword::Package => "package",
        ModelicaKeyword::Parameter => "parameter",
        ModelicaKeyword::Partial => "partial",
        ModelicaKeyword::Protected => "protected",
        ModelicaKeyword::Public => "public",
        ModelicaKeyword::Pure => "pure",
        ModelicaKeyword::Record => "record",
        ModelicaKeyword::Redeclare => "redeclare",
        ModelicaKeyword::Replaceable => "replaceable",
        ModelicaKeyword::Return => "return",
        ModelicaKeyword::Stream => "stream",
        ModelicaKeyword::Time => "time",
        ModelicaKeyword::Then => "then",
        ModelicaKeyword::True => "true",
        ModelicaKeyword::Type => "type",
        ModelicaKeyword::When => "when",
        ModelicaKeyword::While => "while",
        ModelicaKeyword::Within => "within",
    }
}

#[allow(dead_code)]
fn get_keyword(keyword: &str) -> Option<ModelicaKeyword> {
    match keyword {
        "algorithm" => Some(ModelicaKeyword::Algorithm),
        "and" => Some(ModelicaKeyword::And),
        "annotation" => Some(ModelicaKeyword::Annotation),
        "block" => Some(ModelicaKeyword::Block),
        "break" => Some(ModelicaKeyword::Break),
        "class" => Some(ModelicaKeyword::Class),
        "connect" => Some(ModelicaKeyword::Connect),
        "connector" => Some(ModelicaKeyword::Connector),
        "constant" => Some(ModelicaKeyword::Constant),
        "constrainedby" => Some(ModelicaKeyword::ConstrainedBy),
        "der" => Some(ModelicaKeyword::Der),
        "discrete" => Some(ModelicaKeyword::Discrete),
        "each" => Some(ModelicaKeyword::Each),
        "else" => Some(ModelicaKeyword::Else),
        "elseif" => Some(ModelicaKeyword::ElseIf),
        "elsewhen" => Some(ModelicaKeyword::ElseWhen),
        "encapsulated" => Some(ModelicaKeyword::Encapsulated),
        "end" => Some(ModelicaKeyword::End),
        "enumeration" => Some(ModelicaKeyword::Enumeration),
        "equation" => Some(ModelicaKeyword::Equation),
        "expandable" => Some(ModelicaKeyword::Expandable),
        "extends" => Some(ModelicaKeyword::Extends),
        "external" => Some(ModelicaKeyword::External),
        "false" => Some(ModelicaKeyword::False),
        "final" => Some(ModelicaKeyword::Final),
        "flow" => Some(ModelicaKeyword::Flow),
        "for" => Some(ModelicaKeyword::For),
        "function" => Some(ModelicaKeyword::Function),
        "if" => Some(ModelicaKeyword::If),
        "import" => Some(ModelicaKeyword::Import),
        "impure" => Some(ModelicaKeyword::Impure),
        "in" => Some(ModelicaKeyword::In),
        "initial" => Some(ModelicaKeyword::Initial),
        "inner" => Some(ModelicaKeyword::Inner),
        "input" => Some(ModelicaKeyword::Input),
        "loop" => Some(ModelicaKeyword::Loop),
        "model" => Some(ModelicaKeyword::Model),
        "not" => Some(ModelicaKeyword::Not),
        "operator" => Some(ModelicaKeyword::Operator),
        "or" => Some(ModelicaKeyword::Or),
        "outer" => Some(ModelicaKeyword::Outer),
        "output" => Some(ModelicaKeyword::Output),
        "package" => Some(ModelicaKeyword::Package),
        "parameter" => Some(ModelicaKeyword::Parameter),
        "partial" => Some(ModelicaKeyword::Partial),
        "protected" => Some(ModelicaKeyword::Protected),
        "public" => Some(ModelicaKeyword::Public),
        "pure" => Some(ModelicaKeyword::Pure),
        "record" => Some(ModelicaKeyword::Record),
        "redeclare" => Some(ModelicaKeyword::Redeclare),
        "replaceable" => Some(ModelicaKeyword::Replaceable),
        "return" => Some(ModelicaKeyword::Return),
        "stream" => Some(ModelicaKeyword::Stream),
        "time" => Some(ModelicaKeyword::Time),
        "then" => Some(ModelicaKeyword::Then),
        "true" => Some(ModelicaKeyword::True),
        "type" => Some(ModelicaKeyword::Type),
        "when" => Some(ModelicaKeyword::When),
        "while" => Some(ModelicaKeyword::While),
        "within" => Some(ModelicaKeyword::Within),
        _ => None,
    }
}

/// Reserved Modelica operators
pub enum ModelicaOperator {
    PostfixArrayIndex, // [] | arr[index]
    PostfixAccess, // . |  obj.property
    PostfixFunctionCall, // funcName (functionArguments ) | sin(4.36)
    ArrayConstruction, // {expressions } | {2, 3}
    HorizontalConcatenation, // [expressions ] | [5, 6]
    VerticalConcatenation, // [expressions; expressions...] | [2, 3; 7, 8]
    Exponentiation, // ^ | 2 ^ 3
    Multiplicative, // * | 2 * 3
    Divisive, // / | 2 / 3
    ElementwiseMultiplicative, // .* | [1, 2; 3, 4] .* [2, 3; 5, 6]
    ElementwiseDivisive, // ./ | [1, 2; 3, 4] ./ [2, 3; 5, 6]
    Additive, // + | 1 + 2
    Subtractive, // - | 2 - 1
    AdditiveUnary, // + | +2
    SubtractiveUnary, // - | -2
    ArrayElementwiseAdditive, // .+ | [1, 2; 3, 4] .+ [2, 3; 5, 6]
    ArrayElementwiseSubtractive, // .- | [1, 2; 3, 4] .- [2, 3; 5, 6]
    RelationatLessThan, // < | 2 < 3
    RelationatLessThanOrEqual, // <= | 2 <= 3
    RelationatGreaterThan, // > | 2 > 3
    RelationatGreaterThanOrEqual, // >= | 2 >= 3
    RelationatEqual, // == | 2 == 3
    RelationatNotEqual, // <> | 2 <> 3
    UnaryNegation, // not expr | not b1
    LogicalAnd, // and | b1 and b2
    LogicalOr, // or | b1 or b2
    ArrayRange, // expr : expr OR expr : expr : expr  | 1 : 5 OR start : step : stop
    Conditional, // if expr then expr else expr | if b then 3 else x
    NamedArgument, // ident = expr | x = 2.26
    AbsoluteValue, // abs(expr) | abs(-2.26)
    Sign, // sign(expr) | sign(-2.26)
    SquareRoot, // sqrt(expr) | sqrt(2.26)
    IntegerConvert, // Integer(e) | Integer(enum1)
    EnumTypeName, // EnumTypeName(i) | EnumTypeName(3)
    StringConvert, // String(..., <options>) | String(345) ; options are minimumLength, leftJustified, significantDigits
}