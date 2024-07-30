#[derive(Clone, Copy)]
pub enum ValueType {
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
}

impl ValueType {
    pub fn is_64bit(&self) -> bool {
        self.type_infos().0
    }

    pub fn is_unsigned(&self) -> bool {
        self.type_infos().1 .0
    }

    pub fn is_signed(&self) -> bool {
        self.type_infos().1 .1
    }

    pub fn is_float(&self) -> bool {
        self.type_infos().1 .2
    }

    pub fn rust_type_str(&self) -> &str {
        match self {
            ValueType::U32 => "u32",
            ValueType::U64 => "u64",
            ValueType::I32 => "i32",
            ValueType::I64 => "i64",
            ValueType::F32 => "f32",
            ValueType::F64 => "f64",
        }
    }

    pub fn iterator() -> std::slice::Iter<'static, ValueType> {
        static DIRECTIONS: [ValueType; 6] = [
            ValueType::U32,
            ValueType::U64,
            ValueType::I32,
            ValueType::I64,
            ValueType::F32,
            ValueType::F64,
        ];
        DIRECTIONS.iter()
    }

    pub fn from_rust_type_str(rust_type_str: &str) -> Option<Self> {
        ValueType::iterator()
            .filter(|ty| ty.rust_type_str() == rust_type_str)
            .next()
            .cloned()
    }

    fn type_infos(&self) -> (bool, (bool, bool, bool)) {
        match self {
            ValueType::U32 => (false, (true, false, false)),
            ValueType::U64 => (true, (true, false, false)),
            ValueType::I32 => (false, (false, true, false)),
            ValueType::I64 => (true, (false, true, false)),
            ValueType::F32 => (false, (false, false, true)),
            ValueType::F64 => (true, (false, false, true)),
        }
    }
}

#[derive(Clone)]
pub struct Import {
    pub module: String,
    pub func_name: String,
    pub args: Vec<ValueType>,
    pub ret_str: Option<ValueType>,
    pub implementation_func_name: String,
    pub rust_module: String,
}

pub fn fun_name(
    file_content: std::borrow::Cow<str>,
    rust_module: String,
    result: &mut Vec<Import>,
) {
    for line in file_content.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut line = line.to_owned();
        line = line[20..].to_string();
        let dot_pos = line.find(".").unwrap();
        let module = line[..dot_pos].to_string();
        line = line[dot_pos + 1..].to_string();
        // let colon_pos = line.find(":").unwrap();
        let bracket_pos = line.find("(").unwrap();
        let func_name = line[..bracket_pos].to_string();
        line = line[bracket_pos + 1..].to_string();
        let bracket_pos = line.find(")").unwrap();
        let args_str = line[..bracket_pos].to_string();
        line = line[bracket_pos + 6..].to_string();
        let bracket_pos = line.find(")").unwrap();
        let ret_str = line[..bracket_pos].to_string();

        let mut args: Vec<String> = vec![];

        for arg_str in args_str.split(", ") {
            if !arg_str.is_empty() {
                args.push(arg_str.to_string());
            }
        }
        let implementation_func_name = format!("{}::{}", module.clone(), func_name.clone());

        result.push(Import {
            module: module,
            func_name: func_name,
            args: args
                .iter()
                .map(|arg| ValueType::from_rust_type_str(&arg).expect("Unknown type"))
                .collect(),
            ret_str: match ret_str.as_str() {
                "" => None,
                _ => Some(ValueType::from_rust_type_str(&ret_str).expect("Unknown type")),
            },
            implementation_func_name: implementation_func_name,
            rust_module: rust_module.clone(),
        })
    }
}

mod kw {
    syn::custom_keyword!(mutable);
    syn::custom_keyword!(module);
    syn::custom_keyword!(defs);
}

struct Config {
    // pub mutable: bool,
    pub modules: Vec<(String, ModuleConfig)>,
}

pub struct ImportedModule {
    pub module_name: String,
    pub rust_module: String,
    pub funcs: Vec<Import>,
}

pub struct Imports {
    pub modules: Vec<ImportedModule>,
}

