extern crate proc_macro;
use crate::util::has_default_impl;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod util;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};
use util::extract_table_name;

/// Derives the `Table` trait for a struct.
///
/// This procedural macro generates implementations of the `Table` trait for a struct.
/// The `Table` trait provides methods for working with database tables.
///
/// # Example
///
/// ```rust
/// use njord_derive::Table;
/// use njord::table::Table;
/// #[derive(Table)]
/// struct TableA {
///     id: usize,
///     name: String,
///     price: f64,
///     in_stock: bool
/// }
///
/// #[derive(Table)]
/// struct TableB {
///     id: usize,
///     name: String,
///     price: f64,
///     in_stock: bool,
///     table: TableA
/// }
/// ```
///
/// This macro will generate implementations the `Table` trait
/// based on the struct's field names and types.
#[proc_macro_derive(Table, attributes(table_name))]
pub fn table_derive(input: TokenStream) -> TokenStream {
    let cloned_input = input.clone();
    let derive_input: syn::DeriveInput = parse_macro_input!(cloned_input);
    let DeriveInput {
        ident, data, attrs, ..
    } = derive_input.clone();

    let table_name = extract_table_name(&attrs);

    println!("{}", table_name);

    let mut name_stream = TokenStream2::default();
    let mut columns_stream = TokenStream2::default();
    let mut column_fields_stream = TokenStream2::default();
    let mut column_values_stream = TokenStream2::default();
    let mut set_column_values_stream = TokenStream2::default();

    let mut display_impl = TokenStream2::default();
    let mut from_str_impl = TokenStream2::default();

    let mut default_impl = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let field_names = named.iter().map(|f| &f.ident);
            let field_names_clone2 = field_names.clone();
            let field_names_clone3 = field_names.clone();
            let field_names_clone4 = field_names.clone();
            let field_types = named.iter().map(|f| &f.ty);
            let field_types_clone = named.iter().map(|f| &f.ty);
            let field_values = named.iter().map(|f| {
                let field_name = &f.ident;
                quote! { self.#field_name.to_string() }
            });

            // implement the std::fmt::Display trait
            display_impl.extend(quote! {
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.get_name())?;
                        for (name, value) in self.get_column_fields().iter().zip(self.get_column_values()) {
                            write!(f, ", {}: {}", name, value)?;
                        }
                        Ok(())
                    }
                }
            }); // display_impl

            // implement the std::str::FromStr trait
            from_str_impl.extend(quote! {
                impl std::str::FromStr for #ident {
                    type Err = std::string::ParseError;

                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        let parts: Vec<&str> = s.split(',').map(|s| s.trim()).collect();

                        // create a hashmap to store column name-value pairs
                        let mut column_values = std::collections::HashMap::new();

                        // iterate over parts and extract column name-value pairs
                        for part in parts {
                            let pair: Vec<&str> = part.split(':').map(|s| s.trim()).collect();
                            if pair.len() == 2 {
                                let name = pair[0];
                                let value = pair[1];
                                column_values.insert(name.to_string(), value.to_string());
                            }
                        }

                        let mut instance = Self::default();

                        // set column values based on the parsed values
                        for (name, value) in column_values.iter() {
                            instance.set_column_value(name, value);
                        }

                        Ok(instance)
                    }
                }
            }); // from_str_impl

            // implement the get_name() function
            name_stream.extend::<TokenStream2>(quote! {
                fn get_name(&self) -> &str {
                    stringify!(#table_name)
                }
            }); // name_stream

            // implement the get_columns() function
            columns_stream.extend(quote! {
                fn get_columns(&self) -> std::collections::HashMap<String, String> {
                    let mut columns = std::collections::HashMap::new();
                    #(
                        let column_type = match stringify!(#field_types) {
                            "i64" | "i32" | "i16" | "i8" | "u64" | "u32" | "u16" | "u8" | "usize" => "INTEGER",
                            "String" => "TEXT",
                            "f64" | "f32" => "REAL",
                            "Vec<u8>" => "BLOB",
                            "Option<i64>" | "Option<i32>" | "Option<i16>" | "Option<i8>" | "Option<u64>" | "Option<u32>" | "Option<u16>" | "Option<u8>" | "Option<usize>" => "INTEGER",
                            "Option<String>" => "TEXT",
                            "Option<f64>" | "Option<f32>" => "REAL",
                            "Option<Vec<u8>>" => "BLOB",
                            "bool" => "TEXT",

                            _ => {
                                eprintln!("Warning: Unknown data type for column '{}'", stringify!(#field_names));
                                "UNKNOWN_TYPE"
                            }
                        };
                        columns.insert(
                            stringify!(#field_names).to_string(),
                            column_type.to_string(),
                        );
                    )*
                    columns
                }
            }); // columns_stream

            // implement the get_column_fields() function
            column_fields_stream.extend(quote! {
                fn get_column_fields(&self) -> Vec<String> {
                    vec![#(stringify!(#field_names_clone2.clone()).to_string()),*]
                }
            }); // column_fields_stream

            // implement the get_column_values() function
            column_values_stream.extend(quote! {
                fn get_column_values(&self) -> Vec<String> {
                    vec![#(#field_values),*]
                }
            }); // column_values_stream

            set_column_values_stream.extend(quote! {
                fn set_column_value(&mut self, column: &str, value: &str) {
                    match column {
                        #(
                            stringify!(#field_names_clone3) => {
                                if let Ok(val) = value.parse::<#field_types_clone>() {
                                    self.#field_names_clone3 = val;
                                } else {
                                    eprintln!("Error: Failed to convert value for column '{}'", column);
                                }
                            }
                        )*

                        _ => eprintln!("Warning: Unknown column '{}'", column),
                    }
                }
            }); // set_column_values_stream

            // If Default trait is not implemented, generate an implementation
            default_impl = if !has_default_impl(&derive_input) {
                quote! {
                    impl Default for #ident {
                        fn default() -> Self {
                            Self {
                                #(
                                    #field_names_clone4: Default::default(),
                                )*
                            }
                        }
                    }
                }
            } else {
                TokenStream2::new()
            }; // default_impl
        }
    };

    let output = quote! {
        impl Table for #ident {
            #name_stream
            #columns_stream
            #column_fields_stream
            #column_values_stream
            #set_column_values_stream
        }

        #default_impl
        #display_impl
        #from_str_impl
    };

    output.into()
}
