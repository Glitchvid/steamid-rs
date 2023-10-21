use steamid::IdFormat::*;
use steamid::SteamId;

#[test]
fn id_to_string() {
    let ref1 = SteamId::from(76561197990953833);
    let ref2 = SteamId::from(76561198930384249);

    assert_eq!(SteamId64(ref1).to_string(), "76561197990953833");
    assert_eq!(SteamId64(ref2).to_string(), "76561198930384249");

    assert_eq!(SteamId2(ref1).to_string(), "STEAM_1:1:15344052");
    assert_eq!(SteamId2(ref2).to_string(), "STEAM_1:1:485059260");

    assert_eq!(SteamId2Legacy(ref1).to_string(), "STEAM_0:1:15344052");
    assert_eq!(SteamId2Legacy(ref2).to_string(), "STEAM_0:1:485059260");

    assert_eq!(SteamId3(ref1).to_string(), "[U:1:30688105]");
    assert_eq!(SteamId3(ref2).to_string(), "[U:1:970118521]");

    let ref3 = SteamId::from(103582791464489035);
    assert_eq!(SteamId3(ref3).to_string(), "[g:1:34967627]");

    let ref4 = SteamId::from(85568392923371047);
    assert_eq!(SteamId3(ref4).to_string(), "[G:1:3331623]");

    assert_eq!(
        Url(ref3).to_string(),
        "http://steamcommunity.com/gid/[g:1:34967627]"
    );
    assert_eq!(
        Url(ref1).to_string(),
        "http://steamcommunity.com/profiles/76561197990953833"
    );
}
