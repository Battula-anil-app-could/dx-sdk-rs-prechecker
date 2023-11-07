// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback:                       1
// Total number of exported functions:  10

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    crowdfunding_erc20
    (
        init => init
        fund => fund
        status => status
        claim => claim
        get_target => target
        get_deadline => deadline
        get_deposit => deposit
        get_erc20_contract_address => erc20_contract_address
        get_total_balance => total_balance
    )
}

dharitri_sc_wasm_adapter::async_callback! { crowdfunding_erc20 }
