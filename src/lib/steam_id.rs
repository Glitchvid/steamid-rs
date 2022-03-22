const PROFILE_URL: &str = "http://steamcommunity.com/profiles/";
const GROUP_URL: &str = "http://steamcommunity.com/gid/";

use std::fmt::{Debug, Display};
use std::str::FromStr;
use thiserror::Error;

use crate::account_type::AccountType;
use crate::universe::Universe;
use crate::{mask, shift};
use crate::{ChatType, Instance};

/// Reasons why parsing a SteamId might fail.
#[derive(Debug, Error, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum SteamIdParseError {
    #[error("Unable to identify SteamId format")]
    UknownFormat,

    #[error("Invalid value")]
    Invalid,

    #[error("Too short")]
    TooShort,

    #[error("Empty value")]
    Empty,

    #[error("{0}")]
    Other(&'static str),
}

fn replace_bits(val: u64, mask: u64, new: u64) -> u64 {
    (val & (!mask)) | (new & mask)
}

/// Used to build a new SteamId from values.
///
/// # Examples #
///
/// - Initiating a simple user and getting their profile URL
/// ```
/// use steamid::SteamIdBuilder;
///
/// let user = SteamIdBuilder::new().account_number(1).finish();
///
/// let url = user.url();
/// assert_eq!(url, "http://steamcommunity.com/profiles/[U:1:2]" )
/// ```
///
/// - Taking an existing SteamId and getting a builder back to modify.
/// ```
/// use steamid::{SteamId, SteamIdBuilder};
///
/// let base = SteamId::from(76561197990953833);
/// let multiverse = SteamIdBuilder::from(&base).universe(2).finish();
/// assert_eq!(multiverse.url(), "http://steamcommunity.com/profiles/[U:2:30688105]")
/// ```
///
/// - Completely specify a SteamId.
/// ```
/// use steamid::SteamIdBuilder;
///
/// let player = SteamIdBuilder::new()
///     .authentication_server(1)
///     .account_number(15344052)
///     .account_type('U')
///     .instance(4)
///     .finish();
/// assert_eq!(player.id, 76561210875855721)
/// ```

#[derive(Debug, Clone)]
pub struct SteamIdBuilder {
    id: u64,
}

impl SteamIdBuilder {
    /// Begets a new SteamIdBuilder with some resonable defaults.
    ///
    /// Defaults:
    /// - Account Type = [AccountType::Individual]
    /// - Universe = [Universe::Public]
    /// - Instance = [Instance::Desktop]
    pub fn new() -> Self {
        SteamIdBuilder { id: 0 }
            .account_type(AccountType::Individual)
            .universe(Universe::Public)
            // If we don't set instance to 1 here then we won't match 3rd party
            // steamID64 parsing and formatting.
            .instance(1)
    }

    /// Consumes the SteamIdBuilder and returns a new SteamId.
    ///
    /// # Example
    ///
    /// ```
    /// use steamid::{SteamIdBuilder, IdFormat, Instance, AccountType};
    ///
    /// let user = SteamIdBuilder::new()
    /// .account_number(15344052)
    /// .authentication_server(1)
    /// .finish();
    ///
    /// assert_eq!(IdFormat::SteamId3(&user).to_string(), "[U:1:30688105]");
    /// ```
    pub fn finish(self) -> SteamId {
        SteamId { id: self.id }
    }

    /// Sets the Authentication server bit
    ///
    /// Only meaningful values are `0` or `1`, anything `> 1` is capped to `1`.
    pub fn authentication_server(self, val: u64) -> Self {
        let new_val = if val >= 1 { 1 } else { 0 };
        Self {
            id: replace_bits(self.id, mask::AUTH_SERVER, new_val << shift::AUTH_SERVER),
        }
    }

    /// Sets the 31-bit Steam account number.
    ///
    /// This is what is visualized in the [IdFormat::SteamId2] format.  E.G
    /// `[U:1:3]` is account number `1` (`STEAM_1:1:1`)
    ///
    /// **Notice**: Values exceeding `2^31` are truncated at the highest bit.
    pub fn account_number(self, val: u64) -> Self {
        Self {
            id: replace_bits(self.id, mask::ACCOUNT_NUMBER, val << shift::ACCOUNT_NUMBER),
        }
    }

