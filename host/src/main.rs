use hyperlight_host::{GuestBinary, MultiUseSandbox, UninitializedSandbox};

fn main() -> hyperlight_host::Result<()> {
    let guest_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "../guest/target/x86_64-hyperlight-none/release/guest".into());

    // Create a sandbox with the guest binary
    let uninitialized_sandbox = UninitializedSandbox::new(GuestBinary::FilePath(guest_path), None)?;

    // Initialize sandbox
    let mut sandbox: MultiUseSandbox = uninitialized_sandbox.evolve()?;

    // Call the guest's PrintOutput function
    let message = "Hello, World! I am executing inside of a VM :)\n".to_string();
    sandbox.call::<i32>("PrintOutput", message)?;

    Ok(())
}
