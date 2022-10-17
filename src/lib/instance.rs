use crate::{mask, shift, SteamId};

use super::ChatType;

/// Places in which the account exists.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Instance {
    None(ChatType),
    Desktop(ChatType),
    Console(ChatType),
    Web(ChatType),
}

impl Default for Instance {
    fn default() -> Self {
        Instance::Desktop(ChatType::default())
    }
}

impl From<Instance> for u32 {
    #[rustfmt::skip]
    #[allow(clippy::identity_op)]
    fn from(instance: Instance) -> Self {
        use Instance::*;
        match instance {
            None(v)     => 0 | ((u8::from(v) as u32) << (shift::CHAT_TYPE - shift::INSTANCE)),
            Desktop(v)  => 1 | ((u8::from(v) as u32) << (shift::CHAT_TYPE - shift::INSTANCE)),
            Console(v)  => 2 | ((u8::from(v) as u32) << (shift::CHAT_TYPE - shift::INSTANCE)),
            Web(v)      => 4 | ((u8::from(v) as u32) << (shift::CHAT_TYPE - shift::INSTANCE)),
        }
    }
}

impl From<u32> for Instance {
    #[rustfmt::skip]
    fn from(v: u32) -> Self {
        use Instance::*;
        let v = v as u64;
        let masked: u64 = (v << shift::INSTANCE ) & mask::INSTANCE;
        // CHAT_TYPE is an 8-bit mask, so we're safe to cast into a u8 here.
        // We just pass the existing chat_type directly through.
        let chat_type = ChatType::from(((masked & mask::CHAT_TYPE) >> shift::CHAT_TYPE) as u8);
        // Remove the chat bits since we already extracted that.
        let masked_chat = (masked & (!mask::CHAT_TYPE)) >> shift::INSTANCE;
        match masked_chat {
            0 => None(chat_type),
            1 => Desktop(chat_type),
            2 => Console(chat_type),
            4 => Web(chat_type),
            // This is LOSSY!
            // We can only represent values for which we have a discriminant.
            _ => Desktop(chat_type),
        }
    }
}

impl From<&SteamId> for Instance {
    fn from(steamid: &SteamId) -> Self {
        let val = ((steamid.id & mask::INSTANCE) >> shift::INSTANCE) as u32;
        Instance::from(val)
    }
}

/////////////////////////////////////////////////////////////////////////////
// Unit Testing
/////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use crate::*;

    /// Ensures our documentation and everything line up with the actual defaults
    #[test]
    fn defaults() {
        assert_eq!(Instance::default(), Instance::Desktop(ChatType::default()));
    }

    #[test]
    fn value_conversion() {
        assert_eq!(Instance::from(0), Instance::None(ChatType::default()));
        assert_eq!(Instance::from(1), Instance::Desktop(ChatType::default()));
        assert_eq!(Instance::from(3), Instance::Desktop(ChatType::default()));
        assert_eq!(Instance::from(3), Instance::Desktop(ChatType::default()));
    }

    #[test]
    fn steamid_values() {
        assert_eq!(
            SteamId::from(76561193729995004).instance(),
            Instance::None(ChatType::None)
        );
        assert_eq!(
            SteamId::from(76561198024962300).instance(),
            Instance::Desktop(ChatType::None)
        );
        assert_eq!(
            SteamId::from(76561202319929596).instance(),
            Instance::Console(ChatType::None)
        );
        assert_eq!(
            SteamId::from(76561210909864188).instance(),
            Instance::Web(ChatType::None)
        );
    }
}