    /// Sets the account type, this can either by an [AccountType] itself, or
    /// any value which can be converted.
    ///
    ///
    /// **Notice**: This function will also change the `Instance` value when
    /// set to any values other than `Invalid`, `Individual` or `Chat`.
    /// This is to match observed Valve behavior.
    /// If you want to avoid this, use `account_type_preserve_bits` instead.
    ///
    /// # Example
    ///
    /// ```
    /// use steamid::{SteamIdBuilder, IdFormat, Instance, AccountType};
    ///
    /// let group = SteamIdBuilder::new()
    /// .account_number(17483813)
    /// .authentication_server(1)
    /// .account_type(AccountType::Clan)
    /// .finish();
    ///
    /// assert_eq!(IdFormat::SteamId64(&group).to_string(), "103582791464489035");
    /// assert_eq!(IdFormat::SteamId3(&group).to_string(), "[g:1:34967627]");
    /// assert_eq!(IdFormat::SteamId2(&group).to_string(), "STEAM_1:1:17483813");
    /// ```
    pub fn account_type<T: Into<AccountType>>(self, val: T) -> Self {
        let atype: AccountType = val.into();
        let new = match atype {
            AccountType::Invalid | AccountType::Individual => self,
            AccountType::Chat(v) => self.instance(Instance::None(v)),
            _ => self.instance(Instance::None(ChatType::default())),
        };
        new.account_type_preserve_bits(atype)
    }

    /// Sets the account type, this can either by an [AccountType] itself, or
    /// any value which can be converted.
    ///
    /// This is different to `account_type` in that it does not have any
    /// side-effects of changing the `Instance` value.
    pub fn account_type_preserve_bits<T: Into<AccountType>>(self, val: T) -> Self {
        let atype: AccountType = val.into();
        Self {
            id: replace_bits(
                self.id,
                mask::ACCOUNT_TYPE,
                (u8::from(atype) as u64) << shift::ACCOUNT_TYPE,
            ),
        }
    }

    /// Sets the account [Instance], this can either by an Instance itself,
    /// or any value which can be converted.
    ///
    /// This is usually best left to whatever default value is set.
    pub fn instance<T: Into<Instance>>(self, val: T) -> Self {
        let val: Instance = val.into();
        let val = u32::from(val) as u64;
        Self {
            id: replace_bits(self.id, mask::INSTANCE, val << shift::INSTANCE),
        }
    }

    /// Sets the [Universe] this account exists within.
    /// or any value which can be converted.
    pub fn universe<T: Into<Universe>>(self, val: T) -> Self {
        let val: Universe = val.into();
        let val = u8::from(val) as u64;
        Self {
            id: replace_bits(self.id, mask::UNIVERSE, val << shift::UNIVERSE),
        }
    }
}

impl From<&SteamId> for SteamIdBuilder {
    fn from(steamid: &SteamId) -> Self {
        SteamIdBuilder { id: steamid.id }
    }
}

impl FromStr for SteamIdBuilder {
    type Err = SteamIdParseError;

