use std::str::FromStr;

use steamid::{AccountType, Instance, SteamId, Universe};

#[test]
fn from_steamid64_str() {
    assert!(
        SteamId::from_str("0").is_ok(),
        "Unable to parse 1-length SteamId64"
    );
    assert!(
        SteamId::from_str("76561198024962300").is_ok(),
        "Unable to parse valid SteamId64"
    );
    assert!(
        SteamId::from_str("76561198930384249").is_ok(),
        "Unable to parse valid SteamId64"
    );
    assert!(
        SteamId::from_str("76561197990953833").is_ok(),
        "Unable to parse valid SteamId64"
    );
    assert!(
        SteamId::from_str("76561197960265730").is_ok(),
        "Unable to parse valid SteamId64"
    );
    assert!(
        SteamId::from_str("18446744073709551615").is_ok(),
        "Unable to parse max-length 64-bit SteamId64"
    );

    // Things that SHOULDN'T happen:
    assert!(
        SteamId::from_str("").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("-1").is_err(),
        "Able to parse negative SteamId"
    );

    // Actually evaluate some values.
    let shad = SteamId::from_str("76561198024962300").unwrap();
    assert_eq!(shad.universe(), Universe::Public);
    assert_eq!(shad.authentication_server(), 0);
    assert_eq!(shad.instance(), Instance::from(1));

    let ko = SteamId::from_str("103582791464489035").unwrap();
    assert_eq!(ko.universe(), Universe::Public);
    assert_eq!(ko.authentication_server(), 1);
    assert_eq!(u32::from(ko.instance()), 0);
}

#[test]
fn from_steamid2_str() {
    assert!(
        SteamId::from_str("STEAM_1:1:1").is_ok(),
        "Unable to parse valid SteamId2"
    );
    assert!(
        SteamId::from_str("STEAM_0:0:0").is_ok(),
        "Unable to parse weird, but valid SteamId2"
    );
    assert!(
        SteamId::from_str("STEAM_0:1:2147483647").is_ok(),
        "Unable to parse last valid SteamId2"
    );

    // Things that SHOULDN'T happen:
    assert!(
        SteamId::from_str("STEAM_1:1:").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("STEAM_0:fa:1").is_err(),
        "Able to parse SteamId with characters"
    );
    assert!(
        SteamId::from_str("STEAM_").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("STEAM_0:1:2147483648").is_err(),
        "Able to parse overflowing SteamId2 (Account Number)"
    );
    assert!(
        SteamId::from_str("STEAM_0:2:1").is_err(),
        "Able to parse overflowing SteamId2 (Authentication Server)"
    );
    assert!(
        SteamId::from_str("STEAM_256:1:1").is_err(),
        "Able to parse overflowing SteamId2 (Universe)"
    );

    // Actually evaluate some values.
    let mitch = SteamId::from_str("STEAM_1:1:485059260").unwrap();
    assert_eq!(mitch.universe(), Universe::Public);
    assert_eq!(mitch.authentication_server(), 1);
    assert_eq!(mitch.instance(), Instance::from(1));

    // This should be idential to the previous one, due to 'Valve Logic'
    //                                     V - SteamId2Legacy format.
    let mitch_2 = SteamId::from_str("STEAM_0:1:485059260").unwrap();
    assert_eq!(mitch, mitch_2);
}

#[test]
fn from_steamid3_str() {
    assert!(
        SteamId::from_str("[U:0:0]").is_ok(),
        "Unable to parse weird, but valid SteamId3"
    );
    assert!(
        SteamId::from_str("[U:1:3]").is_ok(),
        "Unable to parse valid SteamId3"
    );
    assert!(
        SteamId::from_str("[G:1:3]").is_ok(),
        "Unable to parse valid SteamId3"
    );
    assert!(
        SteamId::from_str("[U:1:4294967295]").is_ok(),
        "Unable to parse last valid SteamId3"
    );

    // Things that SHOULDN'T happen:
    assert!(
        SteamId::from_str("[]").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("[U:]").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("[U::]").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("[::]").is_err(),
        "Able to parse blank SteamId"
    );
    assert!(
        SteamId::from_str("G:1:3").is_err(),
        "Able to parse unbracketed SteamId3"
    );
    assert!(
        SteamId::from_str("[F::]").is_err(),
        "Able to parse SteamId3 with invalid characters"
    );
    assert!(
        SteamId::from_str("[G:1:3").is_err(),
        "Able to parse invalid SteamId3"
    );
    assert!(
        SteamId::from_str("[0:1:3]").is_err(),
        "Able to parse SteamId3 with numeric account type"
    );
    assert!(
        SteamId::from_str("[U:1:4294967296]").is_err(),
        "Able to parse overflowing SteamId3"
    );
    assert!(
        SteamId::from_str("[U:256:1]").is_err(),
        "Able to parse overflowing SteamId3"
    );

    // Actually evaluate some values.
    let mitch = SteamId::from_str("[U:1:970118521]").unwrap();
    assert_eq!(mitch.universe(), Universe::Public);
    assert_eq!(mitch.authentication_server(), 1);
    assert_eq!(mitch.account_type(), AccountType::Individual);
    assert_eq!(mitch.instance(), Instance::from(1));

    let mitch = SteamId::from_str("[I:1:970118521]").unwrap();
    assert_eq!(mitch.universe(), Universe::Public);
    assert_eq!(mitch.authentication_server(), 1);
    assert_eq!(mitch.account_type(), AccountType::Invalid);
    assert_eq!(mitch.instance(), Instance::from(1));

    let zero = SteamId::from_str("[U:1:2]").unwrap();
    assert_eq!(
        zero.authentication_server(),
        0,
        "Not properly masking auth server bit"
    );
    let one = SteamId::from_str("[U:1:3]").unwrap();
    assert_eq!(
        one.authentication_server(),
        1,
        "Not properly masking auth server bit"
    );
}
