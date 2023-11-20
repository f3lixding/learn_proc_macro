use protocol::Context;
use std::collections::HashMap;

use proc_macro2::Literal;
use proc_macro2::TokenTree;
use quote::quote;
use syn::parse::Parse;
use syn::token;
use syn::Ident;

// first we are going to try something simple:
// we are going to take something like num~"phrase" where num is i32 and "phrase" is a string
// the effect is that we are going to println "phrase" num times
#[proc_macro]
pub fn shout_one_or_two(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let tokens = input.into_iter().collect::<Vec<TokenTree>>();

    let first_token = tokens.get(0).unwrap();
    let num = match first_token {
        TokenTree::Literal(lit) => lit.to_string().parse::<i32>().unwrap(),
        _ => panic!("first token is not a literal that can be parsed as an i32"),
    };

    let second_token = tokens.get(1).unwrap();
    if let TokenTree::Punct(punct) = second_token {
        let delimiter = punct.to_string();
        if delimiter != "~" {
            panic!("Unexpected delimiter encountered");
        }
    } else {
        panic!("Unexpected second token encountered");
    }

    let third_token = tokens.get(2).unwrap();
    let phrase = match third_token {
        TokenTree::Literal(lit) => lit.to_string(),
        _ => panic!("Unexpected third token encountered"),
    };

    let output = quote! {
        for _ in 0..#num {
            println!("{}", #phrase);
        }
    };

    output.into()
}

// And then here we are going to do something a bit more involved. We are also going to employ the
// help of some of the crates we have imported to make things a littel easier.
//
// We are going to have a macro that takes in a token stream that looks like this
// ```
// func_id <= {key: value, key2: value2}
// ```
// where keys are the type and values are their respective values
// and this is going to return a function that saves these values and types in scope and wraps it
// another function to call it with these things.
// and func_id is the name of the function
#[proc_macro]
pub fn create_call_back(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let func_expr = syn::parse_macro_input!(input as FuncExpr);

    match func_expr.name.to_string().as_str() {
        "get_name_with_string" => {
            quote! {
                Arc::new(|ctx: &Context| {
                    get_name_with_string(ctx)
                })
            }
        }
        "get_friends_id" => {
            let name = func_expr.arguments.get("String").unwrap();
            quote! {
                Arc::new(|ctx: &Context| -> Option<i32> {
                    get_friend_id(ctx, #name)
                })
            }
        }
        _ => panic!("Unknown function encountered"),
    }
    .into()
}

struct FuncExpr {
    name: Ident,
    arguments: HashMap<String, Literal>,
}

impl Parse for FuncExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<token::Le>()?;

        let mut arguments = HashMap::new();
        let content;
        let _ = syn::braced!(content in input);

        while !content.is_empty() {
            let type_: Ident = content.parse()?;
            content.parse::<token::Colon>()?;
            let literal: Literal = content.parse()?;
            arguments.insert(type_.to_string(), literal);
        }

        Ok(FuncExpr { name, arguments })
    }
}
