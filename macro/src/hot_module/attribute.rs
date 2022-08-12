use syn::{
    punctuated::Punctuated, spanned::Spanned, token, Error, Lit, LitInt, LitStr, Meta,
    MetaNameValue, NestedMeta, Result,
};

pub(crate) struct HotModuleAttribute {
    pub(crate) lib_name: syn::LitStr,
    pub(crate) lib_dir: syn::LitStr,
    pub(crate) file_watch_debounce_ms: syn::LitInt,
}

// Parses something like `#[hot(name = "lib")]`.
impl syn::parse::Parse for HotModuleAttribute {
    fn parse(stream: syn::parse::ParseStream) -> Result<Self> {
        let mut lib_name = None;
        let mut lib_dir = None;
        let mut file_watch_debounce_ms = None;

        let args = Punctuated::<NestedMeta, token::Comma>::parse_separated_nonempty(stream)?;

        for arg in args {
            match arg {
                NestedMeta::Meta(meta) => {
                    match meta {
                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Str(lit),
                            path,
                            ..
                        }) if path.is_ident("dylib") => {
                            lib_name = Some(lit);
                        }

                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Str(lit),
                            path,
                            ..
                        }) if path.is_ident("lib_dir") => {
                            lib_dir = Some(lit);
                        }

                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Int(lit),
                            path,
                            ..
                        }) if path.is_ident("file_watch_debounce") => {
                            file_watch_debounce_ms = Some(lit);
                        }

                        _ => return Err(Error::new(meta.span(), "unexpected attribute field")),
                    };
                }
                _ => return Err(Error::new(arg.span(), "unexpected attribute value")),
            }
        }

        let lib_name = match lib_name {
            None => {
                return Err(Error::new(
                    stream.span(),
                    r#"missing field "name": add `name = "name_of_library""#,
                ))
            }
            Some(lib_name) => lib_name,
        };

        let lib_dir = match lib_dir {
            None => {
                if cfg!(debug_assertions) {
                    LitStr::new("target/debug", stream.span())
                } else {
                    LitStr::new("target/release", stream.span())
                }
            }
            Some(lib_dir) => lib_dir,
        };

        let file_watch_debounce_ms = match file_watch_debounce_ms {
            None => LitInt::new("500", stream.span()),
            Some(file_watch_debounce_ms) => file_watch_debounce_ms,
        };

        Ok(HotModuleAttribute {
            lib_name,
            lib_dir,
            file_watch_debounce_ms,
        })
    }
}
