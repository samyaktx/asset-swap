use anchor_lang::prelude::*;

#[error_code]
pub enum AssetSwapProgramError {
    #[msg("Math overflow on `u64` value")]
    InvalidArithmetic,

    #[msg("An invalid asset mint address was provided")]
    InvalidAssetKey,

    #[msg("The amount proposed to pay is not great enough for at least 1 returned asset quantity")]
    InvalidSwapNotEnoughPay,

    #[msg("The amount proposed to pay resolves to a receive amount that is greater than the current liquidity")]
    InvalidSwapNotEnoughLiquidity,

    #[msg("The asset proposed to pay is the same asset as the requested asset to receive")]
    InvalidSwapMatchingAssets,

    #[msg("A user cannot propose to pay 0 of an asset")]
    InvalidSwapZeroAmount,
}