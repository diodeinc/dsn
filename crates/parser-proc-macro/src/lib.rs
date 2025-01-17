use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[derive(Clone, Debug)]
struct SexprAttrs {
    name: Option<String>,
    anonymous: Option<bool>,
}

// TODO: this is horrendous
impl TryFrom<&Vec<syn::Attribute>> for SexprAttrs {
    type Error = syn::Error;

    fn try_from(input: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut name: Option<String> = None;
        let mut anonymous: Option<bool> = None;
        for attr in input {
            if attr.path().is_ident("sexpr") {
                match &attr.meta {
                    syn::Meta::List(list) => {
                        let nested = list.parse_args::<syn::Meta>()?;
                        match nested {
                            syn::Meta::Path(path) if path.is_ident("anonymous") => {
                                anonymous = Some(true);
                            }
                            syn::Meta::NameValue(name_value)
                                if name_value.path.is_ident("name") =>
                            {
                                name = Some(
                                    match syn::parse2::<syn::LitStr>(
                                        name_value.value.to_token_stream(),
                                    ) {
                                        Ok(lit) => lit.value(),
                                        Err(e) => {
                                            return Err(syn::Error::new_spanned(
                                                &name_value.value,
                                                format!("Expected string literal for name attribute: {}", e),
                                            ));
                                        }
                                    },
                                );
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &nested,
                                    "Expected either 'anonymous' or 'name = \"value\"'",
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "Expected #[sexpr(anonymous)] or #[sexpr(name = \"value\")]",
                        ));
                    }
                }
            }
        }

        if anonymous.is_some() && name.is_some() {
            return Err(syn::Error::new_spanned(
                input.first().unwrap(),
                "anonymous and name attributes cannot be used together",
            ));
        }

        Ok(Self { name, anonymous })
    }
}

#[derive(Debug)]
struct StructParserConfig<'a> {
    /// The identifier of the parser in the generated Parsable implementation.
    parser_ident: &'a syn::Ident,

    /// If none, this will be parsed as an anonymous struct, i.e. without enclosing parens
    /// or a keyword. If Some, this will be parsed as a named struct, i.e. with a keyword.
    parse_as_name: Option<String>,

    /// The fields of the struct.
    fields: Vec<FieldParserConfig<'a>>,

    /// The style of the struct.
    style: StructStyle,
}

#[derive(Debug)]
enum StructStyle {
    Named,
    Tuple,
    Unit,
}

