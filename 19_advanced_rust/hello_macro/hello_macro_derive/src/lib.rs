extern crate proc_macro;
//proc_macro comes with Rust
use proc_macro::TokenStream;
//quote crate turns syn data structures back into Rust code
use quote::quote;
//syn crate parses Rust code from a string into a data structure
use syn;

#[proc_macro_derive(HelloMacro)]
//this function will be called when a library user specifies #[derive(HelloMacro)] on a type
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    //input(TokenStream) -> data structure
    //parse function returns a 'DeriveInput' struct
    //unwrap will panic when parse fail
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    //In DeriveInput struct, there is an 'Ident' field
    let name = &ast.ident; //ident field value
                           //The quote! macro let us define the Rust code
    let gen = quote! {
        //#name -> quote! will replace it with 'name' variable value
        impl HelloMacro for #name {
            fn hello_macro() {
                //stringify macro takes rust expression(e.g. 1 + 2)
                //and turns the expression into a string literal at compile time (e.g. "1+2")
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into() //This changes gen into TokenStream format
}
