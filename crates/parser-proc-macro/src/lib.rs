use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Item, ItemEnum, ItemStruct};

#[derive(Clone, Debug, Default)]
struct SexprAttrs {
    name: Option<String>,
    anonymous: Option<bool>,
}

impl SexprAttrs {
    fn parse_meta_name_value(name_value: &syn::MetaNameValue) -> syn::Result<String> {
        if !name_value.path.is_ident("name") {
            return Err(syn::Error::new_spanned(
                name_value,
                "Expected 'name' attribute",
            ));
        }

        match syn::parse2::<syn::LitStr>(name_value.value.to_token_stream()) {
            Ok(lit) => Ok(lit.value()),
            Err(e) => Err(syn::Error::new_spanned(
                &name_value.value,
                format!("Expected string literal for name attribute: {}", e),
            )),
        }
    }

    fn parse_meta(meta: &syn::Meta) -> syn::Result<(Option<String>, Option<bool>)> {
        match meta {
            syn::Meta::Path(path) if path.is_ident("anonymous") => Ok((None, Some(true))),
            syn::Meta::NameValue(name_value) => {
                let name = Self::parse_meta_name_value(name_value)?;
                Ok((Some(name), None))
            }
            _ => Err(syn::Error::new_spanned(
                meta,
                "Expected either 'anonymous' or 'name = \"value\"'",
            )),
        }
    }

    fn parse_attribute(attr: &syn::Attribute) -> syn::Result<(Option<String>, Option<bool>)> {
        if !attr.path().is_ident("sexpr") {
            return Ok((None, None));
        }

        match &attr.meta {
            syn::Meta::List(list) => {
                let nested = list.parse_args::<syn::Meta>()?;
                Self::parse_meta(&nested)
            }
            _ => Err(syn::Error::new_spanned(
                attr,
                "Expected #[sexpr(anonymous)] or #[sexpr(name = \"value\")]",
            )),
        }
    }

    fn validate(name: Option<String>, anonymous: Option<bool>) -> syn::Result<Self> {
        match (name.clone(), anonymous) {
            (Some(_), Some(_)) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "anonymous and name attributes cannot be used together",
            )),
            _ => Ok(Self { name, anonymous }),
        }
    }
}

impl TryFrom<&Vec<syn::Attribute>> for SexprAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut name = None;
        let mut anonymous = None;

        for attr in attrs {
            let (new_name, new_anonymous) = Self::parse_attribute(attr)?;
            name = name.or(new_name);
            anonymous = anonymous.or(new_anonymous);
        }

        Self::validate(name, anonymous)
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

        let expanded = match (&self.parse_as_name, &self.style, field_parser_impls.len()) {
            (Some(name), StructStyle::Unit, _) | (Some(name), _, 0) => quote! {
                let #parser_ident = parser::keyword(#name).padded();
            },
            (Some(name), _, _) => quote! {
                #(#field_parser_impls)*
                let #parser_ident = parser::lparen()
                    .ignore_then(parser::keyword(#name))
                    #(.then(#field_parser_idents))*
                    .then_ignore(parser::rparen())
                    .padded();
            },
            (None, _, _) => quote! {
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

fn generate_pyo3_impl_if_enabled(input: &Item) -> syn::Result<TokenStream> {
    let ident = match input {
        Item::Struct(item_struct) => &item_struct.ident,
        Item::Enum(item_enum) => &item_enum.ident,
        _ => return Ok(TokenStream::new()),
    };

    let is_pyclass = match input {
        Item::Struct(item_struct) => item_struct
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("pyclass")),
        Item::Enum(item_enum) => item_enum
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("pyclass")),
        _ => false,
    };

    if cfg!(feature = "pyo3") && is_pyclass {
        let field_ops: Vec<TokenStream> = vec![];

        // if let syn::Data::Struct(syn::DataStruct {
        //     fields: syn::Fields::Named(named),
        //     ..
        // }) = &input.data
        // {
        //     for field in &named.named {
        //         if let Some(ident) = &field.ident {
        //             let get_ident = format_ident!("get_{}", ident);
        //             let set_ident = format_ident!("set_{}", ident);
        //             let ty = &field.ty;
        //             field_ops.push(quote! {
        //                 #[getter]
        //                 fn #get_ident(&self) -> PyResult<#ty> {
        //                     Ok(self.#ident)
        //                 }

        //                 #[setter]
        //                 fn #set_ident(&mut self, value: #ty) -> PyResult<()> {
        //                     self.#ident = value;
        //                     Ok(())
        //                 }
        //             });
        //         }
        //     }
        // }

        Ok(quote! {
            #[::pyo3::pymethods]
            impl #ident {
                #[new]
                fn new(s: &str) -> Self {
                    let res = Self::parser().parse(s);

                    if let Err(e) = &res {
                        for err in e.into_iter() {
                            parser::PrettyPrintError::pretty_print(&err, s);
                        }
                    }

                    res.expect("parse failed")
                }

                #(#field_ops)*
            }
        })
    } else {
        Ok(TokenStream::new())
    }
}

/// Generate and return the Parsable implementation for a struct.
fn derive_sexpr_impl_struct(
    input: &ItemStruct,
    type_attrs: SexprAttrs,
) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    let type_name = type_ident.to_string();
    let type_name_snake = type_name.to_case(Case::Snake);
    let parser_ident = &format_ident!("{}_parser", type_name_snake);
    let struct_parser_config = StructParserConfig::new(
        parser_ident,
        Some(type_name_snake),
        type_attrs,
        &input.fields,
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

    // Generate the implementation
    let implementation = match &struct_parser_config.style {
        StructStyle::Named => {
            quote! {
                impl<'a> parser::Parsable<'a> for #type_ident {
                    fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|#field_destructuring| Self {
                            #(#field_idents),*
                        }).boxed()
                    }
                }
            }
        }
        StructStyle::Tuple => {
            quote! {
                impl<'a> parser::Parsable<'a> for #type_ident {
                    fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|#field_destructuring| Self(
                            #(#field_idents),*
                        )).boxed()
                    }
                }
            }
        }
        StructStyle::Unit => {
            quote! {
                impl<'a> parser::Parsable<'a> for #type_ident {
                    fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
                        use chumsky::prelude::*;

                        #parser_impl
                        #parser_ident.map(|_| Self).boxed()
                    }
                }
            }
        }
    };

    // Convert back to token stream and return
    Ok(implementation)
}

