// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::external_view_init! {}

dharitri_sc_wasm_adapter::external_view_endpoints! {
    abi_tester
    (
        external_view => external_view
        payable_any_token => payable_any_token
        label_a => label_a
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
