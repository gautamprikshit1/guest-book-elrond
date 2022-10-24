#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::contract]
pub trait GuestBookContract {
    #[view(getAllMessages)]
    #[storage_mapper("guest_messages")]
    fn guest_messages(&self) -> VecMapper<ManagedBuffer>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn add_message(&self, text: ManagedBuffer) {
        self.guest_messages().push(&text);
    }
}
