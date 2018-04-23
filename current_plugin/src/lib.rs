#![feature(plugin_registrar, rustc_private, quote)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;
use syntax::symbol::Symbol;

use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemModifier};
use syntax::fold::{self, Folder};

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("current_crate"),
        SyntaxExtension::MultiModifier(Box::new(TargetGuardExpander {})),
    );
}

pub struct TargetGuardExpander;
pub struct TargetGuardFolder<'a, 'ctx: 'a> {
    ctx: &'a mut ExtCtxt<'ctx>,
}

impl MultiItemModifier for TargetGuardExpander {
    fn expand(
        &self,
        ctx: &mut ExtCtxt,
        _: Span,
        _: &MetaItem,
        input: Annotatable,
    ) -> Vec<Annotatable> {
        let mut folder = TargetGuardFolder::new(ctx);

        match input {
            Annotatable::Item(item) => {
                // let item = item.map(|mut item| {
                //     folder.add_root_module_attributes(&mut item.attrs);

                //     item
                // });

                folder
                    .fold_item(item)
                    .into_iter()
                    .map(|item| Annotatable::Item(item))
                    .collect()
            }

            other @ _ => vec![other],
        }
    }
}

impl<'a, 'b> Folder for TargetGuardFolder<'a, 'b> {
    // fn fold_mod(&mut self, module: Mod) -> Mod {
    //     let folded = fold::noop_fold_mod(module, self);
    //     folded
    // }

    // fold_foreign_mod

    // TODO: investigate fold_vis to ensure kernels module is exported
}

impl<'a, 'ctx> TargetGuardFolder<'a, 'ctx> {
    pub fn new(ctx: &'a mut ExtCtxt<'ctx>) -> Self {
        TargetGuardFolder { ctx }
    }

    // TODO: investigate how can we setup these attributes - it has to be done earlier!
    // pub fn add_root_module_attributes(&self, attrs: &mut Vec<Attribute>) {
    //     let crate_attrs_tts = &[
    //         quote_tokens!(self.ctx, #![cfg_attr(target_os = "cuda", feature(abi_ptx))]),
    //         quote_tokens!(self.ctx, #![cfg_attr(target_os = "cuda", no_std)]),
    //     ];

    //     for tt in crate_attrs_tts {
    //         attrs.push(
    //             self.ctx
    //                 .new_parser_from_tts(&tt)
    //                 .parse_attribute(true)
    //                 .unwrap(),
    //         );
    //     }
    // }
}
