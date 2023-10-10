#![no_std]

dharitri_sc::imports!();

#[dharitri_sc::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, dct_token_identifier: MoaxOrDctTokenIdentifier) {
        self.set_contract_dct_token_identifier(&dct_token_identifier);
    }

    #[payable("*")]
    #[endpoint(acceptDctPayment)]
    fn accept_dct_payment(&self) {
        let actual_token_identifier = self.call_value().moax_or_single_dct().token_identifier;
        let expected_token_identifier = self.get_contract_dct_token_identifier();
        require!(
            actual_token_identifier == expected_token_identifier,
            "Wrong dct token"
        );
    }

    #[payable("*")]
    #[endpoint(rejectDctPayment)]
    fn reject_dct_payment(&self) {
        sc_panic!("Rejected")
    }

    // storage

    #[storage_set("dctTokenName")]
    fn set_contract_dct_token_identifier(&self, dct_token_identifier: &MoaxOrDctTokenIdentifier);

    #[view(getdctTokenName)]
    #[storage_get("dctTokenName")]
    fn get_contract_dct_token_identifier(&self) -> MoaxOrDctTokenIdentifier;
}
