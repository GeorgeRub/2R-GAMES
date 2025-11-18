use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

#[proc_macro_derive(EntityMacro)]
pub fn entity_macro(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a syn AST (DeriveInput)
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the type we are deriving for
    let struct_name = &input.ident;
    println!("Macros name {:?}", struct_name);

    let fields = match input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => panic!("PrintFields can be used only on structs"),
    };

    // --- Генерация serialize() ---
    let serialize_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let key = name.to_string();

        match &f.ty {
            // Option<bool>
            Type::Path(tp)
                if tp.path.segments.last().unwrap().ident == "Option"
                    && quote!(#f.ty).to_string().contains("bool") =>
            {
                quote! {
                    if let Some(value) = &self.#name {
                        item.insert(#key.to_string(), AttributeValue::Bool(*value));
                    }
                }
            }
            // bool
            Type::Path(tp) if tp.path.segments.last().unwrap().ident == "bool" => {
                quote! {
                    item.insert(#key.to_string(), AttributeValue::Bool(self.#name));
                }
            }
            // Option<String>
            Type::Path(tp)
                if tp.path.segments.last().unwrap().ident == "Option"
                    && quote!(#f.ty).to_string().contains("String") =>
            {
                quote! {
                    if let Some(value) = &self.#name {
                        item.insert(#key.to_string(), AttributeValue::S(value.clone()));
                    }
                }
            }
            // String
            Type::Path(tp) if tp.path.segments.last().unwrap().ident == "String" => {
                quote! {
                    item.insert(#key.to_string(), AttributeValue::S(self.#name.clone()));
                }
            }
            _ => {
                quote! {} // игнорируем неизвестные типы
            }
        }
    });

    // --- Генерация deserialize() ---
    let deserialize_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let key = name.to_string();

        match &f.ty {
            Type::Path(tp) => {
                let last = tp.path.segments.last().unwrap();

                // Option<T>
                if last.ident == "Option" {
                    // Get T
                    if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                        let inner = &args.args[0];

                        let inner_str = quote!(#inner).to_string();

                        return if inner_str == "String" {
                            quote! {
                                #name: get_string_from_item_to_option(#key, &items)
                            }
                        } else if inner_str == "bool" {
                            quote! {
                                #name: get_bool_from_item_to_option(#key, &items)
                            }
                        } else {
                            quote! { #name: None }
                        };
                    }
                }

                // String
                if last.ident == "String" {
                    return quote! {
                        #name: get_string_from_item(#key, &items)
                    };
                }

                // bool
                if last.ident == "bool" {
                    return quote! {
                        #name: get_bool_from_item(#key, &items)
                    };
                }

                // Unknown type:
                quote! { #name: Default::default() }
            }

            _ => quote! { #name: Default::default() },
        }
    });

    // Create the generated implementation using quote!
    let expanded = quote! {
        impl Entity for #struct_name {
            type Item = #struct_name;

            fn serialize_object(&self) -> Option<std::collections::HashMap<String, AttributeValue>> {
                let mut item = std::collections::HashMap::new();
                #(#serialize_fields);*
                Some(item)
            }

            fn deserialize_object(items: std::collections::HashMap<String, AttributeValue>) -> Result<Self::Item, Error> {
                Ok(Self {
                    #(#deserialize_fields),*
                })
            }

           async fn save(&self) -> Result<Self::Item, Error> {
                let item = self.serialize_object().expect("Failed to serialize struct");
                let table_name = TABLES_NAMES.lock().unwrap().get(&"USER_TABLE_NAME".to_string()).unwrap().clone();
                let client = get_connection_to_db().await;
                let _ =  client
                    .put_item()
                    .table_name(table_name)
                    .set_item(Some(item))
                    .send()
                    .await;
                Ok(self.clone())
            }

            fn update(&self) -> Result<Self::Item, Error> {
                todo!()
            }

            fn delete(&self) -> Result<(), Error> {
                todo!()
            }
        }
    };

    // Convert into TokenStream and return it
    TokenStream::from(expanded)
}
