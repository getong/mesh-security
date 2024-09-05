mod msg;
mod query;

pub use msg::{ProviderCustomMsg, ProviderMsg, VirtualStakeCustomMsg, VirtualStakeMsg};
pub use query::{
    BondStatusResponse, SlashRatioResponse, TokenQuerier, TotalDelegationResponse,
    VirtualStakeCustomQuery, VirtualStakeQuery,
};

// This is a signal, such that any contract that imports these helpers
// will only run on blockchains that support virtual_staking feature
#[no_mangle]
extern "C" fn requires_virtual_staking() {}