    // Ugly parsing code since we're not using Regex.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        // Quickly evaluate SteamId64s
        let c = s.chars().count();
        if (c <= 20) && (c > 0) {
            let parse = s.parse::<u64>();
            if let Ok(parse) = parse {
                return Ok(SteamIdBuilder { id: parse });
            }
        }
        // Evaluate if SteamId3
        {
            let mut chars = s.chars();
            let first = chars.next().ok_or(SteamIdParseError::Empty)?;
            let last = chars.last().ok_or(SteamIdParseError::TooShort)?;
            // If we're within SteamId3 bracket.
            if first == '[' && last == ']' {
                let mut steam3 = s.chars();
                let _first = steam3.next().ok_or(SteamIdParseError::Empty)?;
                let atype: AccountType = steam3.next().ok_or(SteamIdParseError::TooShort)?.into();
                let _ = steam3.next().ok_or(SteamIdParseError::TooShort)?;
                let mut values = steam3.collect::<String>();
                // Remove the last bracket.
                values.pop();
                let fields: Vec<&str> = values.split(':').collect();
                if fields.len() < 2 {
                    return Err(SteamIdParseError::TooShort);
                }
                return Ok(SteamIdBuilder::new()
                    .account_type(atype)
                    .universe(u8::from_str(fields[0]).map_err(|_| SteamIdParseError::Invalid)?)
                    .authentication_server(
                        fields[1].parse().map_err(|_| SteamIdParseError::Invalid)?,
                    )
                    .account_number(
                        fields[1]
                            .parse::<u64>()
                            .map_err(|_| SteamIdParseError::Invalid)?
                            >> shift::ACCOUNT_NUMBER,
                    ));
            }
        }
        // Evaluate if SteamId2
        {
            if s.starts_with("STEAM_") {
                let steam2 = s.chars().skip(6).collect::<String>();
                let fields = steam2.split(':').collect::<Vec<&str>>();
                if fields.len() < 3 {
                    return Err(SteamIdParseError::TooShort);
                }
                return Ok(SteamIdBuilder::new()
                    .universe(
                        u8::from_str(fields[0])
                            .map_err(|_| SteamIdParseError::Invalid)?
                            // Interpret 'Unspecified' universe as 'Public' to
                            // comply with Valve's implementation of steamID in
                            // legacy Source/GoldSrc engine games.
                            .max(1),
                    )
                    .authentication_server(
                        fields[1].parse().map_err(|_| SteamIdParseError::Invalid)?,
                    )
                    .account_number(fields[2].parse().map_err(|_| SteamIdParseError::Invalid)?)
                    // SteamId2 is only ever used for individual 'U'sers.
                    .account_type('U'));
            }
        }
        // Unable to find any matching type.
        Err(SteamIdParseError::UknownFormat)
    }
}

/// Represents a complete SteamId for querying values.
///
/// To safely change the values of an existing SteamId convert into a
/// SteamIdBuilder, and use the builder to set desired values,
/// then `.finish()` the builder to get the modified SteamId.
///
/// Using an intermediary builder allows cleaner one-line syntax, instead of
/// using a mutable steamid with setter and getter functions.
///
///  # Examples #
/// - Getting a SteamId64 directly from a u64
/// ```
/// use steamid::{SteamId, IdFormat};
///
/// let player = SteamId::from(76561197990953833);
/// assert_eq!(IdFormat::SteamId3(&player).to_string(), "[U:1:30688105]");
/// ```
/// - Parsing from a string
/// ```
/// use std::str::FromStr;
/// use steamid::{SteamId, IdFormat};
///
/// let player = SteamId::from_str("76561197990953833").unwrap();
/// assert_eq!(IdFormat::SteamId3(&player).to_string(), "[U:1:30688105]");
/// ```
/// - Printing a SteamId
/// ```
/// use steamid::{SteamId, IdFormat};
///
/// let steamid = SteamId::from(76561197990953833);
/// println!("steamID64:\t{}", IdFormat::SteamId64(&steamid));
/// println!("steamID:  \t{}", IdFormat::SteamId2(&steamid));
/// println!("steamID3: \t{}", IdFormat::SteamId3(&steamid));
/// ```
/// - Building from a [SteamIdBuilder]
/// ```
/// use steamid::SteamIdBuilder;
///
/// let player = SteamIdBuilder::new()
///     .authentication_server(1)
///     .account_number(15344052)
///     .account_type('U')
///     .instance(4)
///     .finish();
/// assert_eq!(player.id, 76561210875855721)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct SteamId {
    pub id: u64,
}

