use syn::*;

pub fn extract_struct<'a>(ast: &'a mut DeriveInput) -> &'a mut DataStruct {
    if let syn::Data::Struct(ref mut data) = ast.data {
        return data;
    }

    panic!("`{}` is not a struct", ast.ident);
}

pub fn extract_named_fields<'a>(data: &'a mut DataStruct) -> &'a mut FieldsNamed {
    if let Fields::Named(ref mut fields) = data.fields {
        return fields;
    }

    panic!("struct has no named fields");
}
