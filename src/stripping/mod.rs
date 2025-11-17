// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use swc_ecma_lexer::token::TokenAndSpan;
use swc_ecma_visit::VisitWith;

use crate::TextChange;
use crate::swc::ast::*;
use crate::text_changes;

enum TypeStripError {
  UnsupportedDeclaration {},
}

pub fn type_strip(
  text: &str,
  program: &Program,
  tokens: &[TokenAndSpan],
) -> Result<String, TypeStripError> {
  let mut stripper = TypeStripper {
    changes: Vec::new(),
    tokens,
  };
  stripper.strip_program(program)?;
  Ok(text_changes::apply_text_changes(text, stripper.changes))
}

struct TypeStripper<'a> {
  changes: Vec<TextChange>,
  tokens: &'a [TokenAndSpan],
}

impl<'a> swc_ecma_visit::Visit for TypeStripper<'a> {
  fn visit_accessibility(&mut self, node: &Accessibility) {
    node.visit_children_with(self);
  }

  fn visit_array_lit(&mut self, node: &ArrayLit) {
    node.visit_children_with(self)
  }

  fn visit_array_pat(&mut self, node: &ArrayPat) {
    node.visit_children_with(self);
  }

  fn visit_arrow_expr(&mut self, node: &ArrowExpr) {
    node.visit_children_with(self);
  }

  fn visit_assign_expr(&mut self, node: &AssignExpr) {
    node.visit_children_with(self);
  }

  fn visit_assign_op(&mut self, node: &AssignOp) {
    node.visit_children_with(self);
  }

  fn visit_assign_pat(&mut self, node: &AssignPat) {
    node.visit_children_with(self);
  }

  fn visit_assign_pat_prop(&mut self, node: &AssignPatProp) {
    node.visit_children_with(self);
  }

  fn visit_assign_prop(&mut self, node: &AssignProp) {
    node.visit_children_with(self);
  }

  fn visit_assign_target(&mut self, node: &AssignTarget) {
    node.visit_children_with(self);
  }

  fn visit_assign_target_pat(&mut self, node: &AssignTargetPat) {
    node.visit_children_with(self);
  }

  fn visit_atom(&mut self, node: &swc_atoms::Atom) {
    node.visit_children_with(self);
  }

  fn visit_auto_accessor(&mut self, node: &AutoAccessor) {
    node.visit_children_with(self);
  }

  fn visit_await_expr(&mut self, node: &AwaitExpr) {
    node.visit_children_with(self);
  }

  fn visit_big_int(&mut self, node: &BigInt) {
    node.visit_children_with(self);
  }

  fn visit_big_int_value(&mut self, node: &BigIntValue) {
    node.visit_children_with(self);
  }

  fn visit_bin_expr(&mut self, node: &BinExpr) {
    node.visit_children_with(self);
  }

  fn visit_binary_op(&mut self, node: &BinaryOp) {
    node.visit_children_with(self);
  }

  fn visit_binding_ident(&mut self, node: &BindingIdent) {
    node.visit_children_with(self);
  }

  fn visit_block_stmt(&mut self, node: &BlockStmt) {
    node.visit_children_with(self);
  }

  fn visit_block_stmt_or_expr(&mut self, node: &BlockStmtOrExpr) {
    node.visit_children_with(self);
  }

  fn visit_bool(&mut self, node: &Bool) {
    node.visit_children_with(self);
  }

  fn visit_break_stmt(&mut self, node: &BreakStmt) {
    node.visit_children_with(self);
  }

  fn visit_call_expr(&mut self, node: &CallExpr) {
    node.visit_children_with(self);
  }

  fn visit_callee(&mut self, node: &Callee) {
    node.visit_children_with(self);
  }

  fn visit_catch_clause(&mut self, node: &CatchClause) {
    node.visit_children_with(self);
  }

  fn visit_class(&mut self, node: &Class) {
    node.visit_children_with(self);
  }

  fn visit_class_decl(&mut self, node: &ClassDecl) {
    node.visit_children_with(self);
  }

  fn visit_class_expr(&mut self, node: &ClassExpr) {
    node.visit_children_with(self);
  }

