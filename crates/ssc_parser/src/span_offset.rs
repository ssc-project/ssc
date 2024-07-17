#![allow(clippy::pedantic)]

use miette::SourceSpan;
use oxc_ast::{ast::*, visit::walk_mut::*, VisitMut};
use oxc_diagnostics::{LabeledSpan, OxcDiagnostic};
use oxc_span::Span;
use oxc_syntax::scope::ScopeFlags;

#[derive(Debug, Clone, Copy)]
pub struct SpanOffset(pub u32);

impl SpanOffset {
    pub fn transform_diagnostic(self, diagnostic: OxcDiagnostic) -> OxcDiagnostic {
        let severity = diagnostic.severity;
        let help = diagnostic.help.clone();
        let labels = diagnostic.labels.clone();
        let message = diagnostic.message.clone();

        let diagnostic = OxcDiagnostic::error(message).with_severity(severity);
        let diagnostic =
            if let Some(help) = help { diagnostic.with_help(help) } else { diagnostic };

        if let Some(labels) = labels {
            let labels_with_offset: Vec<LabeledSpan> =
                labels.into_iter().map(|span| self.transform_labeled_span(span)).collect();

            diagnostic.with_labels(labels_with_offset)
        } else {
            diagnostic
        }
    }

    pub fn transform_labeled_span(self, span: LabeledSpan) -> LabeledSpan {
        let label = span.label().map(ToString::to_string);
        let primary = span.primary();
        let source_span = self.transform_source_span(*span.inner());
        if primary {
            LabeledSpan::new_primary_with_span(label, source_span)
        } else {
            LabeledSpan::new_with_span(label, source_span)
        }
    }

    pub fn transform_source_span(self, source_span: SourceSpan) -> SourceSpan {
        SourceSpan::new((source_span.offset() + self.0 as usize).into(), source_span.len())
    }
}

impl<'a> VisitMut<'a> for SpanOffset {
    fn visit_program(&mut self, it: &mut Program<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_program(self, it);
    }

    fn visit_directive(&mut self, it: &mut Directive<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_directive(self, it);
    }

    fn visit_string_literal(&mut self, it: &mut StringLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_string_literal(self, it);
    }

    fn visit_hashbang(&mut self, it: &mut Hashbang<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_hashbang(self, it);
    }

    fn visit_block_statement(&mut self, it: &mut BlockStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_block_statement(self, it);
    }

    fn visit_break_statement(&mut self, it: &mut BreakStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_break_statement(self, it);
    }

    fn visit_label_identifier(&mut self, it: &mut LabelIdentifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_label_identifier(self, it);
    }

    fn visit_continue_statement(&mut self, it: &mut ContinueStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_continue_statement(self, it);
    }