impl SteamId {
    /// Returns the authentication bit for this SteamId
    /// # Example
    ///
    /// ```
    /// use steamid::SteamId;
    ///
    /// let user: SteamId = "[U:1:30688105]".parse().unwrap();
    /// assert_eq!(user.authentication_server(), 1)
    /// ```
    pub fn authentication_server(&self) -> u32 {
        (self.id & mask::AUTH_SERVER) as u32
    }

    /// Returns the account number for this SteamId
    /// # Example
    ///
    /// ```
    /// use steamid::SteamId;
    ///
    /// let user: SteamId = "[U:1:30688105]".parse().unwrap();
    /// assert_eq!(user.account_number(), 15344052)
    /// ```
    pub fn account_number(&self) -> u32 {
        ((self.id & mask::ACCOUNT_NUMBER) >> shift::ACCOUNT_NUMBER) as u32
    }

    /// Returns the [AccountType] for this SteamId
    /// # Example
    ///
    /// ```
    /// use steamid::{SteamId, AccountType};
    ///
    /// let id = SteamId::from(103582791464489035);
    /// assert_eq!(id.account_type(), AccountType::Clan)
    /// ```
    pub fn account_type(&self) -> AccountType {
        AccountType::from(self)
    }

    /// Returns the [Instance] for this SteamId
    ///
    /// **Note**: Chat-typed Ids will have bits set > 4096.
    /// # Example
    /// ```
    /// use steamid::{ChatType, SteamId, Instance};
    ///
    /// let id = SteamId::from(108156759836037195);
    /// assert_eq!(id.instance(), Instance::None(ChatType::ClanChat))
    /// ```
    pub fn instance(&self) -> Instance {
        Instance::from(self)
    }

    /// Returns the [Universe] for this SteamId
    /// # Example
    /// ```
    /// use steamid::{SteamId};
    ///
    /// let id: SteamId = "[g:1:34967627]".parse().unwrap();
    /// assert_eq!(id.url(), "http://steamcommunity.com/gid/[g:1:34967627]")
    /// ```
    pub fn universe(&self) -> Universe {
        Universe::from(self)
    }

    /// Returns a url for this SteamId
    ///
    /// This is a best-effort function, no guarantee that the URL resolves to
    /// a valid user.
    ///
    /// URL formats are only specified for [AccountType::Individual] and
    /// [AccountType::Clan].
    ///
    /// # Example
    /// ```
    /// use steamid::{SteamId, Universe};
    ///
    /// let id: SteamId = "[U:1:30688105]".parse().unwrap();
    /// assert_eq!(id.universe(), Universe::Public)
    /// ```
    pub fn url(&self) -> String {
        let url_prefix = match self.account_type() {
            AccountType::Clan => GROUP_URL,
            _ => PROFILE_URL,
        };
        let url_postfix = IdFormat::SteamId3(self).to_string();
        let mut url = String::with_capacity(url_prefix.len() + url_postfix.len());
        url.push_str(url_prefix);
        url.push_str(&url_postfix);
        url
    }
}

// Let users cast directly from a u64 to a SteamId if they want.
impl From<u64> for SteamId {
    fn from(id: u64) -> Self {
        SteamId { id }
    }
}

impl From<&SteamId> for u64 {
    fn from(steamid: &SteamId) -> Self {
        steamid.id
    }
}

impl FromStr for SteamId {
    type Err = SteamIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SteamIdBuilder::from_str(s)?.finish())
    }
}

