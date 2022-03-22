use std::fmt::Display;

use crate::{mask, shift};
use crate::{ChatType, SteamId};

/// Defines the role for this SteamId
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum AccountType {
    Invalid,
    Individual,
    Multiseat,
    /// Servers registered with a 'Game Server Login Token'
    GameServer,
    AnonGameServer,
    Pending,
    ContentServer,
    /// Steam Groups
    Clan,
    Chat(ChatType),
    /// AKA `P2P SuperSeeder`
    ConsoleUser,
    AnonUser,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Chat(_) => {
                write!(f, "Chat")
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

impl From<AccountType> for char {
    #[rustfmt::skip]
    fn from(acc: AccountType) -> Self {
        use AccountType::*;
        use ChatType::*;
        match acc {
            Invalid         => 'I',
            Individual      => 'U',
            Multiseat       => 'M',
            GameServer      => 'G',
            AnonGameServer  => 'A',
            Pending         => 'P',
            ContentServer   => 'C',
            Clan            => 'g',
            Chat(v)         => match v {
                MatchMakingLobby    => 'T',
                Lobby               => 'L',
                ClanChat            => 'c',
                _                   => 'c',
            },
            ConsoleUser     => 'I',
            AnonUser        => 'a',
        }
    }
}

// We can safely use a u8 here since AccountType is only 4-bits in packed repr.
impl From<AccountType> for u8 {
    #[rustfmt::skip]
    fn from(acc: AccountType) -> Self {
        use AccountType::*;
        match acc {
            Invalid         => 0,
            Individual      => 1,
            Multiseat       => 2,
            GameServer      => 3,
            AnonGameServer  => 4,
            Pending         => 5,
            ContentServer   => 6,
            Clan            => 7,
            Chat(_)         => 8,
            ConsoleUser     => 9,
            AnonUser        => 10,
        }
    }
}

impl From<u8> for AccountType {
    #[rustfmt::skip]
    fn from(v: u8) -> Self {
        use AccountType::*;
        use ChatType::*;
        match v {
            0   => Invalid,
            1   => Individual,
            2   => Multiseat,
            3   => GameServer,
            4   => AnonGameServer,
            5   => Pending,
            6   => ContentServer,
            7   => Clan,
            8   => Chat(ClanChat),
            9   => ConsoleUser,
            10  => AnonUser,
            _   => Invalid,
        }
    }
}

impl From<char> for AccountType {
    #[rustfmt::skip]
    fn from(c: char) -> Self {
        use AccountType::*;
        use ChatType::*;
        match c {
            'I'   => Invalid,
            'U'   => Individual,
            'M'   => Multiseat,
            'G'   => GameServer,
            'A'   => AnonGameServer,
            'P'   => Pending,
            'C'   => ContentServer,
            'g'   => Clan,
            'L'   => Chat(Lobby),
            'T'   => Chat(MatchMakingLobby),
            'c'   => Chat(ClanChat),
            'a'   => AnonUser,
            _     => Invalid,
        }
    }
}

impl From<&SteamId> for AccountType {
    #[rustfmt::skip]
    fn from(steamid: &SteamId) -> Self {
        use AccountType::*;

        let account_type = (steamid.id & mask::ACCOUNT_TYPE) >> shift::ACCOUNT_TYPE ;
        match account_type {
            0   => Invalid,
            1   => Individual,
            2   => Multiseat,
            3   => GameServer,
            4   => AnonGameServer,
            5   => Pending,
            6   => ContentServer,
            7   => Clan,
            8   => Chat(ChatType::from(steamid)),
            9   => ConsoleUser,
            10  => AnonUser,
            _   => Invalid,
        }
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
        assert_eq!(AccountType::from(0), AccountType::Invalid);
        assert_eq!(AccountType::from(1), AccountType::Individual);
        assert_eq!(AccountType::from(8), AccountType::Chat(ChatType::ClanChat));
        assert_eq!(AccountType::from(100), AccountType::Invalid);
    }
}
