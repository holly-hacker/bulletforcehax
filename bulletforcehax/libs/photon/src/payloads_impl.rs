use super::macros::*;
use super::*;
use std::collections::HashMap;
use std::convert::TryFrom;

impl<'s> RoomInfo<'s> {
    pub fn try_from_hashtable_table<'a>(
        big_table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    ) -> PacketReadResult<HashMap<&'a str, Option<RoomInfo<'a>>>> {
        let mut map: HashMap<&'a str, Option<RoomInfo<'a>>> = HashMap::new();
        for (key, value) in big_table {
            // could look into getting map past the borrow checker
            let ht = unwrap_protocol_hashtable(value)?;
            let val = RoomInfo::try_from_hashtable(ht)?;
            map.insert(unwrap_protocol_string(key)?, val);
        }

        Ok(map)
    }

    pub fn try_from_hashtable(table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<Option<RoomInfo<'s>>> {
        if table.contains_key(&ProtocolValue::Byte(GamePropertyKey::Removed)) {
            // got removed
            return Ok(None);
        }

        Some(RoomInfo::try_from(table)).transpose()
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for RoomInfo<'s> {
    type Error = PacketReadError;

    fn try_from(mut table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<Self> {
        Ok(RoomInfo {
            max_players: get_protocol_byte_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::MaxPlayers))?.unwrap_or(0),
            is_open: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::IsVisible))?.unwrap_or(true),
            is_visible: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::IsOpen))?.unwrap_or(true),
            player_count: get_protocol_byte_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::PlayerCount))?.unwrap_or(0),
            cleanup_cache_on_leave: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::CleanupCacheOnLeave))?.unwrap_or(true),
            master_client_id: get_protocol_int_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::MasterClientId))?,
            custom_properties_lobby: get_protocol_array_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::PropsListedInLobby))?
                .map(|arr| arr.into_iter().map(unwrap_protocol_string).collect())
                .transpose()?
                .unwrap_or_else(|| vec![]),
            expected_users: get_protocol_array_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::ExpectedUsers))?
                .map(|arr| arr.into_iter().map(unwrap_protocol_string).collect())
                .transpose()?
                .unwrap_or_else(|| vec![]),
            empty_room_ttl: get_protocol_int_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::EmptyRoomTtl))?.unwrap_or(0),
            player_ttl: get_protocol_int_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::PlayerTtl))?.unwrap_or(0),

            // all remaining properties into custom_properties, assume they use string keys
            // note: maybe I should make this return an error instead of panicking on non-string keys
            custom_properties: table
                .into_iter()
                .map(|(key, value)| (unwrap_protocol_string(key).expect("Expected string key for custom_properties"), value))
                .collect(),
        })
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for RoomInfo<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        // TODO: don't write if default value
        map.insert(ProtocolValue::Byte(GamePropertyKey::MaxPlayers), ProtocolValue::Byte(self.max_players));
        map.insert(ProtocolValue::Byte(GamePropertyKey::IsVisible), ProtocolValue::Bool(self.is_open));
        map.insert(ProtocolValue::Byte(GamePropertyKey::IsOpen), ProtocolValue::Bool(self.is_visible));
        map.insert(ProtocolValue::Byte(GamePropertyKey::PlayerCount), ProtocolValue::Byte(self.player_count));
        map.insert(
            ProtocolValue::Byte(GamePropertyKey::CleanupCacheOnLeave),
            ProtocolValue::Bool(self.cleanup_cache_on_leave),
        );
        self.master_client_id
            .and_then(|id| map.insert(ProtocolValue::Byte(GamePropertyKey::MasterClientId), ProtocolValue::Integer(id)));
        if !self.custom_properties_lobby.is_empty() {
            map.insert(
                ProtocolValue::Byte(GamePropertyKey::PropsListedInLobby),
                ProtocolValue::Array(self.custom_properties_lobby.into_iter().map(ProtocolValue::String).collect()),
            );
        }
        if !self.expected_users.is_empty() {
            map.insert(
                ProtocolValue::Byte(GamePropertyKey::ExpectedUsers),
                ProtocolValue::Array(self.expected_users.into_iter().map(ProtocolValue::String).collect()),
            );
        }
        map.insert(
            ProtocolValue::Byte(GamePropertyKey::EmptyRoomTtl),
            ProtocolValue::Integer(self.empty_room_ttl),
        );
        map.insert(ProtocolValue::Byte(GamePropertyKey::PlayerTtl), ProtocolValue::Integer(self.player_ttl));

        // Add the remaining properties
        let remaining: HashMap<ProtocolValue, ProtocolValue> = self
            .custom_properties
            .into_iter()
            .map(|(key, value)| (ProtocolValue::String(key), value))
            .collect();
        map.extend(remaining);
        map
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for RoomOptions<'s> {
    type Error = PacketReadError;

    fn try_from(mut table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<Self> {
        Ok(RoomOptions {
            max_players: get_protocol_byte_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::MaxPlayers))?.unwrap_or(0),
            is_open: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::IsVisible))?.unwrap_or(true),
            is_visible: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::IsOpen))?.unwrap_or(true),
            cleanup_cache_on_leave: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::CleanupCacheOnLeave))?.unwrap_or(true),
            custom_properties_lobby: get_protocol_array_opt(&mut table, ProtocolValue::Byte(GamePropertyKey::PropsListedInLobby))?
                .map(|arr| arr.into_iter().map(unwrap_protocol_string).collect())
                .transpose()?
                .unwrap_or_else(|| vec![]),

            // all remaining properties into custom_properties, assume they use string keys
            // note: maybe I should make this return an error instead of panicking on non-string keys
            custom_properties: table
                .into_iter()
                .map(|(key, value)| (unwrap_protocol_string(key).expect("Expected string key for custom_properties"), value))
                .collect(),
        })
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for RoomOptions<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        // TODO: don't write if default value
        if self.max_players != 0 {
            map.insert(ProtocolValue::Byte(GamePropertyKey::MaxPlayers), ProtocolValue::Byte(self.max_players));
        }
        map.insert(ProtocolValue::Byte(GamePropertyKey::IsVisible), ProtocolValue::Bool(self.is_open));
        map.insert(ProtocolValue::Byte(GamePropertyKey::IsOpen), ProtocolValue::Bool(self.is_visible));
        map.insert(
            ProtocolValue::Byte(GamePropertyKey::CleanupCacheOnLeave),
            ProtocolValue::Bool(self.cleanup_cache_on_leave),
        );
        map.insert(
            ProtocolValue::Byte(GamePropertyKey::PropsListedInLobby),
            ProtocolValue::Array(self.custom_properties_lobby.into_iter().map(ProtocolValue::String).collect()),
        );

        // Add the remaining properties
        let remaining: HashMap<ProtocolValue, ProtocolValue> = self
            .custom_properties
            .into_iter()
            .map(|(key, value)| (ProtocolValue::String(key), value))
            .collect();
        map.extend(remaining);
        map
    }
}