/// Used to wrap a SteamId to specify output formatting, since SteamId does not implement Display.
///
/// # Examples #
///
/// ```
/// use steamid::{IdFormat, SteamIdBuilder};
///
/// let user = SteamIdBuilder::new()
///     .account_number(15344052)
///     .authentication_server(1)
///     .account_type('U')
///     .finish();
///
/// assert_eq!(format!("{}",  IdFormat::SteamId64(&user)), "76561197990953833");
/// assert_eq!(format!("{}",  IdFormat::SteamId2(&user)), "STEAM_1:1:15344052");
/// assert_eq!(format!("{}",  IdFormat::SteamId3(&user)), "[U:1:30688105]");
///```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdFormat<'a> {
    /// The full 64-bit "steamID64"
    ///
    /// Example: `76561197960265731`
    SteamId64(&'a SteamId),
    /// Older "steamID" format used commonly.
    ///
    /// Example: `STEAM_1:1:15344052`
    SteamId2(&'a SteamId),
    /// This is different from normal `SteamId2` in that the universe is always
    /// shown as `0`.
    ///
    /// ## Example ##
    /// For SteamId3 `[U:1:3]`
    /// - SteamId2  = `STEAM_1:1:1`
    /// - SteamId2Legacy = `STEAM_0:1:1`
    SteamId2Legacy(&'a SteamId),
    /// Modern preferred standard.
    ///
    /// Example: `[U:1:30688105]`
    SteamId3(&'a SteamId),
}

impl Display for IdFormat<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdFormat::SteamId64(v) => write!(f, "{}", v.id),
            IdFormat::SteamId2(v) => write!(
                f,
                "STEAM_{}:{}:{}",
                u8::from(v.universe()),
                v.authentication_server(),
                v.account_number()
            ),
            IdFormat::SteamId2Legacy(v) => write!(
                f,
                "STEAM_{}:{}:{}",
                0,
                v.authentication_server(),
                v.account_number()
            ),
            IdFormat::SteamId3(v) => write!(
                f,
                "[{}:{}:{}]",
                char::from(AccountType::from(*v)),
                u8::from(v.universe()),
                v.id & (mask::AUTH_SERVER | mask::ACCOUNT_NUMBER)
            ),
        }
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
        let user = SteamIdBuilder::new().finish();
        assert_eq!(user.id, 76561197960265728, "Incorrect default SteamId.");
    }

    /// Makes sure builder functions are changing internal values correctly.
    #[test]
    fn builder_authentication_server() {
        let reference = SteamIdBuilder::new().finish().id;

        let set_one = SteamIdBuilder::new().authentication_server(1);
        assert_eq!(set_one.id, reference | mask::AUTH_SERVER);

        let set_high = SteamIdBuilder::new().authentication_server(8);
        assert_eq!(set_high.id, reference | mask::AUTH_SERVER);

        let set_zero = SteamIdBuilder::new().authentication_server(0);
        assert_eq!(set_zero.id, reference & !mask::AUTH_SERVER);
    }

    /// Makes sure builder functions are changing internal values correctly.
    #[test]
    fn builder_account_number() {
        let alfred = SteamIdBuilder::new().account_number(1).finish();
        assert_eq!(alfred.id, 76561197960265730);
    }

    /// Makes sure builder functions are changing internal values correctly.
    #[test]
    fn builder_account_type() {
        // Internally, `account_type` uses `account_type_preserve_bits` so,
        // I consider that covered by these tests.

        let invalid = SteamIdBuilder::new().account_type('I').finish();
        assert_eq!(invalid.id, 72057598332895232);
        let invalid = SteamIdBuilder::new().account_type(0).finish();
        assert_eq!(invalid.id, 72057598332895232);

        let user = SteamIdBuilder::new().account_type('U').finish();
        assert_eq!(user.id, 76561197960265728);
        let user = SteamIdBuilder::new().account_type(1).finish();
        assert_eq!(user.id, 76561197960265728);

        let multiseat = SteamIdBuilder::new().account_type('M').finish();
        assert_eq!(multiseat.id, 81064793292668928);
        let multiseat = SteamIdBuilder::new().account_type(2).finish();
        assert_eq!(multiseat.id, 81064793292668928);

        let game_server = SteamIdBuilder::new().account_type('G').finish();
        assert_eq!(game_server.id, 85568392920039424);
        let game_server = SteamIdBuilder::new().account_type(3).finish();
        assert_eq!(game_server.id, 85568392920039424);

        let anon_game_server = SteamIdBuilder::new().account_type('A').finish();
        assert_eq!(anon_game_server.id, 90071992547409920);
        let anon_game_server = SteamIdBuilder::new().account_type(4).finish();
        assert_eq!(anon_game_server.id, 90071992547409920);

        let pending = SteamIdBuilder::new().account_type('P').finish();
        assert_eq!(pending.id, 94575592174780416);
        let pending = SteamIdBuilder::new().account_type(5).finish();
        assert_eq!(pending.id, 94575592174780416);

        let content_server = SteamIdBuilder::new().account_type('C').finish();
        assert_eq!(content_server.id, 99079191802150912);
        let content_server = SteamIdBuilder::new().account_type(6).finish();
        assert_eq!(content_server.id, 99079191802150912);

        let clan = SteamIdBuilder::new().account_type('g').finish();
        assert_eq!(clan.id, 103582791429521408);
        let clan = SteamIdBuilder::new().account_type(7).finish();
        assert_eq!(clan.id, 103582791429521408);

        let chat = SteamIdBuilder::new().account_type('L').finish();
        assert_eq!(chat.id, 108121575428980736);

        let chat = SteamIdBuilder::new().account_type('T').finish();
        assert_eq!(chat.id, 108103983242936320);

        let chat = SteamIdBuilder::new().account_type('c').finish();
        assert_eq!(chat.id, 108156759801069568);

        // Console User aka "P2P SuperSeeder" does not have a char.
        let console = SteamIdBuilder::new().account_type(9).finish();
        assert_eq!(console.id, 112589990684262400);

        let anon = SteamIdBuilder::new().account_type('a').finish();
        assert_eq!(anon.id, 117093590311632896);
        let anon = SteamIdBuilder::new().account_type(10).finish();
        assert_eq!(anon.id, 117093590311632896);
    }

    /// account type has side effect we need to verify are following spec.
    #[test]
    fn builder_account_type_side_effects() {
        let builder = SteamIdBuilder::new();

        // Baselines, default instance is 1.
        assert_eq!(
            u32::from(builder.clone().finish().instance()),
            1,
            "Baseline instance incorrect."
        );
        assert_eq!(
            u32::from(builder.clone().account_type('I').finish().instance()),
            1
        );

        // Test side-effecting
        assert_eq!(
            u32::from(builder.clone().account_type('c').finish().instance()),
            16384,
            "Chat account_type not setting instance bits."
        );
        assert_eq!(
            u32::from(builder.clone().account_type('c').finish().instance()),
            16384,
            "Chat account_type not resetting instance bits."
        );
        assert_eq!(
            u32::from(
                builder
                    .clone()
                    .instance(4)
                    .account_type('c')
                    .finish()
                    .instance()
            ),
            16384,
            "Chat account_type not resetting instance bits."
        );
        assert_eq!(
            u32::from(
                builder
                    .clone()
                    .instance(4)
                    .account_type('A')
                    .finish()
                    .instance()
            ),
            0,
            "Chat account_type not resetting instance bits."
        );
    }

    /// Makes sure builder functions are changing internal values correctly.
    #[test]
    fn builder_instance() {
        let alfred = SteamIdBuilder::new().account_number(1).instance(1).finish();
        assert_eq!(alfred.id, 76561197960265730);

        let alfred = SteamIdBuilder::new().account_number(1).instance(2).finish();
        assert_eq!(alfred.id, 76561202255233026);

        let alfred = SteamIdBuilder::new().account_number(1).instance(4).finish();
        assert_eq!(alfred.id, 76561210845167618);

        let alfred = SteamIdBuilder::new()
            .account_number(1)
            .instance(Instance::Web(ChatType::default()))
            .finish();
        assert_eq!(alfred.id, 76561210845167618);
    }

    /// Makes sure builder functions are changing internal values correctly.
    #[test]
    fn builder_universe() {
        let builder = SteamIdBuilder::new().account_number(1);
        assert_eq!(builder.clone().universe(0).finish().id, 4503603922337794);
        assert_eq!(builder.clone().universe(1).finish().id, 76561197960265730);
        assert_eq!(
            builder.clone().universe(Universe::Unspecified).finish().id,
            4503603922337794
        );
        assert_eq!(
            builder.clone().universe(Universe::Public).finish().id,
            76561197960265730
        );
    }
}