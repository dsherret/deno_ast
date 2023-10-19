use swc_common::DUMMY_SP;
// use swc_atoms::{js_word, Atom, JsWord};
use swc_common::{
  //   comments::{Comment, CommentKind, Comments},
  //   errors::HANDLER,
  //   iter::IdentifyLast,
  //   sync::Lrc,
  util::take::Take,
  //   FileName, Mark, SourceMap,
  Span,
  Spanned,
};
use swc_ecma_ast::*;
// use swc_ecma_utils::{
//   drop_span, member_expr, prepend_stmt, private_ident, quote_ident, undefined,
//   ExprFactory,
// };
use swc_ecma_visit::{
  as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith,
};

struct JsxString {
  next_index: usize,
}

impl Default for JsxString {
  fn default() -> Self {
    Self { next_index: 0 }
  }
}

fn create_tpl_binding_name(index: usize) -> String {
  format!("$$_tpl_{index}")
}

impl JsxString {
  fn generate_tpl_join(&self, el: JSXElement) -> Expr {
    let name = create_tpl_binding_name(self.next_index);
    let span = el.span();

    // $$_tpl_1.join("");
    Expr::Call(CallExpr {
      span,
      callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new(name.into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident::new("join".into(), DUMMY_SP)),
      }))),
      args: vec![ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Str(Str {
          span: DUMMY_SP,
          value: "".into(),
          raw: None,
        }))),
      }],
      type_args: Default::default(),
    })
  }
}

impl VisitMut for JsxString {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    eprintln!("ast {:#?}", module);
    module.visit_mut_children_with(self);
  }

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    // let top_level_node = self.top_level_node;
    // let mut did_work = false;

    if let Expr::JSXElement(el) = expr {
      // TODO:
      // 1. create a `_tpl_<name>` which is an array literal
      // 2. transform the element into a list of string literals and
      //    push them to the `_tpl_<name>` literal node
      // 3. change the `expr` to be `_tpl_<name>.join("");`

      self.next_index += 1;

      //   did_work = true;
      //   // <div></div> => React.createElement('div', null);
      *expr = self.generate_tpl_join(*el.take());
    } else if let Expr::JSXFragment(frag) = expr {
      //   // <></> => React.createElement(React.Fragment, null);
      //   did_work = true;
      //   *expr = self.jsx_frag_to_expr(frag.take());
    } else if let Expr::Paren(ParenExpr {
      expr: inner_expr, ..
    }) = expr
    {
      if let Expr::JSXElement(el) = &mut **inner_expr {
        // did_work = true;
        // *expr = self.jsx_elem_to_expr(*el.take());
      } else if let Expr::JSXFragment(frag) = &mut **inner_expr {
        // <></> => React.createElement(React.Fragment, null);
        // did_work = true;
        // *expr = self.jsx_frag_to_expr(frag.take());
      }
    }

    // if did_work {
    //   self.top_level_node = false;
    // }

    expr.visit_mut_children_with(self);

    // self.top_level_node = top_level_node;
  }
}

#[cfg(test)]
mod tests {
  use crate::swc::ast::Module;
  use crate::swc::codegen::text_writer::JsWriter;
  use crate::swc::codegen::Node;
  use crate::swc::common::FileName;
  use crate::swc::common::SourceMap;
  use crate::swc::parser::Parser;
  use crate::swc::parser::StringInput;
  use crate::swc::parser::Syntax;
  use crate::swc::parser::TsConfig;
  use crate::swc::visit::Fold;
  use crate::swc::visit::FoldWith;
  use crate::ModuleSpecifier;
  use crate::ES_VERSION;
  use pretty_assertions::assert_eq;
  use std::rc::Rc;

  use super::*;

  #[test]
  fn basic_test() {
    test_transform(
      JsxString::default(),
      r#"const a = <div>Hello!</div>;
const b = <div>Hello!</div>;"#,
      r#"const $$_tpl_1 = ["<div>Hello!</div>"];
const $$_tpl_2 = ["<div>Hello!</div>"];
const a = $$_tpl_1.join("");
const b = $$_tpl_2.join("");"#,
    );
  }

  #[track_caller]
  fn test_transform(
    transform: impl VisitMut,
    src: &str,
    expected_output: &str,
  ) {
    let (source_map, module) = parse(src);
    let mut transform_folder = as_folder(transform);
    let output = print(source_map, module.fold_with(&mut transform_folder));
    assert_eq!(output, format!("{}\n", expected_output));
  }

  fn parse(src: &str) -> (Rc<SourceMap>, Module) {
    let source_map = Rc::new(SourceMap::default());
    let source_file = source_map.new_source_file(
      FileName::Url(ModuleSpecifier::parse("file:///test.ts").unwrap()),
      src.to_string(),
    );
    let input = StringInput::from(&*source_file);
    let syntax = Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
    });
    let mut parser = Parser::new(syntax, input, None);
    (source_map, parser.parse_module().unwrap())
  }

  fn print(source_map: Rc<SourceMap>, module: Module) -> String {
    let mut buf = vec![];
    {
      let mut writer =
        Box::new(JsWriter::new(source_map.clone(), "\n", &mut buf, None));
      writer.set_indent_str("  "); // two spaces
      let mut emitter = crate::swc::codegen::Emitter {
        cfg: crate::swc_codegen_config(),
        comments: None,
        cm: source_map,
        wr: writer,
      };
      module.emit_with(&mut emitter).unwrap();
    }
    String::from_utf8(buf).unwrap()
  }
}