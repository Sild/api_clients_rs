use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxId {
    pub lt: i64,
    pub hash: String,
    pub contract_address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Amm(AmmAction),
    Farm(FarmAction),
    Other(OtherAction),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum AmmAction {
    Swap(
        ActionData<
            DexMajorVersion,
            SwapData,
            ActionStatus<SwapResult>,
        >
    ),
    ProvideLiquidity(
        WithEffects<
            ActionData<
                DexMajorVersion,
                ProvideLiquidityData,
                ActionStatus<ProvideLiquidityResult>
            >,
            ProvideLiquidityEffect
        >
    ),
    DirectAddLiquidity(
        ActionData<
            DexMajorVersion,
            DirectAddLiquidityData,
            ActionStatus<CbAddLiquidityResult>
        >
    ),
    JettonBurn(
        ActionData<
            DexMajorVersion,
            BurnData,
            ActionStatus<BurnNotificationResult>
        >
    ),
    CollectFees(
        ActionData<
            DexMajorVersion,
            CollectFeesData,
            ActionStatus<CollectFeesResult>
        >
    ),
    WithdrawFee(
        ActionData<
            DexMajorVersion,
            WithdrawFeesData,
            ActionStatus<WithdrawFeeResult>
        >
    ),
    RefundLiquidity(
        ActionData<
            DexMajorVersion,
            RefundLiquidityData,
            ActionStatus<RefundResult>
        >
    ),
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum FarmAction {
    FarmDeposit(
        ActionData<
            FarmMajorVersion,
            FarmDepositData,
            ActionStatus<FarmMinterStakeResult>
        >
    ),
    FarmWithdraw(
        WithEffects<
            ActionData<
                FarmMajorVersion,
                FarmWithdrawData,
                ActionStatus<FarmNftUnstakeResult>
            >,
            FarmWithdrawEffect
        >
    ),
    FarmClaimRewards(
        WithEffects<
            ActionData<
                FarmMajorVersion,
                FarmClaimRewardsData,
                ActionStatus<FarmNftClaimRewardsResult>
            >,
            FarmClaimRewardsEffect
        >
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum OtherAction {
    Other(ActionData<Blank, OtherTxPayload, UnknownActionStatus>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionData<Version, Payload, Result> {
    pub contract_major_version: Option<Version>,
    pub destination: Option<String>,
    pub tx_payload: Payload,
    pub status: Result,
    pub prev_index: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithEffects<T, Effect> {
    #[serde(flatten)]
    pub data: T,

    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum ActionStatus<Result> {
    Completed(CompletedActionStatus<Result>),
    Pending,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletedActionStatus<Result> {
    pub tx_id: TxId,
    pub success: bool,

    #[serde(flatten)]
    pub data: Result,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FarmMajorVersion {
    V1,
    V2,
    V3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DexMajorVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwapData {
    // Common fields
    pub opcode: u32,
    pub query_id: u64,
    pub ask_jetton_wallet_address: String,
    pub min_ask_amount: String,
    pub referral_address: Option<String>,

    // V1-specific fields
    pub user_wallet_address: Option<String>,

    // V2-specific fields
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
    pub deadline: Option<u64>,
    pub receiver_address: Option<String>,
    pub dex_forward_gas_amount: Option<String>,
    pub forward_gas_amount: Option<String>,

    pub referral_val: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum SwapResult {
    #[serde(rename = "swap_ok")]
    Ok {
        ask_jetton_wallet: String,
        ask_jetton: String,
        ask_jetton_amount: String,
        offer_jetton_wallet: String,
        offer_jetton: String,
        offer_jetton_amount: String,
        fwd_gas: Option<String>,
        is_reversed: bool,
    },
    #[serde(rename = "swap_ok_ref")]
    OkRef {
        ask_jetton_wallet: String,
        ask_jetton: Option<String>,
        ask_jetton_amount: String,
        offer_jetton_wallet: String,
        offer_jetton: Option<String>,
        offer_jetton_amount: String,
        ref_offer_jetton_wallet: String,
        ref_ask_jetton_wallet: String,
        ref_ask_amount: String,
        fwd_gas: Option<String>,
        is_reversed: bool,
    },
    #[serde(rename = "swap_refund_no_liq")]
    RefundNoLiquidity {
        ask_jetton_wallet: String,
        ask_amount: String,
    },
    #[serde(rename = "swap_refund_reserve_err")]
    RefundReserveError {
        ask_jetton_wallet: String,
        ask_amount: String,
    },
    #[serde(rename = "swap_refund_slippage")]
    RefundSlippageError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_pool_locked")]
    RefundPoolLockedError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_fee_out_of_bounds")]
    RefundSwapFeeOutOfBoundError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_tx_expired")]
    RefundTxExpiredError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_zero_output_err")]
    RefundZeroOutputError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_caller_err")]
    RefundInvalidCallerError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_insufficient_gas_err")]
    RefundInsufficientGasError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_wrong_address_err")]
    RefundWrongAddressError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_low_liquidity_err")]
    RefundLowLiquidityError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_math_err")]
    RefundMathError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_amount_err")]
    RefundInvalidAmountError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_call_err")]
    RefundInvalidCallError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_fee_out_range_err")]
    RefundFeeOutRangeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_invalid_token_err")]
    RefundInvalidTokenError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_empty_not_allowed_err")]
    RefundEmptyNotAllowedError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_max_in_ratio_err")]
    RefundMaxInRatio {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_empty_cell_ratio_err")]
    RefundEmptyCellRatio {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_cell_underflow_err")]
    RefundCellUnderflowError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_wrong_workchain_err")]
    RefundWrongWorkchainError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_refund_0_out")]
    RefundZeroOutError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_x_out_of_bounds")]
    XOutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_y_out_of_bounds")]
    YOutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_product_out_of_bounds")]
    ProductOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_invalid_exponent")]
    InvalidExponentError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_out_of_bounds")]
    OutOfBoundsError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_zero_base")]
    ZeroBaseError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_negative_power")]
    NegativePowerError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_zero_division")]
    ZeroDivisionError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_invalid_input")]
    InvalidInputError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_cmplement_out_of_range")]
    CmplmentOutOfRangeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_not_converge_range")]
    NotConvergeError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "swap_fd_zero")]
    DfZeroError {
        offer_jetton_wallet: String,
        offer_amount: String,
    },
    #[serde(rename = "bounced")]
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvideLiquidityData {
    // Common fields
    pub opcode: u32,
    pub query_id: u64,
    pub router_wallet_address: String,
    pub min_lp_out: String,
    // V2-specific fields
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
    pub deadline: Option<u64>,
    pub receiver_address: Option<String>,
    pub both_positive: Option<u8>,
    pub dex_forward_gas_amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ProvideLiquidityResult {
    #[serde(rename = "provide_liquidity_ok")]
    AddLiquidity {
        amount0_out: String,
        amount1_out: String,
        min_lp_out: String,
        fwd_amount: Option<String>,
        /// Owner of new liquidity tokens
        to_address: Option<String>,
        refund_address: Option<String>,
        excess_address: Option<String>,
    },
    Bounce,
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum ProvideLiquidityEffect {
    Never,
    CbAddLiquidity(EffectData<CbAddLiquidityResult>),
}

impl Default for ProvideLiquidityEffect {
    fn default() -> Self {
        ProvideLiquidityEffect::Never
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum CbAddLiquidityResult {
    #[serde(rename = "add_liquidity_ok")]
    AddLiquidity {
        /// The amount of 0th asset provided
        amount0_out: String,
        /// The amount of 1st asset provided
        amount1_out: String,
        /// Minimal LP token amount to provide
        min_lp_out: String,
        /// Optional destination address
        to_address: Option<String>,
        /// Optional forwarded amount
        fwd_amount: Option<String>,
        /// Optional refund address
        refund_address: Option<String>,
        /// Optional excess address
        excess_address: Option<String>,
    },
    #[serde(rename = "mint_lp_ok")]
    MintLp {
        lp_out: String,
    },
    Empty,
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectData<Result> {
    pub status: ActionStatus<Result>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum UnknownActionStatus {
    Completed {},
    Pending,
    Aborted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectAddLiquidityData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount_0: String,
    pub amount_1: String,
    pub min_lp_to_mint: String,

    // V2-specific fields
    pub dex_forward_gas_amount: Option<String>,
    pub user_wallet_address: Option<String>,
    pub refund_address: Option<String>,
    pub excesses_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BurnData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount: String,
    pub response_destination: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum BurnNotificationResult {
    #[serde(rename = "burn_ok")]
    Ok {
        from_address: String,
        amount0_out: String,
        amount1_out: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectFeesData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WithdrawFeesData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefundLiquidityData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmDepositData {
    pub opcode: u32,
    pub query_id: u64,
    pub owner_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmWithdrawData {
    pub opcode: u32,
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmClaimRewardsData {
    pub opcode: u32,
    pub query_id: u64,
    pub claim_all: Option<bool>,
    pub pool_index: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum CollectFeesResult {
    #[serde(rename = "collect_fees_ok")]
    Ok {
        protocol_fee_address: String,
        collected_token0_protocol_fee: String,
        collected_token1_protocol_fee: String,
    },

    #[serde(rename = "collect_fees_ok_with_rewards")]
    OkWithReward {
        protocol_fee_address: String,
        collected_token0_protocol_fee: String,
        collected_token1_protocol_fee: String,
        sender_address: String,
        reward0: String,
        reward1: String,
    },

    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum WithdrawFeeResult {
    Ok {
        amount_out: String,
        token_address: String,
        to_address: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum RefundResult {
    #[serde(rename = "refund_ok")]
    Ok {
        user_address: String,
        amount0_out: String,
        router_jetton0_address: String,
        amount1_out: String,
        router_jetton1_address: String,
    },
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum FarmMinterStakeResult {
    #[serde(rename = "farm_minter_stake_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum FarmNftClaimRewardsResult {
    #[serde(rename = "farm_nft_claim_rewards_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum FarmNftUnstakeResult {
    #[serde(rename = "farm_nft_unstake_ok")]
    Ok {},
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum FarmWithdrawEffect {
    Never,
    JettonTransfer(EffectData<JettonTransferResult>),
}

impl Default for FarmWithdrawEffect {
    fn default() -> Self {
        FarmWithdrawEffect::Never
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "@type", rename_all = "snake_case")]
pub enum FarmClaimRewardsEffect {
    Never,
    JettonTransfer(EffectData<JettonTransferResult>),
}

impl Default for FarmClaimRewardsEffect {
    fn default() -> Self {
        FarmClaimRewardsEffect::Never
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "exit_code", rename_all = "snake_case")]
pub enum JettonTransferResult {
    Ok(JettonTransferData),
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JettonTransferData {
    pub opcode: u32,
    pub query_id: u64,
    pub amount: String,
    pub destination: String,
    pub response_destination: String,
    pub forward_ton_amount: String,
    pub jetton_master: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OtherTxPayload {
    #[serde(rename = "unknown")]
    Invalid { opcode: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blank;