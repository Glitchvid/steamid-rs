//! Todo: 
//! 
//! TOP LEVEL DOCUMENTATION

mod account_type;
mod chat_type;
mod instance;
mod steam_id;
mod universe;

// Re-Exports
pub use account_type::*;
pub use chat_type::*;
pub use instance::*;
pub use steam_id::*;
pub use universe::*;

/* Valve SteamID Format:
 *  A SteamID is just a packed 64-bit unsigned integer.
 *
 * It consists of five parts, from least to most significant bit:
 *  1. Authentication Server    - 1 bit     (1)
 *  2. Account Number           - 31 bits   (32)
 *  3. Instance                 - 20 bits   (52)
 *  4. Account Type             - 4 bits    (56)
 *  5. Universe                 - 8 bits    (64)
 *
 * This can be visualized like so:
 *  1. _______________________________________________________________X
 *  2. ________________________________XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX_
 *  3. ____________XXXXXXXXXXXXXXXXXXXX________________________________
 *  4. ________XXXX____________________________________________________
 *  5. XXXXXXXX________________________________________________________
 *
 * There are multiple ways to format a SteamID, some are lossy.
 *  A. SteamId64        - (1)+(2)+(3)+(4)+(5)
 *  B. SteamId2         - STEAM_(5):(1):(2)
 *  C. SteamId3         - [(4):(5):(1)+(2)]
*/

/*
 * Instance
 *  The Instance field nominally holds what 'instance' the steamID is, however
 *  when specifying a chatroom, the last 8 bits define the "type" of chatroom.
 * This can be visualized like so:
 *  ____________ZZZZZZZZXXXXXXXXXXXX
*/

#[rustfmt::skip]
mod mask {
    // Mask used to discard all bits we don't care about when using this field.
    // Mask should be applied before shifting.
    pub const AUTH_SERVER: u64 =
    0b0000000000000000000000000000000000000000000000000000000000000001;
    pub const ACCOUNT_NUMBER: u64 =
    0b0000000000000000000000000000000011111111111111111111111111111110;
    pub const INSTANCE: u64 =
    0b0000000000001111111111111111111100000000000000000000000000000000;
    pub const ACCOUNT_TYPE: u64 =
    0b0000000011110000000000000000000000000000000000000000000000000000;
    pub const UNIVERSE: u64 =
    0b1111111100000000000000000000000000000000000000000000000000000000;
    pub const CHAT_TYPE: u64 =
    0b0000000000001111111100000000000000000000000000000000000000000000;
}

mod shift {
    // How far to shift the SteamId64 value to the right to access this value.
    pub const AUTH_SERVER: u32 = super::mask::AUTH_SERVER.trailing_zeros();
    pub const ACCOUNT_NUMBER: u32 = super::mask::ACCOUNT_NUMBER.trailing_zeros();
    pub const INSTANCE: u32 = super::mask::INSTANCE.trailing_zeros();
    pub const ACCOUNT_TYPE: u32 = super::mask::ACCOUNT_TYPE.trailing_zeros();
    pub const UNIVERSE: u32 = super::mask::UNIVERSE.trailing_zeros();
    pub const CHAT_TYPE: u32 = super::mask::CHAT_TYPE.trailing_zeros();
}