impl<'a> StructParserConfig<'a> {
    fn new(
        parser_ident: &'a syn::Ident,
        default_parse_name: Option<String>,
        type_attrs: SexprAttrs,
        fields: &'a syn::Fields,
    ) -> syn::Result<Self> {
        let parse_as_name = match (type_attrs.anonymous, type_attrs.name, default_parse_name) {
            (Some(true), _, _) => None,
            (_, Some(name), _) => Some(name),
            (_, None, Some(name)) => Some(name),
            (_, None, None) => None,
        };

        let field_configs = fields
            .iter()
            .enumerate()
            .map(|(i, field)| -> syn::Result<FieldParserConfig> {
                let field_ident = match &field.ident {
                    Some(ident) => ident.clone(),
                    None => format_ident!("f{}", i),
                };
                Ok(FieldParserConfig::new(
                    format_ident!("{}_{}", parser_ident, field_ident),
                    field.ident.as_ref().map(|ident| ident.to_string()),
                    field_ident,
                    SexprAttrs::try_from(&field.attrs)?,
                    &field.ty,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            parser_ident,
            parse_as_name,
            fields: field_configs,
            style: match fields {
                syn::Fields::Named(_) => StructStyle::Named,
                syn::Fields::Unnamed(_) => StructStyle::Tuple,
                syn::Fields::Unit => StructStyle::Unit,
            },
        })
    }

    fn generate_impl(&self) -> TokenStream {
        let parser_ident = &self.parser_ident;

        let field_parser_idents = self
            .fields
            .iter()
            .map(|field| &field.parser_ident)
            .collect::<Vec<_>>();

        let field_parser_impls = self
            .fields
            .iter()
            .map(|field| TokenStream::from(field.generate_impl()))
            .collect::<Vec<TokenStream>>();

        let expanded = match (&self.parse_as_name, &self.style) {
            (Some(name), StructStyle::Unit) => quote! {
                let #parser_ident = parser::keyword(#name).padded();
            },
            (Some(name), _) => quote! {
                #(#field_parser_impls)*
                let #parser_ident = parser::lparen()
                    .ignore_then(parser::keyword(#name))
                    #(.then(#field_parser_idents))*
                    .then_ignore(parser::rparen())
                    .padded();
            },
            (None, _) => quote! {
                #(#field_parser_impls)*
                let #parser_ident = empty()
                    #(.then(#field_parser_idents))*
                    .padded();
            },
        };

        TokenStream::from(expanded)
    }
}

#[derive(Debug)]
struct FieldParserConfig<'a> {
    /// The identifier of the parser in the generated Parsable implementation.
    parser_ident: syn::Ident,

    /// The identifier we use to reference the field, either a placeholder for unnamed fields or
    /// the name of the field for named fields.
    field_ident: syn::Ident,

    /// If none, this will be parsed as an anonymous field, i.e. without a name.
    parse_as_name: Option<String>,

    /// The type of the field.
    ty: &'a syn::Type,
}

impl<'a> FieldParserConfig<'a> {
    fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
        if let syn::Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            return Some(inner_ty);
                        }
                    }
                }
            }
        }
        None
    }

    fn new(
        parser_ident: syn::Ident,
        default_field_name: Option<String>,
        field_ident: syn::Ident,
        field_attrs: SexprAttrs,
        ty: &'a syn::Type,
    ) -> Self {
        let parse_as_name = match (field_attrs.anonymous, field_attrs.name, default_field_name) {
            (Some(true), _, _) => None,
            (_, Some(name), _) => Some(name),
            (_, None, Some(_)) => None,
            (_, None, None) => None,
        };

        Self {
            parser_ident,
            field_ident,
            parse_as_name,
            ty,
        }
    }

    fn generate_impl(&self) -> TokenStream {
        let parser_ident = &self.parser_ident;
        let ty = self.ty;

        // Check if this is an Option<T> type
        let mut is_option = false;
        let parser_chain = if let Some(inner_type) = Self::extract_option_inner_type(ty) {
            is_option = true;
            let inner_type_tokens = inner_type.to_token_stream();
            quote! { <#inner_type_tokens>::parser() }
        } else {
            let ty_tokens = ty.to_token_stream();
            quote! { <#ty_tokens>::parser() }
        };

        let expanded = match &self.parse_as_name {
            Some(name) => quote! {
                let #parser_ident = parser::lparen()
                    .ignore_then(parser::keyword(#name))
                    .ignore_then(#parser_chain)
                    .then_ignore(parser::rparen())
                    .padded()
            },
            None => quote! {
                let #parser_ident = #parser_chain.padded()
            },
        };

        if is_option {
            quote! {
                #expanded.or_not();
            }
        } else {
            quote! {
                #expanded;
            }
        }
    }
}

/// Generate and return the Parsable implementation for a struct.
fn derive_sexpr_impl_struct(input: &DeriveInput) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    let type_name = type_ident.to_string();
    let type_name_snake = type_name.to_case(Case::Snake);
    let type_attrs = SexprAttrs::try_from(&input.attrs)?;
    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("expected struct"),
    };
    let parser_ident = &format_ident!("{}_parser", type_name_snake);
    let struct_parser_config =
        StructParserConfig::new(parser_ident, Some(type_name_snake), type_attrs, fields)?;

    let field_destructuring = if let Some(first_field) = struct_parser_config.fields.first() {
        let first_field_ident = &first_field.field_ident;
        let mut tokens = quote!((_, #first_field_ident));
        for field in struct_parser_config.fields.iter().skip(1) {
            let field_ident = &field.field_ident;
            tokens = quote!((#tokens, #field_ident));
        }
        tokens
    } else {
        quote!(_)
    };

    let parser_impl = struct_parser_config.generate_impl();

    let field_idents = struct_parser_config
        .fields
        .iter()
        .map(|field| &field.field_ident)
        .collect::<Vec<_>>();

    // Generate the implementation
    let implementation = match &struct_parser_config.style {
        StructStyle::Named => {
            quote! {
                impl parser::Parsable for #type_ident {
                    fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|#field_destructuring| Self {
                            #(#field_idents),*
                        })
                    }
                }
            }
        }
        StructStyle::Tuple => {
            quote! {
                impl parser::Parsable for #type_ident {
                    fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|#field_destructuring| Self(
                            #(#field_idents),*
                        ))
                    }
                }
            }
        }
        StructStyle::Unit => {
            quote! {
                impl parser::Parsable for #type_ident {
                    fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|_| Self)
                    }
                }
            }
        }
    };

    // Convert back to token stream and return
    Ok(implementation)
}

