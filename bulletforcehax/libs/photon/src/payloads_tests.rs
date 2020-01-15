#![cfg(test)]

use super::*;
use maplit::hashmap;
use std::convert::TryFrom;

#[test]
fn test_room_info() {
    fn gen<'a>() -> RoomInfo<'a> {
        RoomInfo {
            max_players: 12,
            is_open: false,
            is_visible: true,
            player_count: 0,
            cleanup_cache_on_leave: true,
            master_client_id: Some(1),
            custom_properties_lobby: vec!["test", "test2", "test3"],
            expected_users: vec![],
            empty_room_ttl: 0,
            player_ttl: 0,
            custom_properties: hashmap! {
                "test" => ProtocolValue::Integer(1),
                "test2" => ProtocolValue::Null(),
                "test3" => ProtocolValue::String("abc"),
            },
        }
    }

    let info = gen();
    let info_clone = gen();

    let ht = info.into();
    let info_new = RoomInfo::try_from_hashtable(ht).unwrap().unwrap();
    assert_eq!(info_clone, info_new);
}

#[test]
fn test_room_options() {
    fn gen<'a>() -> RoomOptions<'a> {
        RoomOptions {
            max_players: 12,
            is_open: false,
            is_visible: true,
            cleanup_cache_on_leave: true,
            custom_properties_lobby: vec!["test", "test2", "test3"],
            custom_properties: hashmap! {
                "test" => ProtocolValue::Integer(1),
                "test2" => ProtocolValue::Null(),
                "test3" => ProtocolValue::String("abc"),
            },
        }
    }

    let info = gen();
    let info_clone = gen();

    let ht: HashMap<ProtocolValue, ProtocolValue> = info.into();
    let info_new = RoomOptions::try_from(ht).unwrap();
    assert_eq!(info_clone, info_new);
}

#[test]
fn test_player() {
    fn gen<'a>() -> Player<'a> {
        Player {
            name: Some("Jeffrey"),
            user_id: Some("abc-def-ghi"),
            is_inactive: None,
            custom_properties: hashmap! {
                "is_cool" => ProtocolValue::Bool(true),
            },
        }
    }

    let info = gen();
    let info_clone = gen();

    let ht: HashMap<ProtocolValue, ProtocolValue> = info.into();
    let info_new = Player::try_from(ht).unwrap();
    assert_eq!(info_clone, info_new);
}