fn derive_sexpr_impl_enum(input: &ItemEnum, _type_attrs: SexprAttrs) -> syn::Result<TokenStream> {
    let type_ident = &input.ident;
    let type_name = type_ident.to_string();
    let type_name_snake = type_name.to_case(Case::Snake);

    let mut variant_parsers: Vec<TokenStream> = Vec::new();
    let mut variant_parser_idents: Vec<syn::Ident> = Vec::new();

    for variant in &input.variants {
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

    let implementation = if input.variants.is_empty() {
        quote!(
            impl<'a> parser::Parsable<'a> for #type_ident {
                fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
                    use chumsky::prelude::*;

                    chumsky::primitive::filter(|_| false)
                        .map(|_| -> #type_ident {
                            unreachable!("Parser for uninhabited enum should never succeed")
                        })
                        .boxed()
                }
            }
        )
    } else {
        quote!(
           impl<'a> parser::Parsable<'a> for #type_ident {
               fn parser() -> chumsky::BoxedParser<'a, char, Self, chumsky::error::Simple<char>> {
                   use chumsky::prelude::*;

                   #(#variant_parsers)*
                   choice((
                       #(#variant_parser_idents),*
                       ,
                   )).boxed()
               }
           }
        )
    };

    Ok(implementation)
}

fn strip_sexpr_field_attrs_from_enum(item: &mut ItemEnum) {
    item.variants.iter_mut().for_each(|variant| {
        variant.attrs.retain(|attr| !attr.path().is_ident("sexpr"));
        variant.fields.iter_mut().for_each(|field| {
            field.attrs.retain(|attr| !attr.path().is_ident("sexpr"));
        });
    });
}

fn strip_sexpr_field_attrs_from_struct(item: &mut ItemStruct) {
    item.fields.iter_mut().for_each(|field| {
        field.attrs.retain(|attr| !attr.path().is_ident("sexpr"));
    });
}

fn strip_sexpr_field_attrs(item: &mut Item) {
    match item {
        Item::Struct(item_struct) => strip_sexpr_field_attrs_from_struct(item_struct),
        Item::Enum(item_enum) => strip_sexpr_field_attrs_from_enum(item_enum),
        _ => return,
    };
}

#[proc_macro_attribute]
pub fn sexpr(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item_input = parse_macro_input!(item as Item);

    // Handle empty attributes case
    let type_attrs = if attr.is_empty() {
        SexprAttrs::default()
    } else {
        let attr_input = parse_macro_input!(attr as syn::Meta);

        // Parse the attribute arguments into our SexprAttrs structure
        let (name, anonymous) = match SexprAttrs::parse_meta(&attr_input) {
            Ok(result) => result,
            Err(e) => return e.to_compile_error().into(),
        };

        match SexprAttrs::validate(name, anonymous) {
            Ok(attrs) => attrs,
            Err(e) => return e.to_compile_error().into(),
        }
    };

    let parser_impl = match &item_input {
        Item::Struct(item_struct) => derive_sexpr_impl_struct(item_struct, type_attrs)
            .expect("failed to generate sexpr impl for struct"),
        Item::Enum(item_enum) => derive_sexpr_impl_enum(item_enum, type_attrs)
            .expect("failed to generate sexpr impl for enum"),
        _ => panic!("sexpr only works with structs and enums"),
    };

    let pyo3_impl =
        generate_pyo3_impl_if_enabled(&item_input).expect("failed to generate pyo3 impl");

    strip_sexpr_field_attrs(&mut item_input);

    let full_impl = quote! {
        #item_input
        #parser_impl
        #pyo3_impl
    };

    TokenStream::from(full_impl).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_sexpr_attrs_anonymous() {
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr(anonymous)]
        };

        let result = SexprAttrs::try_from(&attrs).unwrap();
        assert!(result.anonymous.unwrap());
        assert!(result.name.is_none());
    }

    #[test]
    fn test_sexpr_attrs_named() {
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr(name = "test_name")]
        };

        let result = SexprAttrs::try_from(&attrs).unwrap();
        assert!(result.anonymous.is_none());
        assert_eq!(result.name.unwrap(), "test_name");
    }

    #[test]
    fn test_sexpr_attrs_no_attributes() {
        let attrs: Vec<syn::Attribute> = vec![];

        let result = SexprAttrs::try_from(&attrs).unwrap();
        assert!(result.anonymous.is_none());
        assert!(result.name.is_none());
    }

    #[test]
    fn test_sexpr_attrs_invalid_combination() {
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr(anonymous)]
            #[sexpr(name = "test")]
        };

        assert!(SexprAttrs::try_from(&attrs).is_err());
    }

    #[test]
    fn test_sexpr_attrs_invalid_format() {
        // Test invalid name value
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr(name = 123)]
        };
        assert!(SexprAttrs::try_from(&attrs).is_err());

        // Test invalid attribute format
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr = "invalid"]
        };
        assert!(SexprAttrs::try_from(&attrs).is_err());

        // Test unknown attribute
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[sexpr(unknown)]
        };
        assert!(SexprAttrs::try_from(&attrs).is_err());
    }
}
