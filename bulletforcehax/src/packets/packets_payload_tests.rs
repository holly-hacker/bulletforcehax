#[cfg(test)]
mod packets_payload_tests {
    use super::super::*;

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
                mean_rank: 12.34,
                mean_kd: 2.,
                average_rank: 1337,
                event_code: 0,
                byte_253: false,
                byte_255: 2,
                byte_252: 1,
            }
        }

        let info = gen();
        let info_clone = gen();

        let ht = info.into_hashtable();
        let info_new = GameInfo::new_from_hashtable(ht).unwrap().unwrap();
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
                mean_rank: 12,
                mean_kd: 2.,
                average_rank: 1337,
                event_code: 0,
                byte_253: false,
                byte_255: 2,

                spectate_for_mods_only: false,
                max_ping: 123,
                banned_weapon_message: "Banned message",
                time_scale: 1.,
                match_countdown_time: 10.,
                round_started: false,
                score_limit: 123,
                gun_game_preset: 2,
                byte_249: false,
                byte_250: vec!["1", "2", "", "asd"],
                byte_254: true,
                byte_248: None,
            }
        }

        let info = gen();
        let info_clone = gen();

        let ht = info.into_hashtable();
        let info_new = GameProperties::new_from_hashtable(ht).unwrap();
        assert_eq!(info_clone, info_new);
    }

    #[test]
    fn test_player_properties() {
        fn gen<'a>() -> PlayerProperties<'a> {
            PlayerProperties::NameOnly("Test")
        }

        let info = gen();
        let info_clone = gen();

        let ht = info.into_hashtable();
        let info_new = PlayerProperties::new_from_hashtable(ht).unwrap();
        assert_eq!(info_clone, info_new);
    }
}
