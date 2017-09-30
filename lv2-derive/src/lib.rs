
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Atom, attributes(AtomURI))]
pub fn atom(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_atom(&ast);
    gen.parse().unwrap()
}

fn find_atom_uri(attrs: &Vec<syn::Attribute>) -> Option<String> {
    let mut uri = None;
    for a in attrs {
        match a.value {
            syn::MetaItem::NameValue(ref n, syn::Lit::Str(ref sym, _)) => {
                if n == "AtomURI" {
                    uri = Some(sym);
                    break;
                }
            },
            _ => {}
        }
    }
    uri.map(|uri| {
        if uri.starts_with("http") {
            // literal uri
            format!("\"{}\"", uri)
        }
        else {
            // symbol (must be visible at derive site)
            uri.to_owned()
        }
    })
}

fn impl_atom(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let uri = find_atom_uri(&ast.attrs)
            .expect("#[AtomURI = \"...\"] must be defined with #[derive(Atom)]");
    quote! {
        impl Atom for #name {
            fn type_uri() -> &'static str {
                #uri
            }
            fn type_urid(&self) -> urid::URID {
                self.header.type_urid
            }
            fn size(&self) -> usize {
                self.header.size as usize
            }
            fn header(&self) -> Header {
                self.header
            }
        }
    }
}
