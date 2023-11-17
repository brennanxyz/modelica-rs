#![allow(nonstandard_style)]
// Generated from modelica.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::modelicaparser::*;

pub trait modelicaListener<'input> : ParseTreeListener<'input,modelicaParserContextType>{
/**
 * Enter a parse tree produced by {@link modelicaParser#stored_definition}.
 * @param ctx the parse tree
 */
fn enter_stored_definition(&mut self, _ctx: &Stored_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#stored_definition}.
 * @param ctx the parse tree
 */
fn exit_stored_definition(&mut self, _ctx: &Stored_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#class_definition}.
 * @param ctx the parse tree
 */
fn enter_class_definition(&mut self, _ctx: &Class_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#class_definition}.
 * @param ctx the parse tree
 */
fn exit_class_definition(&mut self, _ctx: &Class_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#class_specifier}.
 * @param ctx the parse tree
 */
fn enter_class_specifier(&mut self, _ctx: &Class_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#class_specifier}.
 * @param ctx the parse tree
 */
fn exit_class_specifier(&mut self, _ctx: &Class_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#class_prefixes}.
 * @param ctx the parse tree
 */
fn enter_class_prefixes(&mut self, _ctx: &Class_prefixesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#class_prefixes}.
 * @param ctx the parse tree
 */
fn exit_class_prefixes(&mut self, _ctx: &Class_prefixesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#long_class_specifier}.
 * @param ctx the parse tree
 */
fn enter_long_class_specifier(&mut self, _ctx: &Long_class_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#long_class_specifier}.
 * @param ctx the parse tree
 */
fn exit_long_class_specifier(&mut self, _ctx: &Long_class_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#short_class_specifier}.
 * @param ctx the parse tree
 */
fn enter_short_class_specifier(&mut self, _ctx: &Short_class_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#short_class_specifier}.
 * @param ctx the parse tree
 */
fn exit_short_class_specifier(&mut self, _ctx: &Short_class_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#der_class_specifier}.
 * @param ctx the parse tree
 */
fn enter_der_class_specifier(&mut self, _ctx: &Der_class_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#der_class_specifier}.
 * @param ctx the parse tree
 */
fn exit_der_class_specifier(&mut self, _ctx: &Der_class_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#base_prefix}.
 * @param ctx the parse tree
 */
fn enter_base_prefix(&mut self, _ctx: &Base_prefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#base_prefix}.
 * @param ctx the parse tree
 */
fn exit_base_prefix(&mut self, _ctx: &Base_prefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#enum_list}.
 * @param ctx the parse tree
 */
fn enter_enum_list(&mut self, _ctx: &Enum_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#enum_list}.
 * @param ctx the parse tree
 */
fn exit_enum_list(&mut self, _ctx: &Enum_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#enumeration_literal}.
 * @param ctx the parse tree
 */
fn enter_enumeration_literal(&mut self, _ctx: &Enumeration_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#enumeration_literal}.
 * @param ctx the parse tree
 */
fn exit_enumeration_literal(&mut self, _ctx: &Enumeration_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#composition}.
 * @param ctx the parse tree
 */
fn enter_composition(&mut self, _ctx: &CompositionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#composition}.
 * @param ctx the parse tree
 */
fn exit_composition(&mut self, _ctx: &CompositionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#language_specification}.
 * @param ctx the parse tree
 */
fn enter_language_specification(&mut self, _ctx: &Language_specificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#language_specification}.
 * @param ctx the parse tree
 */
fn exit_language_specification(&mut self, _ctx: &Language_specificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#external_function_call}.
 * @param ctx the parse tree
 */
fn enter_external_function_call(&mut self, _ctx: &External_function_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#external_function_call}.
 * @param ctx the parse tree
 */
fn exit_external_function_call(&mut self, _ctx: &External_function_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element_list}.
 * @param ctx the parse tree
 */
fn enter_element_list(&mut self, _ctx: &Element_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element_list}.
 * @param ctx the parse tree
 */
fn exit_element_list(&mut self, _ctx: &Element_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element}.
 * @param ctx the parse tree
 */
fn enter_element(&mut self, _ctx: &ElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element}.
 * @param ctx the parse tree
 */
fn exit_element(&mut self, _ctx: &ElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#import_clause}.
 * @param ctx the parse tree
 */
fn enter_import_clause(&mut self, _ctx: &Import_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#import_clause}.
 * @param ctx the parse tree
 */
fn exit_import_clause(&mut self, _ctx: &Import_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#import_list}.
 * @param ctx the parse tree
 */
fn enter_import_list(&mut self, _ctx: &Import_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#import_list}.
 * @param ctx the parse tree
 */
fn exit_import_list(&mut self, _ctx: &Import_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#extends_clause}.
 * @param ctx the parse tree
 */
fn enter_extends_clause(&mut self, _ctx: &Extends_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#extends_clause}.
 * @param ctx the parse tree
 */
fn exit_extends_clause(&mut self, _ctx: &Extends_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#constraining_clause}.
 * @param ctx the parse tree
 */
fn enter_constraining_clause(&mut self, _ctx: &Constraining_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#constraining_clause}.
 * @param ctx the parse tree
 */
fn exit_constraining_clause(&mut self, _ctx: &Constraining_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_clause}.
 * @param ctx the parse tree
 */
fn enter_component_clause(&mut self, _ctx: &Component_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_clause}.
 * @param ctx the parse tree
 */
fn exit_component_clause(&mut self, _ctx: &Component_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#type_prefix}.
 * @param ctx the parse tree
 */
fn enter_type_prefix(&mut self, _ctx: &Type_prefixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#type_prefix}.
 * @param ctx the parse tree
 */
fn exit_type_prefix(&mut self, _ctx: &Type_prefixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#type_specifier}.
 * @param ctx the parse tree
 */
fn enter_type_specifier(&mut self, _ctx: &Type_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#type_specifier}.
 * @param ctx the parse tree
 */
fn exit_type_specifier(&mut self, _ctx: &Type_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_list}.
 * @param ctx the parse tree
 */
fn enter_component_list(&mut self, _ctx: &Component_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_list}.
 * @param ctx the parse tree
 */
fn exit_component_list(&mut self, _ctx: &Component_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_declaration}.
 * @param ctx the parse tree
 */
fn enter_component_declaration(&mut self, _ctx: &Component_declarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_declaration}.
 * @param ctx the parse tree
 */
fn exit_component_declaration(&mut self, _ctx: &Component_declarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#condition_attribute}.
 * @param ctx the parse tree
 */
fn enter_condition_attribute(&mut self, _ctx: &Condition_attributeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#condition_attribute}.
 * @param ctx the parse tree
 */
fn exit_condition_attribute(&mut self, _ctx: &Condition_attributeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#declaration}.
 * @param ctx the parse tree
 */
fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#declaration}.
 * @param ctx the parse tree
 */
fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#modification}.
 * @param ctx the parse tree
 */
fn enter_modification(&mut self, _ctx: &ModificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#modification}.
 * @param ctx the parse tree
 */
fn exit_modification(&mut self, _ctx: &ModificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#class_modification}.
 * @param ctx the parse tree
 */
fn enter_class_modification(&mut self, _ctx: &Class_modificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#class_modification}.
 * @param ctx the parse tree
 */
fn exit_class_modification(&mut self, _ctx: &Class_modificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#argument_list}.
 * @param ctx the parse tree
 */
fn enter_argument_list(&mut self, _ctx: &Argument_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#argument_list}.
 * @param ctx the parse tree
 */
fn exit_argument_list(&mut self, _ctx: &Argument_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#argument}.
 * @param ctx the parse tree
 */
fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#argument}.
 * @param ctx the parse tree
 */
fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element_modification_or_replaceable}.
 * @param ctx the parse tree
 */
fn enter_element_modification_or_replaceable(&mut self, _ctx: &Element_modification_or_replaceableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element_modification_or_replaceable}.
 * @param ctx the parse tree
 */
fn exit_element_modification_or_replaceable(&mut self, _ctx: &Element_modification_or_replaceableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element_modification}.
 * @param ctx the parse tree
 */
fn enter_element_modification(&mut self, _ctx: &Element_modificationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element_modification}.
 * @param ctx the parse tree
 */
fn exit_element_modification(&mut self, _ctx: &Element_modificationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element_redeclaration}.
 * @param ctx the parse tree
 */
fn enter_element_redeclaration(&mut self, _ctx: &Element_redeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element_redeclaration}.
 * @param ctx the parse tree
 */
fn exit_element_redeclaration(&mut self, _ctx: &Element_redeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#element_replaceable}.
 * @param ctx the parse tree
 */
fn enter_element_replaceable(&mut self, _ctx: &Element_replaceableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#element_replaceable}.
 * @param ctx the parse tree
 */
fn exit_element_replaceable(&mut self, _ctx: &Element_replaceableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_clause1}.
 * @param ctx the parse tree
 */
fn enter_component_clause1(&mut self, _ctx: &Component_clause1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_clause1}.
 * @param ctx the parse tree
 */
fn exit_component_clause1(&mut self, _ctx: &Component_clause1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_declaration1}.
 * @param ctx the parse tree
 */
fn enter_component_declaration1(&mut self, _ctx: &Component_declaration1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_declaration1}.
 * @param ctx the parse tree
 */
fn exit_component_declaration1(&mut self, _ctx: &Component_declaration1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#short_class_definition}.
 * @param ctx the parse tree
 */
fn enter_short_class_definition(&mut self, _ctx: &Short_class_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#short_class_definition}.
 * @param ctx the parse tree
 */
fn exit_short_class_definition(&mut self, _ctx: &Short_class_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#equation_section}.
 * @param ctx the parse tree
 */
fn enter_equation_section(&mut self, _ctx: &Equation_sectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#equation_section}.
 * @param ctx the parse tree
 */
fn exit_equation_section(&mut self, _ctx: &Equation_sectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#algorithm_section}.
 * @param ctx the parse tree
 */
fn enter_algorithm_section(&mut self, _ctx: &Algorithm_sectionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#algorithm_section}.
 * @param ctx the parse tree
 */
fn exit_algorithm_section(&mut self, _ctx: &Algorithm_sectionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#equation}.
 * @param ctx the parse tree
 */
fn enter_equation(&mut self, _ctx: &EquationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#equation}.
 * @param ctx the parse tree
 */
fn exit_equation(&mut self, _ctx: &EquationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#if_equation}.
 * @param ctx the parse tree
 */
fn enter_if_equation(&mut self, _ctx: &If_equationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#if_equation}.
 * @param ctx the parse tree
 */
fn exit_if_equation(&mut self, _ctx: &If_equationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#for_equation}.
 * @param ctx the parse tree
 */
fn enter_for_equation(&mut self, _ctx: &For_equationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#for_equation}.
 * @param ctx the parse tree
 */
fn exit_for_equation(&mut self, _ctx: &For_equationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#for_statement}.
 * @param ctx the parse tree
 */
fn enter_for_statement(&mut self, _ctx: &For_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#for_statement}.
 * @param ctx the parse tree
 */
fn exit_for_statement(&mut self, _ctx: &For_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#for_indices}.
 * @param ctx the parse tree
 */
fn enter_for_indices(&mut self, _ctx: &For_indicesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#for_indices}.
 * @param ctx the parse tree
 */
fn exit_for_indices(&mut self, _ctx: &For_indicesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#for_index}.
 * @param ctx the parse tree
 */
fn enter_for_index(&mut self, _ctx: &For_indexContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#for_index}.
 * @param ctx the parse tree
 */
fn exit_for_index(&mut self, _ctx: &For_indexContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#while_statement}.
 * @param ctx the parse tree
 */
fn enter_while_statement(&mut self, _ctx: &While_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#while_statement}.
 * @param ctx the parse tree
 */
fn exit_while_statement(&mut self, _ctx: &While_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#when_equation}.
 * @param ctx the parse tree
 */
fn enter_when_equation(&mut self, _ctx: &When_equationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#when_equation}.
 * @param ctx the parse tree
 */
fn exit_when_equation(&mut self, _ctx: &When_equationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#when_statement}.
 * @param ctx the parse tree
 */
fn enter_when_statement(&mut self, _ctx: &When_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#when_statement}.
 * @param ctx the parse tree
 */
fn exit_when_statement(&mut self, _ctx: &When_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#connect_clause}.
 * @param ctx the parse tree
 */
fn enter_connect_clause(&mut self, _ctx: &Connect_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#connect_clause}.
 * @param ctx the parse tree
 */
fn exit_connect_clause(&mut self, _ctx: &Connect_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#simple_expression}.
 * @param ctx the parse tree
 */
fn enter_simple_expression(&mut self, _ctx: &Simple_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#simple_expression}.
 * @param ctx the parse tree
 */
fn exit_simple_expression(&mut self, _ctx: &Simple_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#logical_expression}.
 * @param ctx the parse tree
 */
fn enter_logical_expression(&mut self, _ctx: &Logical_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#logical_expression}.
 * @param ctx the parse tree
 */
fn exit_logical_expression(&mut self, _ctx: &Logical_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#logical_term}.
 * @param ctx the parse tree
 */
fn enter_logical_term(&mut self, _ctx: &Logical_termContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#logical_term}.
 * @param ctx the parse tree
 */
fn exit_logical_term(&mut self, _ctx: &Logical_termContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#logical_factor}.
 * @param ctx the parse tree
 */
fn enter_logical_factor(&mut self, _ctx: &Logical_factorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#logical_factor}.
 * @param ctx the parse tree
 */
fn exit_logical_factor(&mut self, _ctx: &Logical_factorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#rel_op}.
 * @param ctx the parse tree
 */
fn enter_rel_op(&mut self, _ctx: &Rel_opContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#rel_op}.
 * @param ctx the parse tree
 */
fn exit_rel_op(&mut self, _ctx: &Rel_opContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#arithmetic_expression}.
 * @param ctx the parse tree
 */
fn enter_arithmetic_expression(&mut self, _ctx: &Arithmetic_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#arithmetic_expression}.
 * @param ctx the parse tree
 */
fn exit_arithmetic_expression(&mut self, _ctx: &Arithmetic_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#add_op}.
 * @param ctx the parse tree
 */
fn enter_add_op(&mut self, _ctx: &Add_opContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#add_op}.
 * @param ctx the parse tree
 */
fn exit_add_op(&mut self, _ctx: &Add_opContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#mul_op}.
 * @param ctx the parse tree
 */
fn enter_mul_op(&mut self, _ctx: &Mul_opContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#mul_op}.
 * @param ctx the parse tree
 */
fn exit_mul_op(&mut self, _ctx: &Mul_opContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#factor}.
 * @param ctx the parse tree
 */
fn enter_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#factor}.
 * @param ctx the parse tree
 */
fn exit_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#primary}.
 * @param ctx the parse tree
 */
fn enter_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#primary}.
 * @param ctx the parse tree
 */
fn exit_primary(&mut self, _ctx: &PrimaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#component_reference}.
 * @param ctx the parse tree
 */
fn enter_component_reference(&mut self, _ctx: &Component_referenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#component_reference}.
 * @param ctx the parse tree
 */
fn exit_component_reference(&mut self, _ctx: &Component_referenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#function_call_args}.
 * @param ctx the parse tree
 */
fn enter_function_call_args(&mut self, _ctx: &Function_call_argsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#function_call_args}.
 * @param ctx the parse tree
 */
fn exit_function_call_args(&mut self, _ctx: &Function_call_argsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#function_arguments}.
 * @param ctx the parse tree
 */
fn enter_function_arguments(&mut self, _ctx: &Function_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#function_arguments}.
 * @param ctx the parse tree
 */
fn exit_function_arguments(&mut self, _ctx: &Function_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#named_arguments}.
 * @param ctx the parse tree
 */
fn enter_named_arguments(&mut self, _ctx: &Named_argumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#named_arguments}.
 * @param ctx the parse tree
 */
fn exit_named_arguments(&mut self, _ctx: &Named_argumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#named_argument}.
 * @param ctx the parse tree
 */
fn enter_named_argument(&mut self, _ctx: &Named_argumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#named_argument}.
 * @param ctx the parse tree
 */
fn exit_named_argument(&mut self, _ctx: &Named_argumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#function_argument}.
 * @param ctx the parse tree
 */
fn enter_function_argument(&mut self, _ctx: &Function_argumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#function_argument}.
 * @param ctx the parse tree
 */
fn exit_function_argument(&mut self, _ctx: &Function_argumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#output_expression_list}.
 * @param ctx the parse tree
 */
fn enter_output_expression_list(&mut self, _ctx: &Output_expression_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#output_expression_list}.
 * @param ctx the parse tree
 */
fn exit_output_expression_list(&mut self, _ctx: &Output_expression_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#expression_list}.
 * @param ctx the parse tree
 */
fn enter_expression_list(&mut self, _ctx: &Expression_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#expression_list}.
 * @param ctx the parse tree
 */
fn exit_expression_list(&mut self, _ctx: &Expression_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#array_subscripts}.
 * @param ctx the parse tree
 */
fn enter_array_subscripts(&mut self, _ctx: &Array_subscriptsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#array_subscripts}.
 * @param ctx the parse tree
 */
fn exit_array_subscripts(&mut self, _ctx: &Array_subscriptsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#subscript_}.
 * @param ctx the parse tree
 */
fn enter_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#subscript_}.
 * @param ctx the parse tree
 */
fn exit_subscript_(&mut self, _ctx: &Subscript_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#comment}.
 * @param ctx the parse tree
 */
fn enter_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#comment}.
 * @param ctx the parse tree
 */
fn exit_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#string_comment}.
 * @param ctx the parse tree
 */
fn enter_string_comment(&mut self, _ctx: &String_commentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#string_comment}.
 * @param ctx the parse tree
 */
fn exit_string_comment(&mut self, _ctx: &String_commentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link modelicaParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link modelicaParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : modelicaListener<'input> }


