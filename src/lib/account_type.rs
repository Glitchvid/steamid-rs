use std::fmt::Display;

use crate::{mask, shift};
use crate::{ChatType, SteamId};

/// Defines the role this SteamId is used for.
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum AccountType {
    Invalid,
    // Normal Steam Accounts are usually this
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
        write!(f, "{}", char::from(*self))
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
        let account_typed = AccountType::from(account_type as u8);
        match account_typed {
            Chat(_) => Chat(ChatType::from(steamid)),
            _ => account_typed,
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
        // Test common cases
        assert_eq!(AccountType::from(0), AccountType::Invalid);
        assert_eq!(AccountType::from(1), AccountType::Individual);
        assert_eq!(AccountType::from(8), AccountType::Chat(ChatType::ClanChat));
        assert_eq!(AccountType::from(255), AccountType::Invalid);
        assert_eq!(AccountType::from('Z'), AccountType::Invalid);
        assert_eq!(
            char::from(AccountType::Chat(ChatType::MatchMakingLobby)),
            'T'
        );
        assert_eq!(char::from(AccountType::Chat(ChatType::Lobby)), 'L');
        assert_eq!(
            char::from(AccountType::Chat(ChatType::MatchMakingLobby)),
            'T'
        );
        assert_eq!(char::from(AccountType::Chat(ChatType::None)), 'c');
    }

    #[test]
    fn steamid_chat_conversion() {
        let bld = SteamIdBuilder::new()
            .account_number(1)
            .authentication_server(1);
        assert_eq!(
            AccountType::from(&bld.clone().account_type('L').finish()),
            AccountType::Chat(ChatType::Lobby)
        );
        assert_eq!(
            AccountType::from(&bld.clone().account_type('T').finish()),
            AccountType::Chat(ChatType::MatchMakingLobby)
        );
        assert_eq!(
            AccountType::from(&bld.clone().account_type('c').finish()),
            AccountType::Chat(ChatType::ClanChat)
        );
    }

    /// Ensure some simple round trip conversions
    #[test]
    fn account_type_reciprocity() {
        use AccountType::*;
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(0)))),
            u8::from(Invalid)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(1)))),
            u8::from(Individual)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(2)))),
            u8::from(Multiseat)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(3)))),
            u8::from(GameServer)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(4)))),
            u8::from(AnonGameServer)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(5)))),
            u8::from(Pending)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(6)))),
            u8::from(ContentServer)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(7)))),
            u8::from(Clan)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(8)))),
            u8::from(Chat(ChatType::ClanChat))
        );
        // Console user round-trips to Invalid via Char
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(9)))),
            u8::from(Invalid)
        );
        assert_eq!(u8::from(AccountType::from(9)), u8::from(ConsoleUser));
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(10)))),
            u8::from(AnonUser)
        );
        assert_eq!(
            u8::from(AccountType::from(char::from(AccountType::from(255)))),
            u8::from(Invalid)
        );
    }

    #[test]
    fn account_type_fmt() {
        for v in 0..=10 {
            let f = AccountType::from(v);
            assert_eq!(format!("{f}"), char::from(f).to_string());
        }
    }
}
