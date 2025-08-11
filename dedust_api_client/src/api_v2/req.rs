use serde_derive::Serialize;

pub enum V2Req {
    Pools,
    PoolsLite,
    PoolTrades(String),
    RoutingPlan(RoutingPlanParams),
}

/// Expecting addresses in format `workchain_id:hex_hash`
/// Check tests for examples
#[derive(Serialize, Clone)]
pub struct RoutingPlanParams {
    pub from: String,
    pub to: String,
    pub amount: String,
}

impl RoutingPlanParams {
    pub fn new(from: &str, to: &str, amount: &str) -> Self {
        let from = match from {
            "0:0000000000000000000000000000000000000000000000000000000000000000" => "native".to_string(),
            x => format!("jetton:{x}"),
        };
        let to = match to {
            "0:0000000000000000000000000000000000000000000000000000000000000000" => "native".to_string(),
            x => format!("jetton:{x}"),
        };
        Self {
            from,
            to,
            amount: amount.to_string(),
        }
    }
}
