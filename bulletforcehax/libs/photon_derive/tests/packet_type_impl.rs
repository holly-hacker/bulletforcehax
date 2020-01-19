#![cfg(test)]

use photon_derive::PacketTypeImpl;

#[allow(dead_code)]
#[derive(PacketTypeImpl)]
enum MyPacketType<'a> {
    #[packet_type(0)]
    Unimplemented,
    #[packet_type(1)]
    NoValues(),
    #[packet_type(2)]
    WithUnnamedValues(i32),
    #[packet_type(3)]
    StructWithInt { val: i32 },
    #[packet_type(4)]
    VariantWithLifetime(&'a str),
}

#[test]
fn test_get_type() {
    assert_eq!(MyPacketType::Unimplemented.get_type(), 0);
    assert_eq!(MyPacketType::NoValues().get_type(), 1);
    assert_eq!(MyPacketType::WithUnnamedValues(1).get_type(), 2);
    assert_eq!(MyPacketType::StructWithInt { val: 1 }.get_type(), 3);
    assert_eq!(MyPacketType::VariantWithLifetime("test").get_type(), 4);
}
