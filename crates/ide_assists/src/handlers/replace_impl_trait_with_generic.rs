use syntax::{
    ast::{self, edit_in_place::GenericParamsOwnerEdit, make, AstNode},
    ted,
};

use crate::{utils::suggest_name, AssistContext, AssistId, AssistKind, Assists};

// Assist: replace_impl_trait_with_generic
//
// Replaces `impl Trait` function argument with the named generic.
//
// ```
// fn foo(bar: $0impl Bar) {}
// ```
// ->
// ```
// fn foo<B: Bar>(bar: B) {}
// ```
pub(crate) fn replace_impl_trait_with_generic(
    acc: &mut Assists,
    ctx: &AssistContext,
) -> Option<()> {
    let impl_trait_type = ctx.find_node_at_offset::<ast::ImplTraitType>()?;
    let param = impl_trait_type.syntax().parent().and_then(ast::Param::cast)?;
    let fn_ = param.syntax().ancestors().find_map(ast::Fn::cast)?;

    let type_bound_list = impl_trait_type.type_bound_list()?;

    let target = fn_.syntax().text_range();
    acc.add(
        AssistId("replace_impl_trait_with_generic", AssistKind::RefactorRewrite),
        "Replace impl trait with generic",
        target,
        |edit| {
            let impl_trait_type = edit.make_ast_mut(impl_trait_type);
            let fn_ = edit.make_ast_mut(fn_);

            let type_param_name = suggest_name::for_generic_parameter(&impl_trait_type);

            let type_param = make::type_param(make::name(&type_param_name), Some(type_bound_list))
                .clone_for_update();
            let new_ty = make::ty(&type_param_name).clone_for_update();

            ted::replace(impl_trait_type.syntax(), new_ty.syntax());
            fn_.get_or_create_generic_param_list().add_generic_param(type_param.into())
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::check_assist;

    #[test]
    fn replace_impl_trait_with_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo<G>(bar: $0impl Bar) {}"#,
            r#"fn foo<G, B: Bar>(bar: B) {}"#,
        );
    }

    #[test]
    fn replace_impl_trait_without_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo(bar: $0impl Bar) {}"#,
            r#"fn foo<B: Bar>(bar: B) {}"#,
        );
    }

    #[test]
    fn replace_two_impl_trait_with_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo<G>(foo: impl Foo, bar: $0impl Bar) {}"#,
            r#"fn foo<G, B: Bar>(foo: impl Foo, bar: B) {}"#,
        );
    }

    #[test]
    fn replace_impl_trait_with_empty_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo<>(bar: $0impl Bar) {}"#,
            r#"fn foo<B: Bar>(bar: B) {}"#,
        );
    }

    #[test]
    fn replace_impl_trait_with_empty_multiline_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"
fn foo<
>(bar: $0impl Bar) {}
"#,
            r#"
fn foo<B: Bar
>(bar: B) {}
"#,
        );
    }

    #[test]
    #[ignore = "This case is very rare but there is no simple solutions to fix it."]
    fn replace_impl_trait_with_exist_generic_letter() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo<B>(bar: $0impl Bar) {}"#,
            r#"fn foo<B, C: Bar>(bar: C) {}"#,
        );
    }

    #[test]
    fn replace_impl_trait_with_multiline_generic_params() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"
fn foo<
    G: Foo,
    F,
    H,
>(bar: $0impl Bar) {}
"#,
            r#"
fn foo<
    G: Foo,
    F,
    H, B: Bar,
>(bar: B) {}
"#,
        );
    }

    #[test]
    fn replace_impl_trait_multiple() {
        check_assist(
            replace_impl_trait_with_generic,
            r#"fn foo(bar: $0impl Foo + Bar) {}"#,
            r#"fn foo<F: Foo + Bar>(bar: F) {}"#,
        );
    }
}
