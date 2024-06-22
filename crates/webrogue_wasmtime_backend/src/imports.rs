use webrogue_runtime::{
    imported_function_defenition::ImportedFunctionContext,
    imported_functions_impl::get_function_table, raw_value::RawValue, value_type::ValueType,
};

pub fn implement_imports(
    module: &wasmtime::Module,
    store: *mut wasmtime::Store<ImportedFunctionContext>,
    engine: &wasmtime::Engine,
) -> anyhow::Result<Vec<wasmtime::Extern>> {
    let mut imports: Vec<wasmtime::Extern> = vec![];

    for import in module.imports() {
        let full_name = format!("{}.{}", import.module(), import.name());
        match import.ty() {
            wasmtime::ExternType::Func(_) => {
                let candidates = get_function_table();
                let candidate = candidates
                    .iter()
                    .filter(|implementation| {
                        implementation.0.module == import.module()
                            && implementation.0.name == import.name()
                    })
                    .next();
                match candidate {
                    None => anyhow::bail!("Missing function import: {}", full_name),
                    Some(i) => {
                        let func = i.1;
                        let defenition = i.0.clone();
                        fn convert_type(ty: &ValueType) -> wasmtime::ValType {
                            match ty {
                                ValueType::F32 => wasmtime::ValType::F32,
                                ValueType::F64 => wasmtime::ValType::F64,
                                ValueType::I32 => wasmtime::ValType::I32,
                                ValueType::I64 => wasmtime::ValType::I64,
                            }
                        }
                        imports.push(
                            wasmtime::Func::new(
                                unsafe { &mut *store },
                                wasmtime::FuncType::new(
                                    &engine,
                                    defenition.params.iter().map(convert_type),
                                    defenition.results.iter().map(convert_type),
                                ),
                                move |mut caller, params, results| {
                                    let context = caller.data_mut();
                                    for (pos, param) in params.iter().enumerate() {
                                        context.params[pos] = match param {
                                            wasmtime::Val::I32(v) => RawValue { i32: *v },
                                            wasmtime::Val::I64(v) => RawValue { i64: *v },
                                            wasmtime::Val::F32(v) => RawValue { u32: *v },
                                            wasmtime::Val::F64(v) => RawValue { u64: *v },
                                            _ => anyhow::bail!("unknown Val in {}", full_name),
                                        }
                                    }
                                    func(context);
                                    for (pos, (result, ty)) in results
                                        .iter_mut()
                                        .zip(defenition.results.iter())
                                        .enumerate()
                                    {
                                        let webrogue_result = context.results[pos];
                                        *result = match ty {
                                            ValueType::F32 => {
                                                wasmtime::Val::F32(unsafe { webrogue_result.u32 })
                                            }
                                            ValueType::F64 => {
                                                wasmtime::Val::F64(unsafe { webrogue_result.u64 })
                                            }
                                            ValueType::I32 => {
                                                wasmtime::Val::I32(unsafe { webrogue_result.i32 })
                                            }
                                            ValueType::I64 => {
                                                wasmtime::Val::I64(unsafe { webrogue_result.i64 })
                                            }
                                        };
                                    }
                                    Ok(())
                                },
                            )
                            .into(),
                        )
                    }
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
