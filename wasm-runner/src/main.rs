use chrono::Datelike;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue, Signature, ValueType};

fn print_usage() {
    println!("Usage: wasm-runner path/to/file.wasm");
}

fn main() {
    // read path to the .wasm file from arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        print_usage();
        return;
    }
    let wasm_file_path = std::path::Path::new(&args[1]);

    // open and read the file with compiled WebAssembly
    let mut wasm_binary = vec![];
    std::fs::File::open(wasm_file_path)
        .expect("failed to open WASM file")
        .read_to_end(&mut wasm_binary)
        .expect("failed to read WASM file");

    // deserialize and instantiate a WebAssembly module using wasmi
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // invoke a function from the WebAssembly module and verify the expected return value
    let year_to_check = 2022;
    let return_value = instance
        .invoke_export(
            "is_leap_year",
            &[RuntimeValue::I32(year_to_check)],
            &mut NopExternals,
        )
        .expect("failed to execute export");

    println!("Return value: {:?}", return_value);

    match return_value {
        Some(RuntimeValue::I32(1)) => println!("Year {} is a leap year", year_to_check),
        Some(RuntimeValue::I32(0)) => println!("Year {} is not a leap year", year_to_check),
        Some(_) => panic!("is_leap_year returned a value of unexpected type"),
        None => panic!("is_leap_year did not return a value"),
    };
}

fn get_current_year() -> i32 {
    chrono::Local::now().year()
}

// index value used to identify the get_current_value.
// Can be any number, as long as it is consistent between the TimeProvider and
// the TimeProviderResolver.
const GET_CURRENT_YEAR_INDEX: usize = 0;

struct TimeProvider {}

impl wasmi::Externals for TimeProvider {
    fn invoke_index(
        &mut self,
        index: usize,
        _args: wasmi::RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, wasmi::Trap> {
        match index {
            GET_CURRENT_YEAR_INDEX => {
                let year = get_current_year();
                Ok(Some(RuntimeValue::I32(year)))
            }
            _ => panic!("unknown function index"),
        }
    }
}

struct TimeProviderResolver {}

impl wasmi::ModuleImportResolver for TimeProviderResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &wasmi::Signature,
    ) -> Result<wasmi::FuncRef, wasmi::Error> {
        // identify the desired function by name and return its signature
        // and the numerical index
        match field_name {
            "get_current_year" => Ok(wasmi::FuncInstance::alloc_host(
                Signature::new(&[][..], Some(ValueType::I32)),
                GET_CURRENT_YEAR_INDEX,
            )),
            _ => Err(wasmi::Error::Instantiation(
                "unknown host function".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_year() {
        assert_eq!(get_current_year(), 2022);
    }
}
