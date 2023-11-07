// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dharitri_price_aggregator_sc
    (
        init => init
        addOracles => add_oracles
        removeOracles => remove_oracles
        submit => submit
        submitBatch => submit_batch
        latestRoundData => latest_round_data
        latestPriceFeed => latest_price_feed
        latestPriceFeedOptional => latest_price_feed_optional
        setSubmissionCount => set_submission_count
        getOracles => get_oracles
        setPairDecimals => set_pair_decimals
        getPairDecimals => get_pair_decimals
        submission_count => submission_count
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
        stake => stake
        unstake => unstake
        voteSlashMember => vote_slash_member
        slashMember => slash_member
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
