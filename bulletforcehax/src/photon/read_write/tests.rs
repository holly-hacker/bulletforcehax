#![allow(clippy::float_cmp)]
#![cfg(test)]

use super::super::*;
use super::*;
use maplit::hashmap;
use std::io::Cursor;

// union error of PhotonReadError and PhotonWriteError
#[derive(Debug)]
enum TestError {
    Read(PhotonReadError),
    Write(PhotonWriteError),
}

impl From<PhotonReadError> for TestError {
    fn from(err: PhotonReadError) -> Self {
        TestError::Read(err)
    }
}

impl From<PhotonWriteError> for TestError {
    fn from(err: PhotonWriteError) -> Self {
        TestError::Write(err)
    }
}

type TestResult<T> = Result<T, TestError>;

macro_rules! generate_read_write_test {
    ($test_name: ident, $obj: expr, $data: expr) => {
        paste::item! {
            #[test]
            fn [<can_read_ $test_name>]() -> TestResult<()> {
                let reader = &mut Cursor::new($data.as_ref());
                let t = read_value(reader)?;
                assert_eq!(t, $obj);

                Ok(())
            }

            #[test]
            fn [<can_write_ $test_name>]() -> TestResult<()> {
                let writer = &mut Vec::new();
                write_value_of_type(writer, $obj)?;
                assert_eq!(writer, &$data);

                Ok(())
            }
        }
    };
}

generate_read_write_test!(null, ProtocolValue::Null(), [0x2au8]);
generate_read_write_test!(bool_true, ProtocolValue::Bool(true), [0x6f, 0x01]);
generate_read_write_test!(bool_false, ProtocolValue::Bool(false), [0x6f, 0x00]);
generate_read_write_test!(u8, ProtocolValue::Byte(0x90), [0x62, 0x90]);
generate_read_write_test!(s16, ProtocolValue::Short(-1337), [0x6b, 0xFA, 0xC7]);
generate_read_write_test!(s32, ProtocolValue::Integer(-559038737), [0x69, 0xDE, 0xAD, 0xBE, 0xEF]);
generate_read_write_test!(
    s64,
    ProtocolValue::Long(-3886136854700967234),
    [0x6c, 0xCA, 0x11, 0xAB, 0x1E, 0xCA, 0xFE, 0xBA, 0xBE]
);
generate_read_write_test!(f32, ProtocolValue::Float(42.), [0x66, 0x42, 0x28, 0x00, 0x00]);
generate_read_write_test!(f64, ProtocolValue::Double(13.37), [0x64, 0x40, 0x2a, 0xbd, 0x70, 0xa3, 0xd7, 0x0a, 0x3d]);
generate_read_write_test!(string, ProtocolValue::String("abc"), [0x73, 0x00, 0x03, 0x61, 0x62, 0x63]);
generate_read_write_test!(
    string_unicode,
    ProtocolValue::String("abc»d"),
    [0x73, 0x00, 0x06, 0x61, 0x62, 0x63, 0xc2, 0xbb, 0x64]
);
generate_read_write_test!(
    byte_array,
    ProtocolValue::ByteArray(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    [120, 0, 0, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]
);
generate_read_write_test!(
    int_array,
    ProtocolValue::IntegerArray(vec![-559038737, -889275714]),
    [110, 0, 0, 0, 2, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE]
);
generate_read_write_test!(
    string_array,
    ProtocolValue::StringArray(vec!["abc", ""]),
    [97, 0, 2, 0, 3, 0x61, 0x62, 0x63, 0, 0]
);
generate_read_write_test!(
    array,
    ProtocolValue::Array(vec![ProtocolValue::Bool(true), ProtocolValue::Bool(false), ProtocolValue::Bool(true)]),
    [121, 0, 3, 111, 1, 0, 1]
);
generate_read_write_test!(
    object_array,
    ProtocolValue::ObjectArray(vec![ProtocolValue::String("abc"), ProtocolValue::Null(), ProtocolValue::Short(0x123)]),
    [122, 0, 3, 115, 0, 3, 0x61, 0x62, 0x63, 42, 107, 0x01, 0x23]
);

// hashtable can only have 1 item because order is not deterministic
generate_read_write_test!(
    hashtable,
    ProtocolValue::Hashtable(hashmap! { ProtocolValue::Byte(0xFF) => ProtocolValue::Null(), }),
    [0x68, 0x00, 0x01, 98, 0xFF, 42]
);

// TODO: Dictionary test
// TODO: EventData test
// TODO: OperationResponse test

generate_read_write_test!(
    vec2,
    ProtocolValue::Custom(CustomType::Vector2(1., 13.37)),
    [99, 0x57, 0, 8, 0x3f, 0x80, 0x00, 0x00, 0x41, 0x55, 0xeb, 0x85]
);
generate_read_write_test!(
    vec3,
    ProtocolValue::Custom(CustomType::Vector3(1., 0., 13.37)),
    [99, 0x56, 0, 12, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x55, 0xeb, 0x85]
);
generate_read_write_test!(
    quaternion,
    ProtocolValue::Custom(CustomType::Quaternion(1., 0., 13.37, 16.)),
    [99, 0x51, 0, 16, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x55, 0xeb, 0x85, 0x41, 0x80, 0x00, 0x00,]
);
generate_read_write_test!(
    player,
    ProtocolValue::Custom(CustomType::Player(0x00C0FFEE)),
    [99, 0x50, 0, 4, 0x00, 0xC0, 0xFF, 0xEE]
);
generate_read_write_test!(
    other_custom,
    ProtocolValue::Custom(CustomType::Custom {
        id: 15,
        data: vec![0xDE, 0xAD, 0xBE, 0xEF]
    }),
    [99, 15, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]
);