fn derive_sexpr_impl_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    let type_name = type_ident.to_string();
    let type_name_snake = type_name.to_case(Case::Snake);
    let variants = match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("expected enum"),
    };

    let mut variant_parsers: Vec<TokenStream> = Vec::new();
    let mut variant_parser_idents: Vec<syn::Ident> = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();
        let variant_name_snake = variant_name.to_case(Case::Snake);
        let variant_type_attrs = SexprAttrs::try_from(&variant.attrs)?;

        let parser_ident = &format_ident!("{}_{}_parser", type_name_snake, variant_name_snake);
        let struct_parser_config = StructParserConfig::new(
            parser_ident,
            Some(variant_name_snake),
            variant_type_attrs,
            &variant.fields,
        )?;

        let field_destructuring = if let Some(first_field) = struct_parser_config.fields.first() {
            let first_field_ident = &first_field.field_ident;
            let mut tokens = quote!((_, #first_field_ident));
            for field in struct_parser_config.fields.iter().skip(1) {
                let field_ident = &field.field_ident;
                tokens = quote!((#tokens, #field_ident));
            }
            tokens
        } else {
            quote!(_)
        };

        let parser_impl = struct_parser_config.generate_impl();

        let field_idents = struct_parser_config
            .fields
            .iter()
            .map(|field| &field.field_ident)
            .collect::<Vec<_>>();

        let variant_parser_ident = &format_ident!("{}_variant", parser_ident);
        variant_parser_idents.push(variant_parser_ident.clone());
        let variant_parser = match &struct_parser_config.style {
            StructStyle::Named => quote! {
                #parser_impl
                let #variant_parser_ident = #parser_ident.map(|#field_destructuring| Self::#variant_ident {
                    #(#field_idents),*
                });
            },
            StructStyle::Tuple => quote! {
                #parser_impl
                let #variant_parser_ident = #parser_ident.map(|#field_destructuring| Self::#variant_ident(
                    #(#field_idents),*
                ));
            },
            StructStyle::Unit => quote! {
                #parser_impl
                let #variant_parser_ident = #parser_ident.map(|_| Self::#variant_ident);
            },
        };

        variant_parsers.push(variant_parser);
    }

    let implementation = if variants.is_empty() {
        quote!(
            impl parser::Parsable for #type_ident {
                fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::error::Simple<char>> {
                    use chumsky::prelude::*;

                    chumsky::primitive::filter(|_| false)
                        .map(|_| -> #type_ident {
                            unreachable!("Parser for uninhabited enum should never succeed")
                        })
                }
            }
        )
    } else {
        quote!(
           impl parser::Parsable for #type_ident {
               fn parser() -> impl chumsky::Parser<char, Self, Error = chumsky::error::Simple<char>> {
                   use chumsky::prelude::*;

                   #(#variant_parsers)*
                   choice((
                       #(#variant_parser_idents),*
                       ,
                   ))
               }
           }
        )
    };

    Ok(implementation)
}

#[proc_macro_derive(Sexpr, attributes(sexpr))]
pub fn derive_sexpr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = match &input.data {
        syn::Data::Struct(syn::DataStruct { .. }) => {
            derive_sexpr_impl_struct(&input).expect("failed to generate sexpr impl for struct")
        }
        syn::Data::Enum(syn::DataEnum { .. }) => {
            derive_sexpr_impl_enum(&input).expect("failed to generate sexpr impl for enum")
        }
        _ => panic!("sexpr only works with structs and enums"),
    };

    TokenStream::from(expanded).into()
}
