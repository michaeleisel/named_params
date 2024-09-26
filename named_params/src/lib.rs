extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, FnArg, ItemFn, Lifetime, Pat, PatIdent, PatType, Type
};

#[proc_macro_attribute]
pub fn named_params(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function name
    let fn_name = input_fn.sig.ident.clone();

    // Generate the struct name by converting the function name to PascalCase and appending "Args"
    let struct_name = format_ident!("{}Args", pascal_case(&fn_name.to_string()));

    // Collect the function arguments
    let mut fields = Vec::new();
    let mut patterns = Vec::new();

    let mut did_adjust_lifetime = false;
    for input in input_fn.sig.inputs.iter() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                let mut adjusted_ty = (*ty).clone();
                if replace_lifetimes_with_a(&mut adjusted_ty) {
                    did_adjust_lifetime = true;
                }
                fields.push(quote! { #ident: #adjusted_ty });
                patterns.push(quote! { #ident });
            }
        } else if let FnArg::Receiver(_) = input {
            // Handle methods with self parameter if needed
            unimplemented!("Methods with 'self' parameter are not supported in this macro.");
        }
    }

    // Add the 'a lifetime to the struct declaration if we actually used it
    let struct_def = if did_adjust_lifetime {
        quote! {
            struct #struct_name<'a> {
                #(#fields),*
            }
        }
    } else {
        quote! {
            struct #struct_name {
                #(#fields),*
            }
        }
    };

    let new_arg: FnArg = syn::parse_quote! {
        #struct_name { #(#patterns),* }: #struct_name
    };

    input_fn.sig.inputs.clear();
    input_fn.sig.inputs.push(new_arg);

    let output = quote! {
        #struct_def

        #input_fn
    };

    TokenStream::from(output)
}

fn replace_lifetimes_with_a(ty: &mut Type) -> bool {
    match ty {
        Type::Tuple(ref mut type_tuple) => {
            let mut did_adjust_lifetime = false;
            for elem in &mut type_tuple.elems {
                if replace_lifetimes_with_a(elem) {
                    did_adjust_lifetime = true;
                }
            }
            did_adjust_lifetime
        }
        Type::Reference(ref mut ref_type) => {
            if ref_type.lifetime == None {
                ref_type.lifetime = Some(Lifetime::new("'a", proc_macro2::Span::call_site()));
            } else {
                todo!("Explicit lifetimes not yet supported");
            }
            true
            // TODO: if we have a reference to a tuple, make sure the tuple members have the appropriate lifetime
        }
        _ => {
            false
        }
    }
}

fn pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

// Note that we can't as easily do unit tests because if we call a method that takes or returns a TokenStream, Rust
// complains