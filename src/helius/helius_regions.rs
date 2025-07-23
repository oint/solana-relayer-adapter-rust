use crate::HeliusEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum HeliusRegionsType {
    SaltLakeCity,
    Newark,
    London,
    Frankfurt,
    Tokyo,
    Singapore,
    Amsterdam,
}

macro_rules! helius_endpoint {
    ($region:ident, $name:expr, $submit:expr, $ping:expr) => {
        HeliusEndpoint {
            relayer: HeliusRegionsType::$region,
            relayer_name: $name,
            submit_endpoint: $submit,
            ping_endpoint: $ping,
        }
    };
}

pub const HELIUS_REGIONS: &[HeliusEndpoint] = &[
    helius_endpoint!(
        SaltLakeCity,
        "Helius-SaltLakeCity",
        "http://slc-sender.helius-rpc.com/fast",
        "slc-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Newark,
        "Helius-Newark",
        "http://ewr-sender.helius-rpc.com/fast",
        "ewr-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        London,
        "Helius-London",
        "http://lon-sender.helius-rpc.com/fast",
        "lon-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Frankfurt,
        "Helius-Frankfurt",
        "http://fra-sender.helius-rpc.com/fast",
        "fra-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Tokyo,
        "Helius-Tokyo",
        "http://tyo-sender.helius-rpc.com/fast",
        "tyo-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Singapore,
        "Helius-Singapore",
        "http://sg-sender.helius-rpc.com/fast",
        "sg-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Amsterdam,
        "Helius-Amsterdam",
        "http://ams-sender.helius-rpc.com/fast",
        "ams-sender.helius-rpc.com"
    ),
];
