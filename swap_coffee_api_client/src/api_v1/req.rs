use serde_derive::Serialize;

pub enum V1Req {
    Assets,
    Pools(Dexes),
}

#[derive(Serialize)]
pub struct Dexes {
    pub dexes: String,
}