  fn visit_class_member(&mut self, node: &ClassMember) {
    node.visit_children_with(self);
  }

  fn visit_class_members(&mut self, node: &[ClassMember]) {
    node.visit_children_with(self);
  }

  fn visit_class_method(&mut self, node: &ClassMethod) {
    node.visit_children_with(self);
  }

  fn visit_class_prop(&mut self, node: &ClassProp) {
    node.visit_children_with(self);
  }

  fn visit_computed_prop_name(&mut self, node: &ComputedPropName) {
    node.visit_children_with(self);
  }

  fn visit_cond_expr(&mut self, node: &CondExpr) {
    node.visit_children_with(self);
  }

  fn visit_constructor(&mut self, node: &Constructor) {
    node.visit_children_with(self);
  }

  fn visit_continue_stmt(&mut self, node: &ContinueStmt) {
    node.visit_children_with(self);
  }

  fn visit_debugger_stmt(&mut self, node: &DebuggerStmt) {
    node.visit_children_with(self);
  }

  fn visit_decl(&mut self, node: &Decl) {
    node.visit_children_with(self);
  }

  fn visit_decorator(&mut self, node: &Decorator) {
    node.visit_children_with(self);
  }

  fn visit_decorators(&mut self, node: &[Decorator]) {
    node.visit_children_with(self);
  }

  fn visit_default_decl(&mut self, node: &DefaultDecl) {
    node.visit_children_with(self);
  }

  fn visit_do_while_stmt(&mut self, node: &DoWhileStmt) {
    node.visit_children_with(self);
  }

  fn visit_empty_stmt(&mut self, node: &EmptyStmt) {
    node.visit_children_with(self);
  }

  fn visit_export_all(&mut self, node: &ExportAll) {
    node.visit_children_with(self);
  }

  fn visit_export_decl(&mut self, node: &ExportDecl) {
    node.visit_children_with(self);
  }

  fn visit_export_default_decl(&mut self, node: &ExportDefaultDecl) {
    node.visit_children_with(self);
  }

  fn visit_export_default_expr(&mut self, node: &ExportDefaultExpr) {
    node.visit_children_with(self);
  }

  fn visit_export_default_specifier(&mut self, node: &ExportDefaultSpecifier) {
    node.visit_children_with(self);
  }

  fn visit_export_named_specifier(&mut self, node: &ExportNamedSpecifier) {
    node.visit_children_with(self);
  }

  fn visit_export_namespace_specifier(
    &mut self,
    node: &ExportNamespaceSpecifier,
  ) {
    node.visit_children_with(self);
  }

  fn visit_export_specifier(&mut self, node: &ExportSpecifier) {
    node.visit_children_with(self);
  }

  fn visit_export_specifiers(&mut self, node: &[ExportSpecifier]) {
    node.visit_children_with(self);
  }

  fn visit_expr(&mut self, node: &Expr) {
    VisitWith::visit_children_with(node, self)
  }

  fn visit_expr_or_spread(&mut self, node: &ExprOrSpread) {
    VisitWith::visit_children_with(node, self)
  }

  fn visit_expr_or_spreads(&mut self, node: &[ExprOrSpread]) {
    VisitWith::visit_children_with(node, self)
  }

  fn visit_expr_stmt(&mut self, node: &ExprStmt) {
    VisitWith::visit_children_with(node, self)
  }

  fn visit_exprs(&mut self, node: &[Box<Expr>]) {
    VisitWith::visit_children_with(node, self)
  }

