/*
    AES - Key Expansion - Step - Get Round Constant
*/
pub fn rc(round: u8) -> u16 {
    if round == 1 {
        return 1;
    } else if round > 1 && rc(round - 1) < 0x80 {
        return 2 * rc(round - 1);
    } else if round > 1 && rc(round - 1) >= 0x80 {
        return (2 * rc(round - 1)) ^ 0x11B;
    } else {
        return 0;
    };
}
