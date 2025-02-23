use fvm_sdk as sdk;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// Placeholder invoke for testing
#[no_mangle]
pub fn invoke(_: u32) -> u32 {
    // Conduct method dispatch. Handle input parameters and return data.
    sdk::vm::abort(
        fvm_shared::error::ExitCode::FIRST_USER_EXIT_CODE,
        Some("sample abort"),
    )
}
