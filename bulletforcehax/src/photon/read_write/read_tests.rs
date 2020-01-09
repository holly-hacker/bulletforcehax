#[cfg(test)]
mod reading_protocol_types_tests {
    use super::super::super::*;
    use super::super::*;
    use std::io::Cursor;

    #[test]
    fn can_read_null() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x2au8].as_ref());
        let t = read_value(reader)?;
        assert_eq!(t, ProtocolValue::Null());
        Ok(())
    }

    // TODO: Dictionary test
    // TODO: EventData test
    // TODO: OperationResponse test
    // TODO: OperationRequest test

    #[test]
    fn can_read_bool() -> PhotonReadResult<()> {
        let t1 = read_value(&mut Cursor::new([0x6f, 0x00].as_ref()))?;
        let t2 = read_value(&mut Cursor::new([0x6f, 0x01].as_ref()))?;
        assert_eq!(t1, ProtocolValue::Bool(false));
        assert_eq!(t2, ProtocolValue::Bool(true));
        Ok(())
    }

    #[test]
    fn can_read_u8() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x62, 0x90].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Byte(0x90));
        Ok(())
    }

    #[test]
    fn can_read_s16() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x6b, 0xFA, 0xC7].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Short((-1337 as i16) as u16));
        Ok(())
    }

    #[test]
    fn can_read_s32() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x69, 0xDE, 0xAD, 0xBE, 0xEF].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Integer((-559038737 as i32) as u32));
        Ok(())
    }

    #[test]
    fn can_read_s64() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x6c, 0xCA, 0x11, 0xAB, 0x1E, 0xCA, 0xFE, 0xBA, 0xBE].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Long((-3886136854700967234 as i64) as u64));
        Ok(())
    }

    #[test]
    fn can_read_f32() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x66, 0x42, 0x28, 0x00, 0x00].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Float(42.));
        Ok(())
    }

    #[test]
    fn can_read_f64() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x64, 0x40, 0x2a, 0xbd, 0x70, 0xa3, 0xd7, 0x0a, 0x3d].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::Double(13.37));
        Ok(())
    }

    #[test]
    fn can_read_strings() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x73, 0x00, 0x03, 0x61, 0x62, 0x63].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::String("abc"));
        Ok(())
    }
    #[test]
    fn can_read_unicode_strings() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x73, 0x00, 0x06, 0x61, 0x62, 0x63, 0xc2, 0xbb, 0x64].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::String("abc»d"));
        Ok(())
    }

    #[test]
    fn can_read_byte_array() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([120, 0, 0, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::ByteArray(vec![0xDE, 0xAD, 0xBE, 0xEF]));
        Ok(())
    }

    #[test]
    fn can_read_int_array() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([110, 0, 0, 0, 2, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::IntegerArray(vec![-559038737 as i32 as u32, -889275714 as i32 as u32]));
        Ok(())
    }

    #[test]
    fn can_read_string_array() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([97, 0, 2, 0, 3, 0x61, 0x62, 0x63, 0, 0].as_ref());
        let t = read_value(reader)?;

        assert_eq!(t, ProtocolValue::StringArray(vec!["abc", ""]));
        Ok(())
    }

    #[test]
    fn can_read_array() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([121, 0, 3, 111, 1, 0, 1].as_ref());
        let t = read_value(reader)?;

        assert_eq!(
            t,
            ProtocolValue::Array(vec![ProtocolValue::Bool(true), ProtocolValue::Bool(false), ProtocolValue::Bool(true)])
        );
        Ok(())
    }

    #[test]
    fn can_read_object_array() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([122, 0, 3, 115, 0, 3, 0x61, 0x62, 0x63, 42, 107, 0x01, 0x23].as_ref());
        let t = read_value(reader)?;

        assert_eq!(
            t,
            ProtocolValue::ObjectArray(vec![ProtocolValue::String("abc"), ProtocolValue::Null(), ProtocolValue::Short(0x123)])
        );
        Ok(())
    }

    #[test]
    fn can_read_hashtable() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([0x68, 0x00, 0x02, 98, 0xFF, 42, 115, 0x00, 0x03, 0x61, 0x62, 0x63, 111, 0x01].as_ref());
        let t = read_value(reader)?;

        let mut expected_map = HashMap::new();
        expected_map.insert(ProtocolValue::Byte(0xFF), ProtocolValue::Null());
        expected_map.insert(ProtocolValue::String("abc"), ProtocolValue::Bool(true));

        assert_eq!(t, ProtocolValue::Hashtable(expected_map));

        Ok(())
    }

    #[test]
    #[ignore = "Custom not yet implemented"]
    fn can_read_custom() -> PhotonReadResult<()> {
        let reader = &mut Cursor::new([99, 42, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF].as_ref());
        let _t = read_value(reader)?;
        Ok(())
    }
}
