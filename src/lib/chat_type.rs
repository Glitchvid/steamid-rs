use crate::{mask, shift, SteamId};

/// Defines the type of Chat a [AccountType::Chat] can be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChatType {
    /// Default for all non-Chat account types
    None,
    MatchMakingLobby,
    Lobby,
    /// Default for Chat account types
    ClanChat,
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::None
    }
}

impl From<ChatType> for u8 {
    #[rustfmt::skip]
    fn from(chat: ChatType) -> Self {
        use ChatType::*;
        match chat {
            None                => 0,
            MatchMakingLobby    => 1,
            Lobby               => 2,
            ClanChat            => 4,
        }
    }
}

impl From<u8> for ChatType {
    #[rustfmt::skip]
    fn from(v: u8) -> Self {
        use ChatType::*;
        match v {
            1   => MatchMakingLobby,
            2   => Lobby,
            4   => ClanChat,
            _   => None,
        }
    }
}
impl From<&SteamId> for ChatType {
    fn from(steamid: &SteamId) -> Self {
        // CHAT_TYPE is an 8-bit mask, so we're safe to cast into a u8 here.
        ChatType::from(((steamid.id & mask::CHAT_TYPE) >> shift::CHAT_TYPE) as u8)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Unit Testing
/////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn value_conversion() {
        assert_eq!(ChatType::from(1), ChatType::MatchMakingLobby);
        assert_eq!(ChatType::from(3), ChatType::None);
        assert_eq!(ChatType::from(4), ChatType::ClanChat);
    }
}
