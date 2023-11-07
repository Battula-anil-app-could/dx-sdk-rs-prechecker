// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::external_view_init! {}

dharitri_sc_wasm_adapter::external_view_endpoints! {
    multi_contract_features
    (
        external_pure => external_pure
        sample_value_external_get => sample_value_external_get
        sample_value_external_set => sample_value_external_set
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