  fn visit_fn_decl(&mut self, node: &FnDecl) {
    <FnDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_fn_expr(&mut self, node: &FnExpr) {
    <FnExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_for_head(&mut self, node: &ForHead) {
    <ForHead as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_for_in_stmt(&mut self, node: &ForInStmt) {
    <ForInStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_for_of_stmt(&mut self, node: &ForOfStmt) {
    <ForOfStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_for_stmt(&mut self, node: &ForStmt) {
    <ForStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_function(&mut self, node: &Function) {
    <Function as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_getter_prop(&mut self, node: &GetterProp) {
    <GetterProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ident(&mut self, node: &Ident) {
    <Ident as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ident_name(&mut self, node: &IdentName) {
    <IdentName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_if_stmt(&mut self, node: &IfStmt) {
    <IfStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_import(&mut self, node: &Import) {
    <Import as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_import_decl(&mut self, node: &ImportDecl) {
    <ImportDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_default_specifier(&mut self, node: &ImportDefaultSpecifier) {
    <ImportDefaultSpecifier as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_import_named_specifier(&mut self, node: &ImportNamedSpecifier) {
    <ImportNamedSpecifier as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_import_phase(&mut self, node: &ImportPhase) {
    <ImportPhase as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_specifier(&mut self, node: &ImportSpecifier) {
    <ImportSpecifier as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_specifiers(&mut self, node: &[ImportSpecifier]) {
    <[ImportSpecifier] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_star_as_specifier(&mut self, node: &ImportStarAsSpecifier) {
    <ImportStarAsSpecifier as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_import_with(&mut self, node: &ImportWith) {
    <ImportWith as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_with_item(&mut self, node: &ImportWithItem) {
    <ImportWithItem as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_import_with_items(&mut self, node: &[ImportWithItem]) {
    <[ImportWithItem] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_invalid(&mut self, node: &Invalid) {
    <Invalid as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_attr(&mut self, node: &JSXAttr) {
    <JSXAttr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_attr_name(&mut self, node: &JSXAttrName) {
    <JSXAttrName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_attr_or_spread(&mut self, node: &JSXAttrOrSpread) {
    <JSXAttrOrSpread as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_attr_or_spreads(&mut self, node: &[JSXAttrOrSpread]) {
    <[JSXAttrOrSpread] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_attr_value(&mut self, node: &JSXAttrValue) {
    <JSXAttrValue as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_closing_element(&mut self, node: &JSXClosingElement) {
    <JSXClosingElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_closing_fragment(&mut self, node: &JSXClosingFragment) {
    <JSXClosingFragment as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_element(&mut self, node: &JSXElement) {
    <JSXElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_element_child(&mut self, node: &JSXElementChild) {
    <JSXElementChild as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_element_childs(&mut self, node: &[JSXElementChild]) {
    <[JSXElementChild] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_element_name(&mut self, node: &JSXElementName) {
    <JSXElementName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_empty_expr(&mut self, node: &JSXEmptyExpr) {
    <JSXEmptyExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_expr(&mut self, node: &JSXExpr) {
    <JSXExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_expr_container(&mut self, node: &JSXExprContainer) {
    <JSXExprContainer as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_fragment(&mut self, node: &JSXFragment) {
    <JSXFragment as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_member_expr(&mut self, node: &JSXMemberExpr) {
    <JSXMemberExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_namespaced_name(&mut self, node: &JSXNamespacedName) {
    <JSXNamespacedName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_object(&mut self, node: &JSXObject) {
    <JSXObject as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_opening_element(&mut self, node: &JSXOpeningElement) {
    <JSXOpeningElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_opening_fragment(&mut self, node: &JSXOpeningFragment) {
    <JSXOpeningFragment as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_spread_child(&mut self, node: &JSXSpreadChild) {
    <JSXSpreadChild as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_jsx_text(&mut self, node: &JSXText) {
    <JSXText as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_key(&mut self, node: &Key) {
    <Key as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_key_value_pat_prop(&mut self, node: &KeyValuePatProp) {
    <KeyValuePatProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_key_value_prop(&mut self, node: &KeyValueProp) {
    <KeyValueProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_labeled_stmt(&mut self, node: &LabeledStmt) {
    <LabeledStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_lit(&mut self, node: &Lit) {
    <Lit as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_member_expr(&mut self, node: &MemberExpr) {
    <MemberExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_member_prop(&mut self, node: &MemberProp) {
    <MemberProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_meta_prop_expr(&mut self, node: &MetaPropExpr) {
    <MetaPropExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_meta_prop_kind(&mut self, node: &MetaPropKind) {
    <MetaPropKind as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_method_kind(&mut self, node: &MethodKind) {
    <MethodKind as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_method_prop(&mut self, node: &MethodProp) {
    <MethodProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_module(&mut self, node: &Module) {
    <Module as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_module_decl(&mut self, node: &ModuleDecl) {
    <ModuleDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_module_export_name(&mut self, node: &ModuleExportName) {
    <ModuleExportName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_module_item(&mut self, node: &ModuleItem) {
    <ModuleItem as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_module_items(&mut self, node: &[ModuleItem]) {
    <[ModuleItem] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_named_export(&mut self, node: &NamedExport) {
    <NamedExport as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_new_expr(&mut self, node: &NewExpr) {
    <NewExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_null(&mut self, node: &Null) {
    <Null as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_number(&mut self, node: &Number) {
    <Number as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_object_lit(&mut self, node: &ObjectLit) {
    <ObjectLit as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_object_pat(&mut self, node: &ObjectPat) {
    <ObjectPat as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_object_pat_prop(&mut self, node: &ObjectPatProp) {
    <ObjectPatProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_object_pat_props(&mut self, node: &[ObjectPatProp]) {
    <[ObjectPatProp] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_accessibility(&mut self, node: &Option<Accessibility>) {
    <Option<Accessibility> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_atom(&mut self, node: &Option<swc_atoms::Atom>) {
    <Option<swc_atoms::Atom> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_block_stmt(&mut self, node: &Option<BlockStmt>) {
    <Option<BlockStmt> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_call(&mut self, node: &OptCall) {
    <OptCall as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_catch_clause(&mut self, node: &Option<CatchClause>) {
    <Option<CatchClause> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_chain_base(&mut self, node: &OptChainBase) {
    <OptChainBase as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_chain_expr(&mut self, node: &OptChainExpr) {
    <OptChainExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_expr(&mut self, node: &Option<Box<Expr>>) {
    <Option<Box<Expr>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_expr_or_spread(&mut self, node: &Option<ExprOrSpread>) {
    <Option<ExprOrSpread> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_expr_or_spreads(&mut self, node: &Option<Vec<ExprOrSpread>>) {
    <Option<Vec<ExprOrSpread>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ident(&mut self, node: &Option<Ident>) {
    <Option<Ident> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_jsx_attr_value(&mut self, node: &Option<JSXAttrValue>) {
    <Option<JSXAttrValue> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_jsx_closing_element(
    &mut self,
    node: &Option<JSXClosingElement>,
  ) {
    <Option<JSXClosingElement> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_module_export_name(&mut self, node: &Option<ModuleExportName>) {
    <Option<ModuleExportName> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_object_lit(&mut self, node: &Option<Box<ObjectLit>>) {
    <Option<Box<ObjectLit>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_pat(&mut self, node: &Option<Pat>) {
    <Option<Pat> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_span(&mut self, node: &Option<swc_common::Span>) {
    <Option<swc_common::Span> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_stmt(&mut self, node: &Option<Box<Stmt>>) {
    <Option<Box<Stmt>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_str(&mut self, node: &Option<Box<Str>>) {
    <Option<Box<Str>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_opt_true_plus_minus(&mut self, node: &Option<TruePlusMinus>) {
    <Option<TruePlusMinus> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_entity_name(&mut self, node: &Option<TsEntityName>) {
    <Option<TsEntityName> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_import_call_options(
    &mut self,
    node: &Option<TsImportCallOptions>,
  ) {
    <Option<TsImportCallOptions> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_namespace_body(&mut self, node: &Option<TsNamespaceBody>) {
    <Option<TsNamespaceBody> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_type(&mut self, node: &Option<Box<TsType>>) {
    <Option<Box<TsType>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_type_ann(&mut self, node: &Option<Box<TsTypeAnn>>) {
    <Option<Box<TsTypeAnn>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_type_param_decl(
    &mut self,
    node: &Option<Box<TsTypeParamDecl>>,
  ) {
    <Option<Box<TsTypeParamDecl>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_ts_type_param_instantiation(
    &mut self,
    node: &Option<Box<TsTypeParamInstantiation>>,
  ) {
    <Option<Box<TsTypeParamInstantiation>> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_var_decl_or_expr(&mut self, node: &Option<VarDeclOrExpr>) {
    <Option<VarDeclOrExpr> as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_vec_expr_or_spreads(&mut self, node: &[Option<ExprOrSpread>]) {
    <[Option<ExprOrSpread>] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_opt_vec_pats(&mut self, node: &[Option<Pat>]) {
    <[Option<Pat>] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_param(&mut self, node: &Param) {
    <Param as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_param_or_ts_param_prop(&mut self, node: &ParamOrTsParamProp) {
    <ParamOrTsParamProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_param_or_ts_param_props(&mut self, node: &[ParamOrTsParamProp]) {
    <[ParamOrTsParamProp] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_params(&mut self, node: &[Param]) {
    <[Param] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_paren_expr(&mut self, node: &ParenExpr) {
    <ParenExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_pat(&mut self, node: &Pat) {
    <Pat as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_pats(&mut self, node: &[Pat]) {
    <[Pat] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_private_method(&mut self, node: &PrivateMethod) {
    <PrivateMethod as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_private_name(&mut self, node: &PrivateName) {
    <PrivateName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_private_prop(&mut self, node: &PrivateProp) {
    <PrivateProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_program(&mut self, node: &Program) {
    <Program as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_prop(&mut self, node: &Prop) {
    <Prop as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_prop_name(&mut self, node: &PropName) {
    <PropName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_prop_or_spread(&mut self, node: &PropOrSpread) {
    <PropOrSpread as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_prop_or_spreads(&mut self, node: &[PropOrSpread]) {
    <[PropOrSpread] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_regex(&mut self, node: &Regex) {
    <Regex as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_rest_pat(&mut self, node: &RestPat) {
    <RestPat as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_return_stmt(&mut self, node: &ReturnStmt) {
    <ReturnStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_script(&mut self, node: &Script) {
    <Script as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_seq_expr(&mut self, node: &SeqExpr) {
    <SeqExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_setter_prop(&mut self, node: &SetterProp) {
    <SetterProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_simple_assign_target(&mut self, node: &SimpleAssignTarget) {
    <SimpleAssignTarget as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_span(&mut self, node: &swc_common::Span) {
    <swc_common::Span as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_spread_element(&mut self, node: &SpreadElement) {
    <SpreadElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_static_block(&mut self, node: &StaticBlock) {
    <StaticBlock as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_stmt(&mut self, node: &Stmt) {
    <Stmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_stmts(&mut self, node: &[Stmt]) {
    <[Stmt] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_str(&mut self, node: &Str) {
    <Str as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_super(&mut self, node: &Super) {
    <Super as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_super_prop(&mut self, node: &SuperProp) {
    <SuperProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_super_prop_expr(&mut self, node: &SuperPropExpr) {
    <SuperPropExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_switch_case(&mut self, node: &SwitchCase) {
    <SwitchCase as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_switch_cases(&mut self, node: &[SwitchCase]) {
    <[SwitchCase] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_switch_stmt(&mut self, node: &SwitchStmt) {
    <SwitchStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_syntax_context(&mut self, node: &swc_common::SyntaxContext) {
    <swc_common::SyntaxContext as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_tagged_tpl(&mut self, node: &TaggedTpl) {
    <TaggedTpl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_this_expr(&mut self, node: &ThisExpr) {
    <ThisExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_throw_stmt(&mut self, node: &ThrowStmt) {
    <ThrowStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_tpl(&mut self, node: &Tpl) {
    <Tpl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_tpl_element(&mut self, node: &TplElement) {
    <TplElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_tpl_elements(&mut self, node: &[TplElement]) {
    <[TplElement] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_true_plus_minus(&mut self, node: &TruePlusMinus) {
    <TruePlusMinus as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_try_stmt(&mut self, node: &TryStmt) {
    <TryStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_array_type(&mut self, node: &TsArrayType) {
    <TsArrayType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_as_expr(&mut self, node: &TsAsExpr) {
    <TsAsExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_call_signature_decl(&mut self, node: &TsCallSignatureDecl) {
    <TsCallSignatureDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_conditional_type(&mut self, node: &TsConditionalType) {
    <TsConditionalType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_const_assertion(&mut self, node: &TsConstAssertion) {
    <TsConstAssertion as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_construct_signature_decl(
    &mut self,
    node: &TsConstructSignatureDecl,
  ) {
    <TsConstructSignatureDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_constructor_type(&mut self, node: &TsConstructorType) {
    <TsConstructorType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_entity_name(&mut self, node: &TsEntityName) {
    <TsEntityName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_enum_decl(&mut self, node: &TsEnumDecl) {
    <TsEnumDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_enum_member(&mut self, node: &TsEnumMember) {
    <TsEnumMember as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_enum_member_id(&mut self, node: &TsEnumMemberId) {
    <TsEnumMemberId as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_enum_members(&mut self, node: &[TsEnumMember]) {
    <[TsEnumMember] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_export_assignment(&mut self, node: &TsExportAssignment) {
    <TsExportAssignment as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_expr_with_type_args(&mut self, node: &TsExprWithTypeArgs) {
    <TsExprWithTypeArgs as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_expr_with_type_argss(&mut self, node: &[TsExprWithTypeArgs]) {
    <[TsExprWithTypeArgs] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_external_module_ref(&mut self, node: &TsExternalModuleRef) {
    <TsExternalModuleRef as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_fn_or_constructor_type(&mut self, node: &TsFnOrConstructorType) {
    <TsFnOrConstructorType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_fn_param(&mut self, node: &TsFnParam) {
    <TsFnParam as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_fn_params(&mut self, node: &[TsFnParam]) {
    <[TsFnParam] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_fn_type(&mut self, node: &TsFnType) {
    <TsFnType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_getter_signature(&mut self, node: &TsGetterSignature) {
    <TsGetterSignature as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_import_call_options(&mut self, node: &TsImportCallOptions) {
    <TsImportCallOptions as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_import_equals_decl(&mut self, node: &TsImportEqualsDecl) {
    <TsImportEqualsDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_import_type(&mut self, node: &TsImportType) {
    <TsImportType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_index_signature(&mut self, node: &TsIndexSignature) {
    <TsIndexSignature as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_indexed_access_type(&mut self, node: &TsIndexedAccessType) {
    <TsIndexedAccessType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_infer_type(&mut self, node: &TsInferType) {
    <TsInferType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_instantiation(&mut self, node: &TsInstantiation) {
    <TsInstantiation as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_interface_body(&mut self, node: &TsInterfaceBody) {
    <TsInterfaceBody as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_interface_decl(&mut self, node: &TsInterfaceDecl) {
    <TsInterfaceDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_intersection_type(&mut self, node: &TsIntersectionType) {
    <TsIntersectionType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_keyword_type(&mut self, node: &TsKeywordType) {
    <TsKeywordType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_keyword_type_kind(&mut self, node: &TsKeywordTypeKind) {
    <TsKeywordTypeKind as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_lit(&mut self, node: &TsLit) {
    <TsLit as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_lit_type(&mut self, node: &TsLitType) {
    <TsLitType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_mapped_type(&mut self, node: &TsMappedType) {
    <TsMappedType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_method_signature(&mut self, node: &TsMethodSignature) {
    <TsMethodSignature as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_module_block(&mut self, node: &TsModuleBlock) {
    <TsModuleBlock as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_module_decl(&mut self, node: &TsModuleDecl) {
    <TsModuleDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_module_name(&mut self, node: &TsModuleName) {
    <TsModuleName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_module_ref(&mut self, node: &TsModuleRef) {
    <TsModuleRef as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_namespace_body(&mut self, node: &TsNamespaceBody) {
    <TsNamespaceBody as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_namespace_decl(&mut self, node: &TsNamespaceDecl) {
    <TsNamespaceDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_namespace_export_decl(&mut self, node: &TsNamespaceExportDecl) {
    <TsNamespaceExportDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_non_null_expr(&mut self, node: &TsNonNullExpr) {
    <TsNonNullExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_optional_type(&mut self, node: &TsOptionalType) {
    <TsOptionalType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_param_prop(&mut self, node: &TsParamProp) {
    <TsParamProp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_param_prop_param(&mut self, node: &TsParamPropParam) {
    <TsParamPropParam as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_parenthesized_type(&mut self, node: &TsParenthesizedType) {
    <TsParenthesizedType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_property_signature(&mut self, node: &TsPropertySignature) {
    <TsPropertySignature as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_qualified_name(&mut self, node: &TsQualifiedName) {
    <TsQualifiedName as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_rest_type(&mut self, node: &TsRestType) {
    <TsRestType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_satisfies_expr(&mut self, node: &TsSatisfiesExpr) {
    <TsSatisfiesExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_setter_signature(&mut self, node: &TsSetterSignature) {
    <TsSetterSignature as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_this_type(&mut self, node: &TsThisType) {
    <TsThisType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_this_type_or_ident(&mut self, node: &TsThisTypeOrIdent) {
    <TsThisTypeOrIdent as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_tpl_lit_type(&mut self, node: &TsTplLitType) {
    <TsTplLitType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_tuple_element(&mut self, node: &TsTupleElement) {
    <TsTupleElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_tuple_elements(&mut self, node: &[TsTupleElement]) {
    <[TsTupleElement] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_tuple_type(&mut self, node: &TsTupleType) {
    <TsTupleType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type(&mut self, node: &TsType) {
    <TsType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_type_alias_decl(&mut self, node: &TsTypeAliasDecl) {
    <TsTypeAliasDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_ann(&mut self, node: &TsTypeAnn) {
    <TsTypeAnn as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_assertion(&mut self, node: &TsTypeAssertion) {
    <TsTypeAssertion as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_element(&mut self, node: &TsTypeElement) {
    <TsTypeElement as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_elements(&mut self, node: &[TsTypeElement]) {
    <[TsTypeElement] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_lit(&mut self, node: &TsTypeLit) {
    <TsTypeLit as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_operator(&mut self, node: &TsTypeOperator) {
    <TsTypeOperator as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_operator_op(&mut self, node: &TsTypeOperatorOp) {
    <TsTypeOperatorOp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_param(&mut self, node: &TsTypeParam) {
    <TsTypeParam as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_param_decl(&mut self, node: &TsTypeParamDecl) {
    <TsTypeParamDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_param_instantiation(
    &mut self,
    node: &TsTypeParamInstantiation,
  ) {
    <TsTypeParamInstantiation as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_type_params(&mut self, node: &[TsTypeParam]) {
    <[TsTypeParam] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_predicate(&mut self, node: &TsTypePredicate) {
    <TsTypePredicate as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_query(&mut self, node: &TsTypeQuery) {
    <TsTypeQuery as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_query_expr(&mut self, node: &TsTypeQueryExpr) {
    <TsTypeQueryExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_type_ref(&mut self, node: &TsTypeRef) {
    <TsTypeRef as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_types(&mut self, node: &[Box<TsType>]) {
    <[Box<TsType>] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_ts_union_or_intersection_type(
    &mut self,
    node: &TsUnionOrIntersectionType,
  ) {
    <TsUnionOrIntersectionType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(node, self)
  }

  fn visit_ts_union_type(&mut self, node: &TsUnionType) {
    <TsUnionType as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_unary_expr(&mut self, node: &UnaryExpr) {
    <UnaryExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_unary_op(&mut self, node: &UnaryOp) {
    <UnaryOp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_update_expr(&mut self, node: &UpdateExpr) {
    <UpdateExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_update_op(&mut self, node: &UpdateOp) {
    <UpdateOp as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_using_decl(&mut self, node: &UsingDecl) {
    <UsingDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_var_decl(&mut self, node: &VarDecl) {
    <VarDecl as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_var_decl_kind(&mut self, node: &VarDeclKind) {
    <VarDeclKind as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_var_decl_or_expr(&mut self, node: &VarDeclOrExpr) {
    <VarDeclOrExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_var_declarator(&mut self, node: &VarDeclarator) {
    <VarDeclarator as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_var_declarators(&mut self, node: &[VarDeclarator]) {
    <[VarDeclarator] as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_while_stmt(&mut self, node: &WhileStmt) {
    <WhileStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_with_stmt(&mut self, node: &WithStmt) {
    <WithStmt as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }

  fn visit_yield_expr(&mut self, node: &YieldExpr) {
    <YieldExpr as swc_ecma_visit::VisitWith<Self>>::visit_children_with(
      node, self,
    )
  }
}
