use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Ident, Member, Path, Stmt};

#[proc_macro]
pub fn self_assign(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let SelfAssign {
        target,
        struct_name,
        fields,
    } = parse_macro_input!(input as SelfAssign);

    let mut quoted_field_def = vec![];
    for FieldAssign { key, .. } in &fields {
        let q = quote! { #key: unreachable!() };
        quoted_field_def.push(q);
    }

    let mut quoted_field_assigning = vec![];
    for FieldAssign { key, value } in &fields {
        let Some(value) = value else {
            continue;
        };
        let q = quote! { #target.#key = #value };
        quoted_field_assigning.push(q);
    }

    let output = quote! {
        // Make sure all fields are explicitly mentioned.
        #[allow(unreachable_code)]
        let _ = || #struct_name {
            #( #quoted_field_def, )*
        };

        // assigning
        #( #quoted_field_assigning; )*
    };
    output.into()
}

struct SelfAssign {
    pub target: Box<Expr>,
    pub struct_name: Path,
    pub fields: Vec<FieldAssign>,
}
impl Parse for SelfAssign {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let stmt: Stmt = input.parse()?;
        let expr = match stmt {
            Stmt::Expr(expr, _) => expr,
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    format!("`stmt` expected `Stmt::Expr`, found {stmt:?}"),
                ));
            }
        };
        let expr = match expr {
            Expr::Assign(expr) => expr,
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    format!("`expr` expected `Expr::Assign`, found {expr:?}"),
                ));
            }
        };
        let target = expr.left;
        let right = expr.right;
        let structure = match *right {
            Expr::Struct(structure) => structure,
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    format!("`right` expected `Expr::Struct`, found {right:?}"),
                ));
            }
        };
        let struct_name = structure.path;
        let mut fields = vec![];
        for f in structure.fields {
            let member = f.member;
            let key = match member {
                Member::Named(ident) => ident,
                _ => {
                    return Err(syn::Error::new(
                        input.span(),
                        format!("`member` expected `Member::Named`, found {member:?}"),
                    ));
                }
            };
            let expr = f.expr;
            let value = match expr {
                Expr::Infer(_) => None,
                _ => Some(expr),
            };
            fields.push(FieldAssign { key, value });
        }

        Ok(Self {
            target,
            struct_name,
            fields,
        })
    }
}

struct FieldAssign {
    pub key: Ident,
    pub value: Option<Expr>,
}