impl<'s> TryFrom<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for Player<'s> {
    type Error = PacketReadError;

    fn try_from(mut table: HashMap<ProtocolValue<'s>, ProtocolValue<'s>>) -> PacketReadResult<Self> {
        Ok(Player {
            name: get_protocol_string_opt(&mut table, ProtocolValue::Byte(ActorProperties::PlayerName))?,
            user_id: get_protocol_string_opt(&mut table, ProtocolValue::Byte(ActorProperties::UserId))?,
            is_inactive: get_protocol_bool_opt(&mut table, ProtocolValue::Byte(ActorProperties::IsInactive))?,

            // all remaining properties into custom_properties, assume they use string keys
            // note: maybe I should make this return an error instead of panicking on non-string keys
            custom_properties: table
                .into_iter()
                .map(|(key, value)| (unwrap_protocol_string(key).expect("Expected string key for custom_properties"), value))
                .collect(),
        })
    }
}

impl<'s> Into<HashMap<ProtocolValue<'s>, ProtocolValue<'s>>> for Player<'s> {
    fn into(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();

        self.name
            .and_then(|n| map.insert(ProtocolValue::Byte(ActorProperties::PlayerName), ProtocolValue::String(n)));
        self.user_id
            .and_then(|n| map.insert(ProtocolValue::Byte(ActorProperties::UserId), ProtocolValue::String(n)));
        self.is_inactive
            .and_then(|i| map.insert(ProtocolValue::Byte(ActorProperties::IsInactive), ProtocolValue::Bool(i)));

        // Add the remaining properties
        let remaining: HashMap<ProtocolValue, ProtocolValue> = self
            .custom_properties
            .into_iter()
            .map(|(key, value)| (ProtocolValue::String(key), value))
            .collect();
        map.extend(remaining);

        map
    }
}
