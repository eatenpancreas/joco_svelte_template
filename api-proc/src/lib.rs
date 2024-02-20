use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{ItemFn, LitStr, parse_macro_input, Token, Type};
use syn::parse::{Parse, ParseStream};

struct Args {
    route: LitStr,
    method: LitStr,
    in_t: Type,
    out_t: Type,
    query_t: Type,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let route = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let method = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let in_t = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let out_t = input.parse()?;
        let _ = input.parse::<Token![,]>();
        let query_t = input.parse()?;
        
        Ok(Args {
            route, method,
            in_t, out_t, query_t
        })
    }
}

#[proc_macro_attribute]
pub fn endpoint(input: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as Args);
    let func = parse_macro_input!(item as ItemFn);
    
    let struct_name = format_ident!("__Endpoint_{}", func.sig.ident);
    let test_name = format_ident!("__export_write_endpoint_{}", func.sig.ident);
    let struct_name_str = struct_name.to_string();

    let route = args.route;
    let method = args.method;
    let method_val = method.value().to_uppercase();
    let in_t = args.in_t;
    let out_t = args.out_t;
    let query_t = args.query_t;
    
    let route_raw = format!("../src/lib/endpoint_defines{}/{}.ts", route.value(), method_val);
    let route_full = LitStr::new(route_raw.as_str(), Span::call_site());
    let expanded = quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, ts_rs::TS)]
        #[ts(export, export_to = #route_full)]
        struct #struct_name {
          in_type: #in_t,
          out_data_type: #out_t,
          query_type: #query_t
        }
        
        #[cfg(test)]
        #[test]
        fn #test_name() {
          api_lib::handshake::endpoint::write_endpoint(#method, #route, #struct_name_str);
        }
    };

    TokenStream::from(quote!(
        #expanded
        #func
    ))
}

