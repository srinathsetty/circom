use num_bigint::BigInt;

const P_STR: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989";

pub struct UsefulConstants {
    p: BigInt,
}

impl Clone for UsefulConstants {
    fn clone(&self) -> Self {
        UsefulConstants { p: self.p.clone() }
    }
}
impl Default for UsefulConstants {
    fn default() -> Self {
        UsefulConstants { p: BigInt::parse_bytes(P_STR.as_bytes(), 10).expect("can not parse p") }
    }
}

impl UsefulConstants {
    pub fn new() -> UsefulConstants {
        UsefulConstants::default()
    }
    pub fn get_p(&self) -> &BigInt {
        &self.p
    }
}
