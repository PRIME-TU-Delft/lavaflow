//
// Implement the smoothing_operation macro
//

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields, Ident, Type};
use convert_case::{Case, Casing};
//use miette::{Result, miette};

// This is a custom derive macro SmoothingOperator
#[proc_macro_derive(SmoothingOperation)]
pub fn smoothing_operation_derive(input: TokenStream) -> TokenStream {
	// Construct a Rust-syntax tree of the input TokenStream, that we can manipulate later
	//
	// unwrap is safe, and even desired, to use here, since this code will be evaluated during compile-time.
	// This means that we WANT our code to panic if this unwrap fails.
	let abstract_syntax_tree = syn::parse(input).unwrap();

	// Build the implementation for our trait
	impl_smoothing_operation(&abstract_syntax_tree)
}


fn impl_smoothing_operation(ast: &syn::DeriveInput) -> TokenStream {

	// Extract the name of the struct
	let name = &ast.ident;

	// Turn the name into the right function-call towards the smoother,
	// by removing prefix 'SmoothingOperation' and transforming the name from camel-case to snake-case.
	let function_name_str = name.to_string()
		.replace("SmoothingOperation", "")
		.to_case(Case::Snake);

	// Convert this new name into an identity-format
	let function_name_ident = Ident::new(&function_name_str, name.span());

	// Extract the properties of this struct as fields
	let found_fields = match &ast.data {
		Data::Struct(ref s) => &s.fields,
		_ => panic!("The SmoothingOperation macro can only be applied to structs"),
	};

	// From these fields, we only need the names
	let mut found_field_names: Vec<&Ident> = Vec::new();
	let mut found_field_types: Vec<&Type> = Vec::new();

	// Match on the fields to see whether they meet all the requirements for this macro and
	// then extract the field-names.
	match found_fields {
		Fields::Named(ref s) => for field_name in s.named.iter() {
			found_field_names.push(&field_name.ident.as_ref().unwrap());
			found_field_types.push(&field_name.ty);
		},
		_ => panic!("The SmoothingOperation macro can only be applied to structs that have named fields."),
	}

	//
	// Prepare the additional implementation for ModelConstructionApi struct
	//

	let impl_model_construction_api = quote! {
		impl ModelConstructionApi {
			pub fn #function_name_ident(&mut self, #(#found_field_names:#found_field_types),*) {
				self.enqueue_smoothing_operation(Box::new(#name::new(#(#found_field_names),*)));
			}
		}
	};

	//
	// impl #name
	//

	// Quote the implementation of the constructor
	let impl_constructor = quote! {
		impl #name {
			pub fn new(#(#found_field_names:#found_field_types),*) -> Self {
				Self {
					#(#found_field_names),*
				}
			}
		}
	};


	//
	// impl SmoothingOperation for #name
	//

	// Construct the quote for making the function call to the smoother
	let function_call = quote!{ smoother.#function_name_ident(#(self.#found_field_names),*) };

	// Quote the implementation of the trait for the struct
	let impl_trait_quote = quote! {
		impl SmoothingOperation for #name {
			fn apply(&self, smoother: &mut Smoother) -> Result<()> {
				#function_call
			}
		}
	};


	// Combine both implementations
	let gen = quote! {
		#impl_model_construction_api
		#impl_constructor
		#impl_trait_quote
	};

	gen.into()

}