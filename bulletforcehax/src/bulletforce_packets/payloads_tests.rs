#[cfg(test)]
mod packets_payload_tests {
    use super::super::*;
    use either::Either;
    use std::convert::TryFrom;

    #[test]
    fn test_game_info() {
        fn gen<'a>() -> GameInfo<'a> {
            GameInfo {
                game_id: "game id",
                room_id: "room id",
                store_id: "store id",
                room_name: "room name",
                mode_name: "mode name",
                password: "password",
                map_name: "map name",
                match_started: true,
                switching_map: false,
                room_type: 2,
                dedicated: false,
                hardcore: false,
                allowed_weapons: 0x1234567890ABCDEF,
                mean_rank: Either::Right(12.34),
                mean_kd: 2.,
                average_rank: 1337,
                event_code: 0,
                is_open: false,
                max_players: 2,
                player_count: 1,
            }
        }

        let info = gen();
        let info_clone = gen();

        let ht = info.into();
        let info_new = GameInfo::try_from_hashtable(ht).unwrap().unwrap();
        assert_eq!(info_clone, info_new);
    }

    #[test]
    fn test_game_properties() {
        fn gen<'a>() -> GameProperties<'a> {
            GameProperties {
                game_id: "game id",
                room_id: "room id",
                store_id: "store id",
                room_name: "room name",
                mode_name: "mode name",
                password: "password",
                map_name: "map name",
                match_started: true,
                switching_map: false,
                room_type: 2,
                dedicated: false,
                hardcore: false,
                allowed_weapons: 0x1234567890ABCDEF,
                mean_rank: Either::Left(12),
                mean_kd: 2.,
                average_rank: 1337,
                event_code: 0,
                is_open: Some(false),
                max_players: Some(2),

                spectate_for_mods_only: false,
                max_ping: 123,
                banned_weapon_message: "Banned message",
                time_scale: 1.,
                match_countdown_time: 10.,
                round_started: false,
                score_limit: 123,
                gun_game_preset: 2,
                cleanup_cache_on_leave: Some(false),
                props_listed_in_lobby: Some(vec!["1", "2", "", "asd"]),
                is_visible: Some(true),
                master_client_id: None,
            }
        }

        let info = gen();
        let info_clone = gen();

        let ht: HashMap<ProtocolValue, ProtocolValue> = info.into();
        let info_new = GameProperties::try_from(ht).unwrap();
        assert_eq!(info_clone, info_new);
    }

    #[test]
    fn test_player_properties() {
        fn gen<'a>() -> PlayerProperties<'a> {
            PlayerProperties::NameOnly("Test")
        }

        let info = gen();
        let info_clone = gen();

        let ht: HashMap<ProtocolValue, ProtocolValue> = info.into();
        let info_new = PlayerProperties::try_from(ht).unwrap();
        assert_eq!(info_clone, info_new);
    }
}
