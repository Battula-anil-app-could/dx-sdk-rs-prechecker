use super::{
    builtin_func_trait::BuiltinFunction,
    builtin_function_names::*,
    dct_nft::{
        DCTLocalBurn, DCTLocalMint, DCTNftAddQuantity, DCTNftAddUri, DCTNftBurn,
        DCTNftCreate, DCTNftUpdateAttributes,
    },
    general::{ChangeOwner, ClaimDeveloperRewards, SetUsername, UpgradeContract},
    transfer::{DCTMultiTransfer, DCTNftTransfer, DCTTransfer},
    BuiltinFunctionDctTransferInfo,
};
use crate::{
    tx_execution::BlockchainVMRef,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::DctLocalRole,
};

/// Container for builtin function logic.
///
/// Currently has no data, but could conceivably be configurable in the future.
pub struct BuiltinFunctionContainer;

impl BuiltinFunctionContainer {
    /// If the call points to a builtin function, it executes it, otherwise calls the `or_else` closure.
    ///
    /// It also checks that the appropriate roles are set, where applicable.
    pub fn execute_builtin_function_or_else<F, Else>(
        &self,
        vm: &BlockchainVMRef,
        tx_input: TxInput,
        tx_cache: TxCache,
        f: F,
        or_else: Else,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
        Else: FnOnce(TxInput, TxCache, F) -> (TxResult, BlockchainUpdate),
    {
        BuiltinFunctionCall::new(vm, tx_input, tx_cache).execute_or_else(f, or_else)
    }

    /// Provides data on the builtin functions that perform DCT token transfers.
    pub fn extract_token_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        match tx_input.func_name.as_str() {
            DCT_MULTI_TRANSFER_FUNC_NAME => bf_extract_transfers(DCTMultiTransfer, tx_input),
            DCT_NFT_TRANSFER_FUNC_NAME => bf_extract_transfers(DCTNftTransfer, tx_input),
            DCT_TRANSFER_FUNC_NAME => bf_extract_transfers(DCTTransfer, tx_input),
            _ => BuiltinFunctionDctTransferInfo::empty(tx_input),
        }
    }
}

fn bf_extract_transfers<B>(builtin_func: B, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo
where
    B: BuiltinFunction,
{
    builtin_func.extract_dct_transfers(tx_input)
}

/// Syntax helper for the big builtin function match in `execute_or_else`.
/// Thanks to it we do not need to write out the arguments for each match arm.
struct BuiltinFunctionCall<'a> {
    vm: &'a BlockchainVMRef,
    tx_input: TxInput,
    tx_cache: TxCache,
}

impl<'a> BuiltinFunctionCall<'a> {
    pub fn new(vm: &'a BlockchainVMRef, tx_input: TxInput, tx_cache: TxCache) -> Self {
        BuiltinFunctionCall {
            vm,
            tx_input,
            tx_cache,
        }
    }

    pub fn execute_or_else<F, Else>(self, f: F, or_else: Else) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
        Else: FnOnce(TxInput, TxCache, F) -> (TxResult, BlockchainUpdate),
    {
        match self.tx_input.func_name.as_str() {
            DCT_LOCAL_MINT_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::Mint, DCTLocalMint, f)
            },
            DCT_LOCAL_BURN_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::Burn, DCTLocalBurn, f)
            },
            DCT_NFT_CREATE_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::NftCreate, DCTNftCreate, f)
            },
            DCT_NFT_BURN_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::NftBurn, DCTNftBurn, f)
            },
            DCT_NFT_ADD_QUANTITY_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::NftAddQuantity, DCTNftAddQuantity, f)
            },
            DCT_NFT_ADD_URI_FUNC_NAME => {
                self.check_role_and_execute(DctLocalRole::NftAddUri, DCTNftAddUri, f)
            },
            DCT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME => self.check_role_and_execute(
                DctLocalRole::NftUpdateAttributes,
                DCTNftUpdateAttributes,
                f,
            ),

            DCT_MULTI_TRANSFER_FUNC_NAME => self.execute_bf(DCTMultiTransfer, f),
            DCT_NFT_TRANSFER_FUNC_NAME => self.execute_bf(DCTNftTransfer, f),
            DCT_TRANSFER_FUNC_NAME => self.execute_bf(DCTTransfer, f),
            CHANGE_OWNER_BUILTIN_FUNC_NAME => self.execute_bf(ChangeOwner, f),
            CLAIM_DEVELOPER_REWARDS_FUNC_NAME => self.execute_bf(ClaimDeveloperRewards, f),
            SET_USERNAME_FUNC_NAME => self.execute_bf(SetUsername, f),
            UPGRADE_CONTRACT_FUNC_NAME => self.execute_bf(UpgradeContract, f),
            MIGRATE_USERNAME_FUNC_NAME => {
                panic!("builtin function {MIGRATE_USERNAME_FUNC_NAME} not implemented")
            },
            _ => or_else(self.tx_input, self.tx_cache, f),
        }
    }

    fn execute_bf<B, F>(self, builtin_func: B, f: F) -> (TxResult, BlockchainUpdate)
    where
        B: BuiltinFunction,
        F: FnOnce(),
    {
        builtin_func.execute(self.tx_input, self.tx_cache, self.vm, f)
    }

    fn check_role_and_execute<B, F>(
        self,
        role: DctLocalRole,
        builtin_func: B,
        f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        B: BuiltinFunction,
        F: FnOnce(),
    {
        if check_allowed_to_execute(role, &self.tx_input, &self.tx_cache) {
            self.execute_bf(builtin_func, f)
        } else {
            (
                TxResult::from_vm_error("action is not allowed"),
                BlockchainUpdate::empty(),
            )
        }
    }
}

fn check_allowed_to_execute(role: DctLocalRole, tx_input: &TxInput, tx_cache: &TxCache) -> bool {
    let token_identifier = tx_input.args[0].clone();
    let available_roles = tx_cache.with_account_mut(&tx_input.to, |account| {
        account.dct.get_roles(&token_identifier)
    });
    available_roles
        .iter()
        .any(|available_role| available_role.as_slice() == role.name().as_bytes())
}