    fn visit_debugger_statement(&mut self, it: &mut DebuggerStatement) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_debugger_statement(self, it);
    }

    fn visit_do_while_statement(&mut self, it: &mut DoWhileStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_do_while_statement(self, it);
    }

    fn visit_boolean_literal(&mut self, it: &mut BooleanLiteral) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_boolean_literal(self, it);
    }

    fn visit_null_literal(&mut self, it: &mut NullLiteral) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_null_literal(self, it);
    }

    fn visit_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_numeric_literal(self, it);
    }

    fn visit_big_int_literal(&mut self, it: &mut BigIntLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_big_int_literal(self, it);
    }

    fn visit_reg_exp_literal(&mut self, it: &mut RegExpLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_reg_exp_literal(self, it);
    }

    fn visit_template_literal(&mut self, it: &mut TemplateLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_template_literal(self, it);
    }

    fn visit_template_element(&mut self, it: &mut TemplateElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_template_element(self, it);
    }

    fn visit_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_identifier_reference(self, it);
    }

    fn visit_meta_property(&mut self, it: &mut MetaProperty<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_meta_property(self, it);
    }

    fn visit_identifier_name(&mut self, it: &mut IdentifierName<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_identifier_name(self, it);
    }

    fn visit_super(&mut self, it: &mut Super) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_super(self, it);
    }

    fn visit_array_expression(&mut self, it: &mut ArrayExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_array_expression(self, it);
    }

    fn visit_spread_element(&mut self, it: &mut SpreadElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_spread_element(self, it);
    }

    fn visit_elision(&mut self, it: &mut Elision) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_elision(self, it);
    }

    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_arrow_function_expression(self, it);
    }

    fn visit_formal_parameters(&mut self, it: &mut FormalParameters<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_formal_parameters(self, it);
    }

    fn visit_formal_parameter(&mut self, it: &mut FormalParameter<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_formal_parameter(self, it);
    }

    fn visit_decorator(&mut self, it: &mut Decorator<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_decorator(self, it);
    }

    fn visit_binding_identifier(&mut self, it: &mut BindingIdentifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_binding_identifier(self, it);
    }

    fn visit_object_pattern(&mut self, it: &mut ObjectPattern<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_object_pattern(self, it);
    }

    fn visit_binding_property(&mut self, it: &mut BindingProperty<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_binding_property(self, it);
    }

    fn visit_private_identifier(&mut self, it: &mut PrivateIdentifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_private_identifier(self, it);
    }

    fn visit_binding_rest_element(&mut self, it: &mut BindingRestElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_binding_rest_element(self, it);
    }

    fn visit_array_pattern(&mut self, it: &mut ArrayPattern<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_array_pattern(self, it);
    }

    fn visit_assignment_pattern(&mut self, it: &mut AssignmentPattern<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_pattern(self, it);
    }

    fn visit_ts_type_annotation(&mut self, it: &mut TSTypeAnnotation<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_annotation(self, it);
    }

    fn visit_ts_any_keyword(&mut self, it: &mut TSAnyKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_any_keyword(self, it);
    }

    fn visit_ts_big_int_keyword(&mut self, it: &mut TSBigIntKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_big_int_keyword(self, it);
    }

    fn visit_ts_boolean_keyword(&mut self, it: &mut TSBooleanKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_boolean_keyword(self, it);
    }

    fn visit_ts_intrinsic_keyword(&mut self, it: &mut TSIntrinsicKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_intrinsic_keyword(self, it);
    }

    fn visit_ts_never_keyword(&mut self, it: &mut TSNeverKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_never_keyword(self, it);
    }

    fn visit_ts_null_keyword(&mut self, it: &mut TSNullKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_null_keyword(self, it);
    }

    fn visit_ts_number_keyword(&mut self, it: &mut TSNumberKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_number_keyword(self, it);
    }

    fn visit_ts_object_keyword(&mut self, it: &mut TSObjectKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_object_keyword(self, it);
    }

    fn visit_ts_string_keyword(&mut self, it: &mut TSStringKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_string_keyword(self, it);
    }

    fn visit_ts_symbol_keyword(&mut self, it: &mut TSSymbolKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_symbol_keyword(self, it);
    }

    fn visit_ts_undefined_keyword(&mut self, it: &mut TSUndefinedKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_undefined_keyword(self, it);
    }

    fn visit_ts_unknown_keyword(&mut self, it: &mut TSUnknownKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_unknown_keyword(self, it);
    }

    fn visit_ts_void_keyword(&mut self, it: &mut TSVoidKeyword) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_void_keyword(self, it);
    }

    fn visit_ts_array_type(&mut self, it: &mut TSArrayType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_array_type(self, it);
    }

    fn visit_ts_conditional_type(&mut self, it: &mut TSConditionalType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_conditional_type(self, it);
    }

    fn visit_ts_constructor_type(&mut self, it: &mut TSConstructorType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_constructor_type(self, it);
    }

    fn visit_ts_type_parameter_declaration(&mut self, it: &mut TSTypeParameterDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_parameter_declaration(self, it);
    }

    fn visit_ts_type_parameter(&mut self, it: &mut TSTypeParameter<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_parameter(self, it);
    }

    fn visit_ts_function_type(&mut self, it: &mut TSFunctionType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_function_type(self, it);
    }

    fn visit_ts_this_parameter(&mut self, it: &mut TSThisParameter<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_this_parameter(self, it);
    }

    fn visit_ts_import_type(&mut self, it: &mut TSImportType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_import_type(self, it);
    }

    fn visit_ts_qualified_name(&mut self, it: &mut TSQualifiedName<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_qualified_name(self, it);
    }

    fn visit_ts_import_attributes(&mut self, it: &mut TSImportAttributes<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_import_attributes(self, it);
    }

    fn visit_ts_import_attribute(&mut self, it: &mut TSImportAttribute<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_import_attribute(self, it);
    }

    fn visit_ts_type_parameter_instantiation(&mut self, it: &mut TSTypeParameterInstantiation<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_parameter_instantiation(self, it);
    }

    fn visit_ts_indexed_access_type(&mut self, it: &mut TSIndexedAccessType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_indexed_access_type(self, it);
    }

    fn visit_ts_infer_type(&mut self, it: &mut TSInferType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_infer_type(self, it);
    }

    fn visit_ts_intersection_type(&mut self, it: &mut TSIntersectionType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_intersection_type(self, it);
    }

    fn visit_ts_literal_type(&mut self, it: &mut TSLiteralType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_literal_type(self, it);
    }

    fn visit_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_unary_expression(self, it);
    }

    fn visit_ts_mapped_type(&mut self, it: &mut TSMappedType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_mapped_type(self, it);
    }

    fn visit_ts_named_tuple_member(&mut self, it: &mut TSNamedTupleMember<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_named_tuple_member(self, it);
    }

    fn visit_ts_optional_type(&mut self, it: &mut TSOptionalType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_optional_type(self, it);
    }

    fn visit_ts_rest_type(&mut self, it: &mut TSRestType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_rest_type(self, it);
    }

    fn visit_ts_template_literal_type(&mut self, it: &mut TSTemplateLiteralType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_template_literal_type(self, it);
    }

    fn visit_ts_this_type(&mut self, it: &mut TSThisType) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_this_type(self, it);
    }

    fn visit_ts_tuple_type(&mut self, it: &mut TSTupleType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_tuple_type(self, it);
    }

    fn visit_ts_type_literal(&mut self, it: &mut TSTypeLiteral<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_literal(self, it);
    }

    fn visit_ts_index_signature(&mut self, it: &mut TSIndexSignature<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_index_signature(self, it);
    }

    fn visit_ts_index_signature_name(&mut self, it: &mut TSIndexSignatureName<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_index_signature_name(self, it);
    }

    fn visit_ts_property_signature(&mut self, it: &mut TSPropertySignature<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_property_signature(self, it);
    }

    fn visit_ts_call_signature_declaration(&mut self, it: &mut TSCallSignatureDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_call_signature_declaration(self, it);
    }

    fn visit_ts_construct_signature_declaration(
        &mut self,
        it: &mut TSConstructSignatureDeclaration<'a>,
    ) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_construct_signature_declaration(self, it);
    }

    fn visit_ts_method_signature(&mut self, it: &mut TSMethodSignature<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_method_signature(self, it);
    }

    fn visit_ts_type_operator(&mut self, it: &mut TSTypeOperator<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_operator(self, it);
    }

    fn visit_ts_type_predicate(&mut self, it: &mut TSTypePredicate<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_predicate(self, it);
    }

    fn visit_ts_type_query(&mut self, it: &mut TSTypeQuery<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_query(self, it);
    }

    fn visit_ts_type_reference(&mut self, it: &mut TSTypeReference<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_reference(self, it);
    }

    fn visit_ts_union_type(&mut self, it: &mut TSUnionType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_union_type(self, it);
    }

    fn visit_ts_parenthesized_type(&mut self, it: &mut TSParenthesizedType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_parenthesized_type(self, it);
    }

    fn visit_js_doc_nullable_type(&mut self, it: &mut JSDocNullableType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_js_doc_nullable_type(self, it);
    }

    fn visit_js_doc_non_nullable_type(&mut self, it: &mut JSDocNonNullableType<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_js_doc_non_nullable_type(self, it);
    }

    fn visit_js_doc_unknown_type(&mut self, it: &mut JSDocUnknownType) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_js_doc_unknown_type(self, it);
    }

    fn visit_function_body(&mut self, it: &mut FunctionBody<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_function_body(self, it);
    }

    fn visit_assignment_expression(&mut self, it: &mut AssignmentExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_expression(self, it);
    }

    fn visit_ts_as_expression(&mut self, it: &mut TSAsExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_as_expression(self, it);
    }

    fn visit_ts_satisfies_expression(&mut self, it: &mut TSSatisfiesExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_satisfies_expression(self, it);
    }

    fn visit_ts_non_null_expression(&mut self, it: &mut TSNonNullExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_non_null_expression(self, it);
    }

    fn visit_ts_type_assertion(&mut self, it: &mut TSTypeAssertion<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_assertion(self, it);
    }

    fn visit_ts_instantiation_expression(&mut self, it: &mut TSInstantiationExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_instantiation_expression(self, it);
    }

    fn visit_computed_member_expression(&mut self, it: &mut ComputedMemberExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_computed_member_expression(self, it);
    }

    fn visit_static_member_expression(&mut self, it: &mut StaticMemberExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_static_member_expression(self, it);
    }

    fn visit_private_field_expression(&mut self, it: &mut PrivateFieldExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_private_field_expression(self, it);
    }

    fn visit_array_assignment_target(&mut self, it: &mut ArrayAssignmentTarget<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_array_assignment_target(self, it);
    }

    fn visit_assignment_target_with_default(&mut self, it: &mut AssignmentTargetWithDefault<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_target_with_default(self, it);
    }

    fn visit_assignment_target_rest(&mut self, it: &mut AssignmentTargetRest<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_target_rest(self, it);
    }

    fn visit_object_assignment_target(&mut self, it: &mut ObjectAssignmentTarget<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_object_assignment_target(self, it);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &mut AssignmentTargetPropertyIdentifier<'a>,
    ) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_target_property_identifier(self, it);
    }

    fn visit_assignment_target_property_property(
        &mut self,
        it: &mut AssignmentTargetPropertyProperty<'a>,
    ) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_assignment_target_property_property(self, it);
    }

    fn visit_await_expression(&mut self, it: &mut AwaitExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_await_expression(self, it);
    }

    fn visit_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_binary_expression(self, it);
    }

    fn visit_call_expression(&mut self, it: &mut CallExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_call_expression(self, it);
    }

    fn visit_chain_expression(&mut self, it: &mut ChainExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_chain_expression(self, it);
    }

    fn visit_class(&mut self, it: &mut Class<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_class(self, it);
    }

    fn visit_class_body(&mut self, it: &mut ClassBody<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_class_body(self, it);
    }

    fn visit_static_block(&mut self, it: &mut StaticBlock<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_static_block(self, it);
    }

    fn visit_method_definition(&mut self, it: &mut MethodDefinition<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_method_definition(self, it);
    }

    fn visit_function(&mut self, it: &mut Function<'a>, flags: Option<ScopeFlags>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_function(self, it, flags);
    }

    fn visit_property_definition(&mut self, it: &mut PropertyDefinition<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_property_definition(self, it);
    }

    fn visit_accessor_property(&mut self, it: &mut AccessorProperty<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_accessor_property(self, it);
    }

    fn visit_ts_class_implements(&mut self, it: &mut TSClassImplements<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_class_implements(self, it);
    }

    fn visit_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_conditional_expression(self, it);
    }

    fn visit_import_expression(&mut self, it: &mut ImportExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_expression(self, it);
    }

    fn visit_logical_expression(&mut self, it: &mut LogicalExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_logical_expression(self, it);
    }

    fn visit_new_expression(&mut self, it: &mut NewExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_new_expression(self, it);
    }

    fn visit_object_expression(&mut self, it: &mut ObjectExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_object_expression(self, it);
    }

    fn visit_object_property(&mut self, it: &mut ObjectProperty<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_object_property(self, it);
    }

    fn visit_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_parenthesized_expression(self, it);
    }

    fn visit_sequence_expression(&mut self, it: &mut SequenceExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_sequence_expression(self, it);
    }

    fn visit_tagged_template_expression(&mut self, it: &mut TaggedTemplateExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_tagged_template_expression(self, it);
    }

    fn visit_this_expression(&mut self, it: &mut ThisExpression) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_this_expression(self, it);
    }

    fn visit_update_expression(&mut self, it: &mut UpdateExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_update_expression(self, it);
    }

    fn visit_yield_expression(&mut self, it: &mut YieldExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_yield_expression(self, it);
    }

    fn visit_private_in_expression(&mut self, it: &mut PrivateInExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_private_in_expression(self, it);
    }

    fn visit_jsx_element(&mut self, it: &mut JSXElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_element(self, it);
    }

    fn visit_jsx_opening_element(&mut self, it: &mut JSXOpeningElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_opening_element(self, it);
    }

    fn visit_jsx_identifier(&mut self, it: &mut JSXIdentifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_identifier(self, it);
    }

    fn visit_jsx_namespaced_name(&mut self, it: &mut JSXNamespacedName<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_namespaced_name(self, it);
    }

    fn visit_jsx_member_expression(&mut self, it: &mut JSXMemberExpression<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_member_expression(self, it);
    }

    fn visit_jsx_attribute(&mut self, it: &mut JSXAttribute<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_attribute(self, it);
    }

    fn visit_jsx_expression_container(&mut self, it: &mut JSXExpressionContainer<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_expression_container(self, it);
    }

    fn visit_jsx_empty_expression(&mut self, it: &mut JSXEmptyExpression) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_empty_expression(self, it);
    }

    fn visit_jsx_fragment(&mut self, it: &mut JSXFragment<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_fragment(self, it);
    }

    fn visit_jsx_text(&mut self, it: &mut JSXText<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_text(self, it);
    }

    fn visit_jsx_spread_child(&mut self, it: &mut JSXSpreadChild<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_spread_child(self, it);
    }

    fn visit_jsx_spread_attribute(&mut self, it: &mut JSXSpreadAttribute<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_spread_attribute(self, it);
    }

    fn visit_jsx_closing_element(&mut self, it: &mut JSXClosingElement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_jsx_closing_element(self, it);
    }

    fn visit_empty_statement(&mut self, it: &mut EmptyStatement) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_empty_statement(self, it);
    }

    fn visit_expression_statement(&mut self, it: &mut ExpressionStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_expression_statement(self, it);
    }

    fn visit_for_in_statement(&mut self, it: &mut ForInStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_for_in_statement(self, it);
    }

    fn visit_variable_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_variable_declaration(self, it);
    }

    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_variable_declarator(self, it);
    }

    fn visit_using_declaration(&mut self, it: &mut UsingDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_using_declaration(self, it);
    }

    fn visit_for_of_statement(&mut self, it: &mut ForOfStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_for_of_statement(self, it);
    }

    fn visit_for_statement(&mut self, it: &mut ForStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_for_statement(self, it);
    }

    fn visit_if_statement(&mut self, it: &mut IfStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_if_statement(self, it);
    }

    fn visit_labeled_statement(&mut self, it: &mut LabeledStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_labeled_statement(self, it);
    }

    fn visit_return_statement(&mut self, it: &mut ReturnStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_return_statement(self, it);
    }

    fn visit_switch_statement(&mut self, it: &mut SwitchStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_switch_statement(self, it);
    }

    fn visit_switch_case(&mut self, it: &mut SwitchCase<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_switch_case(self, it);
    }

    fn visit_throw_statement(&mut self, it: &mut ThrowStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_throw_statement(self, it);
    }

    fn visit_try_statement(&mut self, it: &mut TryStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_try_statement(self, it);
    }

    fn visit_catch_clause(&mut self, it: &mut CatchClause<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_catch_clause(self, it);
    }

    fn visit_catch_parameter(&mut self, it: &mut CatchParameter<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_catch_parameter(self, it);
    }

    fn visit_finally_clause(&mut self, it: &mut BlockStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_finally_clause(self, it);
    }

    fn visit_while_statement(&mut self, it: &mut WhileStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_while_statement(self, it);
    }

    fn visit_with_statement(&mut self, it: &mut WithStatement<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_with_statement(self, it);
    }

    fn visit_ts_type_alias_declaration(&mut self, it: &mut TSTypeAliasDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_type_alias_declaration(self, it);
    }

    fn visit_ts_interface_declaration(&mut self, it: &mut TSInterfaceDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_interface_declaration(self, it);
    }

    fn visit_ts_interface_heritage(&mut self, it: &mut TSInterfaceHeritage<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_interface_heritage(self, it);
    }

    fn visit_ts_interface_body(&mut self, it: &mut TSInterfaceBody<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_interface_body(self, it);
    }

    fn visit_ts_enum_declaration(&mut self, it: &mut TSEnumDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_enum_declaration(self, it);
    }

    fn visit_ts_enum_member(&mut self, it: &mut TSEnumMember<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_enum_member(self, it);
    }

    fn visit_ts_module_declaration(&mut self, it: &mut TSModuleDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_module_declaration(self, it);
    }

    fn visit_ts_module_block(&mut self, it: &mut TSModuleBlock<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_module_block(self, it);
    }

    fn visit_ts_import_equals_declaration(&mut self, it: &mut TSImportEqualsDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_import_equals_declaration(self, it);
    }

    fn visit_ts_external_module_reference(&mut self, it: &mut TSExternalModuleReference<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_external_module_reference(self, it);
    }

    fn visit_import_declaration(&mut self, it: &mut ImportDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_declaration(self, it);
    }

    fn visit_import_specifier(&mut self, it: &mut ImportSpecifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_specifier(self, it);
    }

    fn visit_import_default_specifier(&mut self, it: &mut ImportDefaultSpecifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_default_specifier(self, it);
    }

    fn visit_import_namespace_specifier(&mut self, it: &mut ImportNamespaceSpecifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_namespace_specifier(self, it);
    }

    fn visit_with_clause(&mut self, it: &mut WithClause<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_with_clause(self, it);
    }

    fn visit_import_attribute(&mut self, it: &mut ImportAttribute<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_import_attribute(self, it);
    }

    fn visit_export_all_declaration(&mut self, it: &mut ExportAllDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_export_all_declaration(self, it);
    }

    fn visit_export_default_declaration(&mut self, it: &mut ExportDefaultDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_export_default_declaration(self, it);
    }

    fn visit_export_named_declaration(&mut self, it: &mut ExportNamedDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_export_named_declaration(self, it);
    }

    fn visit_export_specifier(&mut self, it: &mut ExportSpecifier<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_export_specifier(self, it);
    }

    fn visit_ts_export_assignment(&mut self, it: &mut TSExportAssignment<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_export_assignment(self, it);
    }

    fn visit_ts_namespace_export_declaration(&mut self, it: &mut TSNamespaceExportDeclaration<'a>) {
        it.span = Span::new(it.span.start + self.0, it.span.start + self.0);
        walk_ts_namespace_export_declaration(self, it);
    }
}
