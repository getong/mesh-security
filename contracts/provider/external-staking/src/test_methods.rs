use cosmwasm_std::{Coin, Response, StdError, Uint128};
use mesh_apis::converter_api::RewardInfo;
use mesh_apis::ibc::AddValidator;
use sylvia::ctx::ExecCtx;
use sylvia::interface;

/// Interface to work around lack of support for IBC in `cw-multi-test`
/// This interface is for test usage only
#[interface]
pub trait TestMethods {
    type Error: From<StdError>;

    /// Commits a pending stake.
    #[sv::msg(exec)]
    fn test_commit_stake(&self, ctx: ExecCtx, tx_id: u64) -> Result<Response, Self::Error>;

    /// Rollbacks a pending stake.
    #[sv::msg(exec)]
    fn test_rollback_stake(&self, ctx: ExecCtx, tx_id: u64) -> Result<Response, Self::Error>;

    /// Updates the active validator set.
    #[sv::msg(exec)]
    fn test_set_active_validator(
        &self,
        ctx: ExecCtx,
        validator: AddValidator,
        height: u64,
        time: u64,
    ) -> Result<Response, Self::Error>;

    /// Sets validator as `unbonded`.
    #[sv::msg(exec)]
    fn test_remove_validator(
        &self,
        ctx: ExecCtx,
        valoper: String,
        height: u64,
        time: u64,
    ) -> Result<Response, Self::Error>;

    #[sv::msg(exec)]
    fn test_tombstone_validator(
        &self,
        ctx: ExecCtx,
        valoper: String,
        height: u64,
        time: u64,
    ) -> Result<Response, Self::Error>;

    /// Commits a pending unstake.
    #[sv::msg(exec)]
    fn test_commit_unstake(&self, ctx: ExecCtx, tx_id: u64) -> Result<Response, Self::Error>;

    /// Rollbacks a pending unstake.
    #[sv::msg(exec)]
    fn test_rollback_unstake(&self, ctx: ExecCtx, tx_id: u64) -> Result<Response, Self::Error>;

    /// Distribute rewards.
    #[sv::msg(exec)]
    fn test_distribute_rewards(
        &self,
        ctx: ExecCtx,
        validator: String,
        rewards: Coin,
    ) -> Result<Response, Self::Error>;

    /// Batch distribute rewards.
    #[sv::msg(exec)]
    fn test_distribute_rewards_batch(
        &self,
        ctx: ExecCtx,
        denom: String,
        rewards: Vec<RewardInfo>,
    ) -> Result<Response, Self::Error>;

    /// Commits a withdraw rewards transaction.
    #[sv::msg(exec)]
    fn test_commit_withdraw_rewards(
        &self,
        ctx: ExecCtx,
        tx_id: u64,
    ) -> Result<Response, Self::Error>;

    /// Rollbacks a withdraw rewards transaction.
    #[sv::msg(exec)]
    fn test_rollback_withdraw_rewards(
        &self,
        ctx: ExecCtx,
        tx_id: u64,
    ) -> Result<Response, Self::Error>;

    /// Slashes a validator.
    /// This will not perform any check on the validator's state in the validator set, which should
    /// be done before calling this function.
    #[sv::msg(exec)]
    fn test_handle_slashing(
        &self,
        ctx: ExecCtx,
        validator: String,
        slash_amount: Uint128,
    ) -> Result<Response, Self::Error>;
}
