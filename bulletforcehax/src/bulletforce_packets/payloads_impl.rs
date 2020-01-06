use super::macros::*;
use super::*;
use either::Either;
use log::warn;
use std::collections::HashMap;
use std::convert::TryFrom;

impl<'s> GameInfo<'s> {
    pub fn new_from_hashtable_table<'a>(
        big_table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    ) -> PacketReadResult<HashMap<&'a str, Option<GameInfo<'a>>>> {
        let mut map: HashMap<&'a str, Option<GameInfo<'a>>> = HashMap::new();
        for (key, value) in big_table {
            // could look into getting map past the borrow checker
            let ht = unwrap_protocol_hashtable(value)?;
            let val = GameInfo::new_from_hashtable(ht)?;
            map.insert(unwrap_protocol_string(key)?, val);
        }

        Ok(map)
    }

    pub fn new_from_hashtable(table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<Option<GameInfo<'s>>> {
        if table.contains_key(&ProtocolValue::Byte(251)) {
            // got removed
            return Ok(None);
        }

        Some(GameInfo::try_from(table)).transpose()
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for GameInfo<'s> {
    type Error = PacketReadError;

    fn try_from(mut table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<GameInfo<'s>> {
        let ret = Ok(GameInfo {
            game_id: get_protocol_string(&mut table, ProtocolValue::String("gameID"))?,
            room_id: get_protocol_string(&mut table, ProtocolValue::String("roomID"))?,
            store_id: get_protocol_string(&mut table, ProtocolValue::String("storeID"))?,
            room_name: get_protocol_string(&mut table, ProtocolValue::String("roomName"))?,
            mode_name: get_protocol_string(&mut table, ProtocolValue::String("modeName"))?,
            password: get_protocol_string(&mut table, ProtocolValue::String("password"))?,
            map_name: get_protocol_string(&mut table, ProtocolValue::String("mapName"))?,
            match_started: get_protocol_bool(&mut table, ProtocolValue::String("matchStarted"))?,
            switching_map: get_protocol_bool(&mut table, ProtocolValue::String("switchingmap"))?,
            room_type: get_protocol_byte(&mut table, ProtocolValue::String("roomType"))?,
            dedicated: get_protocol_bool(&mut table, ProtocolValue::String("dedicated"))?,
            hardcore: get_protocol_bool(&mut table, ProtocolValue::String("hardcore"))?,
            allowed_weapons: {
                // this is an array of 2 u32s, but we save this as a u64 because it makes more sense
                let mut arr = get_protocol_array(&mut table, ProtocolValue::String("allowedweapons"))?;
                if arr.len() != 2 {
                    return Err(PacketReadError::Other(format!("allowedweapons array was not 2 long, but {}", arr.len())));
                }
                let int2 = unwrap_protocol_int(arr.remove(1))? as u64;
                let int1 = unwrap_protocol_int(arr.remove(0))? as u64;
                int1 | (int2 << 32)
            },
            mean_rank: get_protocol_int_or_float(&mut table, ProtocolValue::String("meanRank"))?,
            mean_kd: get_protocol_float(&mut table, ProtocolValue::String("meanKD"))?,
            average_rank: get_protocol_int(&mut table, ProtocolValue::String("averagerank"))?,
            event_code: get_protocol_int(&mut table, ProtocolValue::String("eventcode"))?,
            byte_252: get_protocol_byte(&mut table, ProtocolValue::Byte(252))?,
            byte_253: get_protocol_bool(&mut table, ProtocolValue::Byte(253))?,
            byte_255: get_protocol_byte(&mut table, ProtocolValue::Byte(255))?,
        });

        if ret.is_ok() && table.len() > 0 {
            warn!("Missed GameInfo parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for GameInfo<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        map.insert(ProtocolValue::String("gameID"), ProtocolValue::String(self.game_id));
        map.insert(ProtocolValue::String("roomID"), ProtocolValue::String(self.room_id));
        map.insert(ProtocolValue::String("storeID"), ProtocolValue::String(self.store_id));
        map.insert(ProtocolValue::String("roomName"), ProtocolValue::String(self.room_name));
        map.insert(ProtocolValue::String("modeName"), ProtocolValue::String(self.mode_name));
        map.insert(ProtocolValue::String("password"), ProtocolValue::String(self.password));
        map.insert(ProtocolValue::String("mapName"), ProtocolValue::String(self.map_name));
        map.insert(ProtocolValue::String("matchStarted"), ProtocolValue::Bool(self.match_started));
        map.insert(ProtocolValue::String("switchingmap"), ProtocolValue::Bool(self.switching_map));
        map.insert(ProtocolValue::String("roomType"), ProtocolValue::Byte(self.room_type));
        map.insert(ProtocolValue::String("dedicated"), ProtocolValue::Bool(self.dedicated));
        map.insert(ProtocolValue::String("hardcore"), ProtocolValue::Bool(self.hardcore));
        map.insert(
            ProtocolValue::String("allowedweapons"),
            ProtocolValue::Array(vec![
                ProtocolValue::Integer((self.allowed_weapons & 0xFFFFFFFF) as u32),
                ProtocolValue::Integer((self.allowed_weapons >> 32) as u32),
            ]),
        );
        map.insert(
            ProtocolValue::String("meanRank"),
            match self.mean_rank {
                Either::Left(x) => ProtocolValue::Integer(x),
                Either::Right(x) => ProtocolValue::Float(x),
            },
        );
        map.insert(ProtocolValue::String("meanKD"), ProtocolValue::Float(self.mean_kd));
        map.insert(ProtocolValue::String("averagerank"), ProtocolValue::Integer(self.average_rank));
        map.insert(ProtocolValue::String("eventcode"), ProtocolValue::Integer(self.event_code));
        map.insert(ProtocolValue::Byte(252), ProtocolValue::Byte(self.byte_252));
        map.insert(ProtocolValue::Byte(253), ProtocolValue::Bool(self.byte_253));
        map.insert(ProtocolValue::Byte(255), ProtocolValue::Byte(self.byte_255));
        map
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for GameProperties<'s> {
    type Error = PacketReadError;

    fn try_from(mut table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<GameProperties<'s>> {
        let ret = Ok(GameProperties {
            spectate_for_mods_only: get_protocol_bool(&mut table, ProtocolValue::String("spectateForModsOnly"))?,
            max_ping: get_protocol_short(&mut table, ProtocolValue::String("maxPing"))?,
            banned_weapon_message: get_protocol_string(&mut table, ProtocolValue::String("bannedweaponmessage"))?,
            time_scale: get_protocol_float(&mut table, ProtocolValue::String("timeScale"))?,
            match_countdown_time: get_protocol_float(&mut table, ProtocolValue::String("matchCountdownTime"))?,
            round_started: get_protocol_bool(&mut table, ProtocolValue::String("roundStarted"))?,
            score_limit: get_protocol_int(&mut table, ProtocolValue::String("scorelimit"))?,
            gun_game_preset: get_protocol_int(&mut table, ProtocolValue::String("gunGamePreset"))?,
            byte_249: get_protocol_bool(&mut table, ProtocolValue::Byte(249)).ok(),
            byte_250: get_protocol_array(&mut table, ProtocolValue::Byte(250))
                .and_then(|x| {
                    Ok(x.into_iter()
                        .map(|protocol_val| unwrap_protocol_string(protocol_val).expect("Found non-string type in GameProperties::byte_250"))
                        .collect())
                })
                .ok(),
            byte_253: get_protocol_bool(&mut table, ProtocolValue::Byte(253)).ok(),
            byte_254: get_protocol_bool(&mut table, ProtocolValue::Byte(254)).ok(),
            byte_255: get_protocol_byte(&mut table, ProtocolValue::Byte(255)).ok(),
            byte_248: get_protocol_int(&mut table, ProtocolValue::Byte(248)).ok(), // could use direction to conditionally check for this
            room_name: get_protocol_string(&mut table, ProtocolValue::String("roomName"))?,
            map_name: get_protocol_string(&mut table, ProtocolValue::String("mapName"))?,
            mode_name: get_protocol_string(&mut table, ProtocolValue::String("modeName"))?,
            password: get_protocol_string(&mut table, ProtocolValue::String("password"))?,
            hardcore: get_protocol_bool(&mut table, ProtocolValue::String("hardcore"))?,
            dedicated: get_protocol_bool(&mut table, ProtocolValue::String("dedicated"))?,
            match_started: get_protocol_bool(&mut table, ProtocolValue::String("matchStarted"))?,
            mean_kd: get_protocol_float(&mut table, ProtocolValue::String("meanKD"))?,
            mean_rank: get_protocol_int_or_float(&mut table, ProtocolValue::String("meanRank"))?,
            room_type: get_protocol_byte(&mut table, ProtocolValue::String("roomType"))?,
            switching_map: get_protocol_bool(&mut table, ProtocolValue::String("switchingmap"))?,
            allowed_weapons: {
                // this is an array of 2 u32s, but we save this as a u64 because it makes more sense
                let mut arr = get_protocol_array(&mut table, ProtocolValue::String("allowedweapons"))?;
                if arr.len() != 2 {
                    return Err(PacketReadError::Other(format!("allowedweapons array was not 2 long, but {}", arr.len())));
                }
                let int2 = unwrap_protocol_int(arr.remove(1))? as u64;
                let int1 = unwrap_protocol_int(arr.remove(0))? as u64;
                int1 | (int2 << 32)
            },
            event_code: get_protocol_int(&mut table, ProtocolValue::String("eventcode"))?,
            average_rank: get_protocol_int(&mut table, ProtocolValue::String("averagerank"))?,
            game_id: get_protocol_string(&mut table, ProtocolValue::String("gameID"))?,
            room_id: get_protocol_string(&mut table, ProtocolValue::String("roomID"))?,
            store_id: get_protocol_string(&mut table, ProtocolValue::String("storeID"))?,
        });

        if ret.is_ok() && table.len() > 0 {
            warn!("Missed GameProperties parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for GameProperties<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        map.insert(
            ProtocolValue::String("spectateForModsOnly"),
            ProtocolValue::Bool(self.spectate_for_mods_only),
        );
        map.insert(ProtocolValue::String("maxPing"), ProtocolValue::Short(self.max_ping));
        map.insert(
            ProtocolValue::String("bannedweaponmessage"),
            ProtocolValue::String(self.banned_weapon_message),
        );
        map.insert(ProtocolValue::String("timeScale"), ProtocolValue::Float(self.time_scale));
        map.insert(
            ProtocolValue::String("matchCountdownTime"),
            ProtocolValue::Float(self.match_countdown_time),
        );
        map.insert(ProtocolValue::String("roundStarted"), ProtocolValue::Bool(self.round_started));
        map.insert(ProtocolValue::String("scorelimit"), ProtocolValue::Integer(self.score_limit));
        map.insert(ProtocolValue::String("gunGamePreset"), ProtocolValue::Integer(self.gun_game_preset));
        self.byte_249.and_then(|b| map.insert(ProtocolValue::Byte(249), ProtocolValue::Bool(b)));
        self.byte_250.and_then(|b| {
            map.insert(
                ProtocolValue::Byte(250),
                ProtocolValue::Array(b.into_iter().map(|s| ProtocolValue::String(s)).collect()),
            )
        });
        self.byte_253.and_then(|b| map.insert(ProtocolValue::Byte(253), ProtocolValue::Bool(b)));
        self.byte_254.and_then(|b| map.insert(ProtocolValue::Byte(254), ProtocolValue::Bool(b)));
        self.byte_255.and_then(|b| map.insert(ProtocolValue::Byte(255), ProtocolValue::Byte(b)));
        self.byte_248
            .and_then(|b| map.insert(ProtocolValue::Byte(248), ProtocolValue::Integer(b)));
        map.insert(ProtocolValue::String("roomName"), ProtocolValue::String(self.room_name));
        map.insert(ProtocolValue::String("mapName"), ProtocolValue::String(self.map_name));
        map.insert(ProtocolValue::String("modeName"), ProtocolValue::String(self.mode_name));
        map.insert(ProtocolValue::String("password"), ProtocolValue::String(self.password));
        map.insert(ProtocolValue::String("hardcore"), ProtocolValue::Bool(self.hardcore));
        map.insert(ProtocolValue::String("dedicated"), ProtocolValue::Bool(self.dedicated));
        map.insert(ProtocolValue::String("matchStarted"), ProtocolValue::Bool(self.match_started));
        map.insert(ProtocolValue::String("meanKD"), ProtocolValue::Float(self.mean_kd));
        map.insert(
            ProtocolValue::String("meanRank"),
            match self.mean_rank {
                Either::Left(x) => ProtocolValue::Integer(x),
                Either::Right(x) => ProtocolValue::Float(x),
            },
        );
        map.insert(ProtocolValue::String("roomType"), ProtocolValue::Byte(self.room_type));
        map.insert(ProtocolValue::String("switchingmap"), ProtocolValue::Bool(self.switching_map));
        map.insert(
            ProtocolValue::String("allowedweapons"),
            ProtocolValue::Array(vec![
                ProtocolValue::Integer((self.allowed_weapons & 0xFFFFFFFF) as u32),
                ProtocolValue::Integer((self.allowed_weapons >> 32) as u32),
            ]),
        );
        map.insert(ProtocolValue::String("eventcode"), ProtocolValue::Integer(self.event_code));
        map.insert(ProtocolValue::String("averagerank"), ProtocolValue::Integer(self.average_rank));
        map.insert(ProtocolValue::String("gameID"), ProtocolValue::String(self.game_id));
        map.insert(ProtocolValue::String("roomID"), ProtocolValue::String(self.room_id));
        map.insert(ProtocolValue::String("storeID"), ProtocolValue::String(self.store_id));
        map
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for PlayerProperties<'s> {
    type Error = PacketReadError;

    fn try_from<'a>(mut table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>) -> PacketReadResult<PlayerProperties<'a>> {
        if table.len() != 1 || !table.contains_key(&ProtocolValue::Byte(255)) {
            return Err(PacketReadError::Other("Full PlayerProperties not yet implemented!".to_string()));
        }
        let ret = Ok(PlayerProperties::NameOnly(get_protocol_string(&mut table, ProtocolValue::Byte(255))?));

        // can't be hit, actually
        if ret.is_ok() && table.len() > 0 {
            warn!("Missed PlayerProperties parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for PlayerProperties<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        match self {
            PlayerProperties::NameOnly(name) => map.insert(ProtocolValue::Byte(255), ProtocolValue::String(name)),
        };
        map
    }
}
