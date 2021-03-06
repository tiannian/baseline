use syn::{parse_quote, FieldValue, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

pub fn impl_clone(st: &ItemStruct) -> Result<Item> {
    let mut field = Vec::new();

    for item in &st.fields {
        let ident = item.ident.clone();

        let f: FieldValue = parse_quote! {
            #ident: self.#ident.clone()
        };

        field.push(f);
    }

    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        impl Clone for #ident<#generics_params> {
            fn clone(&self) -> Self {
                Self {
                    #(#field,)*
                }
            }
        }
    };

    res.generics = st.generics.clone();

    //     for item in res.generics.params.iter_mut() {
    //     if let GenericParam::Type(t) = item {
    //         if let Type::Path(p) = ctx_type {
    //             if Some(&t.ident) == p.path.get_ident() {
    //                 t.bounds.push(parse_quote!(baseline::prelude::ContextSetable))
    //             }
    //         }
    //     }
    // }
    //
    Ok(Item::Impl(res))
}
