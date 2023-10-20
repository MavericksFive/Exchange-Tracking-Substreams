use substreams::scalar::BigInt;

pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}

pub fn decimals_per_token_id(id: &BigInt) -> u64{
    let id_u64 = id.to_u64();

    match id_u64 {
        0_u64 => {8}, // TOKEN ID: 0, ADDRESS : 0xEB4C2781e4ebA804CE9a9803C67d0893436bB27D, decimals : 8
        1_u64 => {8}, // TOKEN ID: 1, ADDRESS : 0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599, decimals : 8
        2_u64 => {18}, // TOKEN ID: 2, ADDRESS : 0xfE18be6b3Bd88A2D2A7f928d00292E7a9963CfC6, decimals : 16
        _ => {0},
    }
}