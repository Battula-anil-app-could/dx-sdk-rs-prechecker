// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    payable_features
    (
        echo_call_value
        payment_multiple
        payment_array_3
        payable_any_1
        payable_any_2
        payable_any_3
        payable_any_4
        payable_moax_1
        payable_moax_2
        payable_moax_3
        payable_moax_4
        payable_token_1
        payable_token_2
        payable_token_3
        payable_token_4
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}
