pub fn implement_imports(
    module: &wasmtime::Module,
    store: *mut wasmtime::Store<webrogue_runtime::Context>,
    engine: &wasmtime::Engine,
) -> anyhow::Result<Vec<wasmtime::Extern>> {
    let funcs = crate::wrapped_funcs::get_funcs(store, engine);

    let mut imports: Vec<wasmtime::Extern> = vec![];

    for import in module.imports() {
        let full_name = format!("{}.{}", import.module(), import.name());
        match import.ty() {
            wasmtime::ExternType::Func(_) => {
                let candidate = funcs
                    .iter()
                    .filter(|(name, _)| {
                        name.to_string() == (import.module().to_string() + "." + import.name())
                    })
                    .next();
                match candidate {
                    None => anyhow::bail!("Missing function import: {}", full_name),
                    Some((_, func)) => imports.push(func.clone().into()),
                };
            }
            wasmtime::ExternType::Global(_) => {
                anyhow::bail!("Missing global import: {}", full_name)
            }
            wasmtime::ExternType::Memory(_) => {
                anyhow::bail!("Missing memory import: {}", full_name)
            }
            wasmtime::ExternType::Table(_) => anyhow::bail!("Missing table import: {}", full_name),
        };
    }

    if imports.len() != module.imports().len() {
        panic!();
    }

    Ok(imports)
}
