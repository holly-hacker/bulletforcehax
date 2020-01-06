#[cfg(test)]
mod writing_protocol_types_tests {
    use super::super::super::*;
    use super::super::*;

    #[test]
    fn can_write_null() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Null())?;
        assert_eq!(writer, &vec![0x2a]);
        Ok(())
    }

    #[test]
    fn can_write_bool() -> PhotonWriteResult<()> {
        let ref mut writer1 = Vec::new();
        write_value_of_type(writer1, ProtocolValue::Bool(false))?;
        let ref mut writer2 = Vec::new();
        write_value_of_type(writer2, ProtocolValue::Bool(true))?;

        assert_eq!(writer1, &vec![0x6f, 0x00]);
        assert_eq!(writer2, &vec![0x6f, 0x01]);
        Ok(())
    }

    #[test]
    fn can_write_u8() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Byte(0x90))?;
        assert_eq!(writer, &vec![0x62, 0x90]);
        Ok(())
    }

    #[test]
    fn can_write_s16() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Short((-1337 as i16) as u16))?;
        assert_eq!(writer, &vec![0x6b, 0xFA, 0xC7]);
        Ok(())
    }

    #[test]
    fn can_write_s32() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Integer((-559038737 as i32) as u32))?;
        assert_eq!(writer, &vec![0x69, 0xDE, 0xAD, 0xBE, 0xEF]);
        Ok(())
    }

    #[test]
    fn can_write_s64() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Long((-3886136854700967234 as i64) as u64))?;
        assert_eq!(writer, &vec![0x6c, 0xCA, 0x11, 0xAB, 0x1E, 0xCA, 0xFE, 0xBA, 0xBE]);
        Ok(())
    }

    #[test]
    fn can_write_f32() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Float(42.))?;
        assert_eq!(writer, &vec![0x66, 0x42, 0x28, 0x00, 0x00]);
        Ok(())
    }

    #[test]
    fn can_write_f64() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Double(13.37))?;
        assert_eq!(writer, &vec![0x64, 0x40, 0x2a, 0xbd, 0x70, 0xa3, 0xd7, 0x0a, 0x3d]);
        Ok(())
    }

    #[test]
    fn can_write_strings() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::String("abc"))?;
        assert_eq!(writer, &vec![0x73, 0x00, 0x03, 0x61, 0x62, 0x63]);
        Ok(())
    }
    #[test]
    fn can_write_unicode_strings() -> PhotonWriteResult<()> {
        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::String("abc»d"))?;
        assert_eq!(writer, &vec![0x73, 0x00, 0x06, 0x61, 0x62, 0x63, 0xc2, 0xbb, 0x64]);
        Ok(())
    }

    #[test]
    #[ignore = "ByteArray not yet implemented"]
    fn can_write_byte_array() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        // write_value_of_type(writer, ProtocolValue::ByteArray([0xDE, 0xAD, 0xBE, 0xEF]))?;
        assert_eq!(writer, &vec![120, 0, 0, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]);
        Ok(())
    }

    #[test]
    #[ignore = "IntegerArray not yet implemented"]
    fn can_write_int_array() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        // write_value_of_type(writer, ProtocolValue::IntegerArray([-559038737, -889275714]))?;
        assert_eq!(writer, &vec![110, 0, 0, 0, 2, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE]);
        Ok(())
    }

    #[test]
    #[ignore = "StringArray not yet implemented"]
    fn can_write_string_array() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        // write_value_of_type(writer, ProtocolValue::StringArray(["abc", ""])?;
        assert_eq!(writer, &vec![97, 0, 2, 0, 3, 0x61, 0x62, 0x63, 0, 0]);
        Ok(())
    }

    #[test]
    fn can_write_array() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        write_value_of_type(
            writer,
            ProtocolValue::Array(vec![ProtocolValue::Bool(true), ProtocolValue::Bool(false), ProtocolValue::Bool(true)]),
        )?;
        assert_eq!(writer, &vec![121, 0, 3, 111, 1, 0, 1]);
        Ok(())
    }

    #[test]
    #[ignore = "ObjectArray not yet implemented"]
    fn can_write_object_array() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        // write_value_of_type(writer, ProtocolValue::ObjectArray(["abc", null, s16(0x123)])?;
        assert_eq!(writer, &vec![122, 0, 3, 115, 0, 3, 0x61, 0x62, 0x63, 42, 107, 0x01, 0x23]);
        Ok(())
    }

    #[test]
    fn can_write_hashtable() -> PhotonWriteResult<()> {
        let mut expected_map = HashMap::new();
        expected_map.insert(ProtocolValue::String("abc"), ProtocolValue::Bool(true));
        expected_map.insert(ProtocolValue::Byte(0xFF), ProtocolValue::Null());

        let ref mut writer = Vec::new();
        write_value_of_type(writer, ProtocolValue::Hashtable(expected_map))?;

        // hashmap order is not stable, so there are 2 distinct possibilities we check for
        assert_eq!(14, writer.len());
        match writer[3] {
            115 => assert_eq!(
                writer,
                &vec![0x68, 0x00, 0x02, 115, 0x00, 0x03, 0x61, 0x62, 0x63, 111, 0x01, 98, 0xFF, 42]
            ),
            98 => assert_eq!(
                writer,
                &vec![0x68, 0x00, 0x02, 98, 0xFF, 42, 115, 0x00, 0x03, 0x61, 0x62, 0x63, 111, 0x01]
            ),
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    #[ignore = "Customdata not yet implemented"]
    fn can_write_custom_data() -> PhotonWriteResult<()> {
        let ref mut writer: Vec<u8> = Vec::new();
        // write_value_of_type(writer, UnimplementedCustomData(42, &vec![0xDE, 0xAD, 0xBE, 0xEF]))?;
        assert_eq!(writer, &vec![99, 42, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]);
        Ok(())
    }
}
