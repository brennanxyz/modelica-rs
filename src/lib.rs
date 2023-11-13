//! # Wrapping the Modelica language in Rust
//! 
//! This crate is a wrapper for the Modelica language, used primarily for
//! modeling physical systems. It is **not** ready for use by anyone for any reason.

// ANTLR is the way to go: https://github.com/rrevenantt/antlr4rust/blob/master/README.md
// Copy this: https://github.com/urbanopt/modelica-builder/blob/develop/modelica_builder/modelica_project.py

use regex::Regex;

/// Most declared blocks in Modelica are classes
pub trait ModelicaClass {
    fn get_name(&self) -> String;
    fn extract(text_in: String) -> (Option<Self>, Option<String>) where Self: Sized;
}

/// A `package` holds a collection of Modelica entities.
#[derive(Debug)]
pub struct ModelicaPackage {
    pub name: String,
}

impl ModelicaClass for ModelicaPackage {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn extract(text_in: String) -> (Option<ModelicaPackage>, Option<String>) {
        // let package_name_pattern = Regex::new(r#"\bpackage\s(\w+)"#).unwrap();
        let package_name_pattern = Regex::new(r#"package\s+(\w+)[.\s\S]+end\s+\1;"#).unwrap();
        let package: Option<ModelicaPackage> = None;
        let block_content: Option<String> = None;

        let mut count = 0;

        let captures = package_name_pattern.captures_iter(text_in.as_str());
        for capture in captures {
            println!("Found package {}", count);
            println!("Found package: {}", capture.get(1).unwrap().as_str());
            count += 1;
        }
        // println!("Found package: {}", capture.get(1).unwrap().as_str());

            // package = match capture.get(1) {
            //     Some(name) => {
            //         let n = name.as_str();
            //         Some(ModelicaPackage {
            //             name: n.to_string(),
            //         })
            //     },
            //     None => None,
            // };

        

        (package, block_content)
    }
}

/// A `type` may only be predefined types, enumerations, array of `type`, or classes extending from `type`
pub struct ModelicaType {
    pub name: String,
}

// impl ModelicaClass for ModelicaType {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// A `model` is a class that defines a set of variables that are connected to other connectors or to variables outside the model. 
/// 
/// A model may also contain equations, algorithm sections, and initial
/// equations.
pub struct ModelicaModel {
    pub name: String,
}

// impl ModelicaClass for ModelicaModel {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// A `block` is a class that defines a set of variables that are connected to other connectors or to variables outside the model.
/// 
/// A block may also contain equations, algorithm sections, and initial
/// equations.
pub struct ModelicaBlock {
    pub name: String,
}

// impl ModelicaClass for ModelicaBlock {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// A `connector` is a class that defines a set of variables that are connected to other connectors or to
/// variables outside the model. 
/// 
/// A connector may also contain equations, algorithm sections, and initial
/// equations.
pub struct ModelicaConnector {
    pub name: String,
}

// impl ModelicaClass for ModelicaConnector {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// A `function` is enhanced to allow the function to contain an external function interface.
pub struct ModelicaFunction {
    pub name: String,
}

// impl ModelicaClass for ModelicaFunction {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// A `record` is primarily used to group data together.
/// 
/// Only public sections are allowed in the definition or in any of its components 
/// (i.e., equation, algorithm, initial equation, initial algorithm and protected 
/// sections are not allowed). The elements of a record shall not have prefixes 
/// input, output, inner, outer, stream, or flow. Enhanced with implicitly available 
/// record constructor function, see section 12.6. The components directly declared 
/// in a record may only be of specialized class record or type.
pub struct ModelicaRecord {
    pub name: String,
}

// impl ModelicaClass for ModelicaRecord {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// An `operator record` is imilar to `record`; but operator overloading is possible
pub struct ModelicaOperatorRecord {
    pub name: String,
}

// impl ModelicaClass for ModelicaOperatorRecord {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// An `operator` is similar to `package``; but may only contain declarations of functions.
pub struct ModelicaOperatorClass {
    pub name: String,
}

// impl ModelicaClass for ModelicaOperatorClass {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }

/// An `operator function` is shorthand for an operator with exactly one function
pub struct ModelicaOperatorFunction {
    pub name: String,
}

// impl ModelicaClass for ModelicaOperatorFunction {
//     fn get_name(&self) -> String {
//         self.name.clone()
//     }
// }


/// Content ignored by the Modelica translator
/// 
/// Text on a line following the // character pair is ignored by the Modelica translator.
/// Also, text located between the character pairs /* and */ is ignored by the Modelica translator.
pub struct ModelicaComment {
    pub value: String,
}

pub enum ModelicaAccessControl {
    Public,
    Protected,
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

/// Reserved Modelica prefixes
#[allow(dead_code)]
pub enum ModelicaPrefix {
    Flow,
    Stream,
    Discrete,
    Parameter,
    Constant,
    Input,
    Output,
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
    EventDivision, // div(expr, expr) | div(5, 2)
    EventModulus, // mod(expr, expr) | mod(5, 2)
    EventRemainder, // rem(expr, expr) | rem(5, 2)
    EventCeiling, // ceil(expr) | ceil(2.26)
    EventFloor, // floor(expr) | floor(2.26)
    EventInteger, // integer(expr) | integer(2.26)
    Sine, // sin(expr) | sin(2.26)
    Cosine, // cos(expr) | cos(2.26)
    Tangent, // tan(expr) | tan(2.26)
    InverseSine, // asin(expr) | asin(2.26)
    InverseCosine, // acos(expr) | acos(2.26)
    InverseTangent, // atan(expr) | atan(2.26)
    PrincipalValueInverseTangent, // atan2(expr, expr) | atan2(2.26, 3.14)
    HyperbolicSine, // sinh(expr) | sinh(2.26)
    HyperbolicCosine, // cosh(expr) | cosh(2.26)
    HyperbolicTangent, // tanh(expr) | tanh(2.26)
    ExponentialBaseE, // exp(expr) | exp(2.26)
    NaturalBaseE, // log(expr) | log(2.26)
    LogBase10, // log10(expr) | log10(2.26)
    Derivative, // der(expr) | der(x)
    TimeDelay, // delay(expr, delayTime, <delayMax>) | delay(x, 2)
    Occurrences, // cardinality(c) | cardinality(?) ; DEPRECATED
    HomotopyInitialization, // homotopy(actual, simplified) | homotopy(?)
    SignDependentSlope, // semiLinear(x, k+, k-) | semiLinear(?)
    InStream, // inStream(expr) | inStream(x)
    ActualStream, // actualStream(expr) | actualStream(x)
    VariableSpeedTransport, //spatialDistribution(...) | spatialDistribution(?)
    InstanceName, // getInstanceName() | getInstanceName()
    InitialPredicate, // initial() | initial()
    TerminalPredicate, // terminal() | terminal()
    NoEvent, // noEvent(expr) | noEvent(x)
    Smooth, // smooth(p, expr) | smooth(3, x)
    Pre, // pre(expr) | pre(x)
    Edge, // edge(expr) | edge(x)
    Change, // change(expr) | change(x)
    ReInit, // reinit(x, expr) | reinit(x, 2)
}