impl syn::parse::Parse for Imports {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let contents;
        let _lbrace = syn::braced!(contents in input);
        let fields: syn::punctuated::Punctuated<ConfigField, syn::Token![,]> =
            contents.parse_terminated(ConfigField::parse, syn::Token![,])?;
        let config = Config::build(fields.into_iter(), input.span())?;
        Ok(Imports {
            modules: config
                .modules
                .iter()
                .map(|module_config| {
                    let rust_module = module_config.1.module.clone();
                    let wasm_module = module_config.0.clone();
                    let file_content = match wasm_module.as_str() {
                        "wr_gl" => String::from_utf8_lossy(include_bytes!("../../gl/defs.in")),
                        "wasi_snapshot_preview1" => {
                            String::from_utf8_lossy(include_bytes!("../../wasi/defs.in"))
                        }
                        _ => panic!("unknown module"),
                    };
                    let mut imports: Vec<Import> = vec![];
                    fun_name(file_content, rust_module.clone(), &mut imports);
                    ImportedModule {
                        module_name: wasm_module,
                        rust_module: rust_module,
                        funcs: imports,
                    }
                })
                .collect(),
        })
    }
}

impl Config {
    pub fn build(
        fields: impl Iterator<Item = ConfigField>,
        err_loc: proc_macro2::Span,
    ) -> syn::Result<Self> {
        let mut mutable = None;
        let mut modules = vec![];
        for f in fields {
            match f {
                ConfigField::Mutable(c) => {
                    if mutable.is_some() {
                        return Err(syn::Error::new(err_loc, "duplicate `mutable` field"));
                    }
                    mutable = Some(c);
                }
                ConfigField::Module(name, module_config) => modules.push((name, module_config)),
            }
        }
        Ok(Config {
            // mutable: mutable.unwrap_or(true),
            modules,
        })
    }
}

enum ConfigField {
    Mutable(bool),
    Module(String, ModuleConfig),
}

impl syn::parse::Parse for ConfigField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::mutable) {
            input.parse::<kw::mutable>()?;
            input.parse::<syn::Token![:]>()?;
            Ok(ConfigField::Mutable(input.parse::<syn::LitBool>()?.value))
        } else if lookahead.peek(syn::LitStr) {
            let s = input.parse::<syn::LitStr>()?;
            input.parse::<syn::Token![:]>()?;
            Ok(ConfigField::Module(s.value(), input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct ModuleConfig {
    // pub defs: Option<String>,
    pub module: String,
}

impl syn::parse::Parse for ModuleConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let contents;
        let _lbrace = syn::braced!(contents in input);
        let fields: syn::punctuated::Punctuated<ModuleConfigField, syn::Token![,]> =
            contents.parse_terminated(ModuleConfigField::parse, syn::Token![,])?;
        Ok(ModuleConfig::build(fields.into_iter(), input.span())?)
    }
}

enum ModuleConfigField {
    Module(String),
    Defs(String),
}

impl syn::parse::Parse for ModuleConfigField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::defs) {
            input.parse::<kw::defs>()?;
            input.parse::<syn::Token![:]>()?;
            Ok(ModuleConfigField::Defs(
                input.parse::<syn::LitStr>()?.value(),
            ))
        } else if lookahead.peek(kw::module) {
            input.parse::<kw::module>()?;
            input.parse::<syn::Token![:]>()?;
            let module_paths: syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]> =
                input.parse_terminated(syn::PathSegment::parse, syn::Token![::])?;
            let module_paths = module_paths
                .into_iter()
                .map(|segment| segment.ident.to_string());
            let module_path = module_paths.collect::<Vec<_>>().join("::");
            Ok(ModuleConfigField::Module(module_path))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ModuleConfig {
    pub fn build(
        fields: impl Iterator<Item = ModuleConfigField>,
        err_loc: proc_macro2::Span,
    ) -> syn::Result<Self> {
        let mut defs = None;
        let mut module = None;
        for f in fields {
            match f {
                ModuleConfigField::Defs(path) => {
                    if defs.is_some() {
                        return Err(syn::Error::new(err_loc, "duplicate `defs` field"));
                    }
                    defs = Some(path);
                }
                ModuleConfigField::Module(name) => {
                    if module.is_some() {
                        return Err(syn::Error::new(err_loc, "duplicate `module` field"));
                    }
                    module = Some(name);
                }
            }
        }
        Ok(ModuleConfig {
            // defs,
            module: module.ok_or_else(|| syn::Error::new(err_loc, "`module` field required"))?,
        })
    }
}

pub use syn::parse_macro_input;
