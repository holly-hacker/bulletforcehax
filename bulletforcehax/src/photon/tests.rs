#![cfg(test)]

use super::*;

const TEST_TOKEN_NAME: &str = "AulCdG5VnAHZvN+szlhEdQ05tkCjp41x0e/KgxUezpfEvhgGsuR5AOEbYuurSm1omxSDuwfPUNlgHoOBdPXj2Aj6CcB7A13C/o0/jSnIwxfQQi+rJR8P+dH8EHtbLEzZhLTHe/lWLnXmEKPlL3xBlrWM42/iSqu1/L2LN/+2StPNYnXJXQTGcacKp6piFm/HbVs8EbwQBSk3lOxV/D1xuZPkTreBPQMtOa2i+Z/8EB1EyNNRBocu4RPzMy+jhY7GIM3ZShoyib+miMUcoywV4uOkqN4gNz0wweVgtk/tT3gZ+4TUQuQZtNu3Z+sAbaplQkrQQaQJ0ul2WcXw/Wd7Voe3MfIw+EnUv4jbSiYW2l+D";
const TEST_TOKEN_MASTER: &str = "eHWX1lVV2VC8wfiwZP11NkMW8Vnni4zZqJfeIwhdqYFNAiqY9J6Shz/UvBD46oypAy+U0ZthuWJSjlwRvJ2YI2uOSaBjsOPXXaHCIVzYLWtkLn0ow0GCDUaJ4tTyN0lBK5s4gTQRaICJDey6YOU1vEPRvy0DDt5z4RkLcM+GklZdEVzvXMERwoSqBBeAiHYyBcxlDqDdeLLYmcRPFq4QinH1vy1h8YtjplpTXYzYe6wDM2qVA3F+KMXlBUSM9pZXzFlI6OrIhUPNl60UkTlzCcWAsFJTuSvCgzLnuFZevq8=";
const TEST_TOKEN_MASTER2: &str = "eHWX1lVV2VC8wfiwZP11NroXhq5d0Yiy5G10tlSd90zotI40MehGTD7WsqPixps1Fazbe8TsH1y0U2PqiyNydxC2urFZdxZ+n2herPDvDfqy3sEr2j9cb7JaT0bFe9HivCj6HS0FW5APi/+icueQs/TxcjAdfniNxEbHhAEBeoiv0ikiaMcxCIOqmddbCgIfu0Hkgbe1oylZ8DLC+x4wJ0qavRyj5Atjvs5+2zsTgUQ1wNcxo3Kzp4/19g5U5KllEDXHvQWECIo877uXcv9mfuSAZCtVA9C1lAV9YFQh49k=";

macro_rules! gen_test {
    ($name: ident, $direction: path, $data: expr, $unpacked: expr) => {
        #[test]
        fn $name() {
            let data = $data;
            let expected = $unpacked;

            let packet = Packet::read(data.as_slice(), $direction).expect("Failure while reading packet from bytes");

            assert_eq!(packet, expected);

            // now test if serializing and deserializing it gives the original value again
            let serialized_expected = expected.into_vec().expect("Failure while turning packet back into bytes");
            assert_eq!(
                $data.len(),
                serialized_expected.len(),
                "Expected original bytes to have same length as serialized packet. You probably missed some data."
            );
            let new_expected = Packet::read(serialized_expected.as_slice(), $direction).expect("Failure while reading serialized packet back");
            assert_eq!(new_expected, $unpacked, "Deserialized packet differs from expected packet");
        }
    };
}

gen_test!(
    ping_request,
    Direction::Send,
    vec![0xf3, 0x6, 0x1, 0x0, 0x1, 0x1, 0x69, 0x0, 0x0, 0x2d, 0x51],
    Packet::InternalOperationRequest(InternalOperation::PingRequest { local_time: 11601 })
);

gen_test!(
    ping_response,
    Direction::Recv,
    vec![0xf3, 0x7, 0x1, 0x0, 0x0, 0x2a, 0x0, 0x2, 0x1, 0x69, 0x0, 0x0, 0x2d, 0x51, 0x2, 0x69, 0x92, 0x87, 0xd3, 0xc5,],
    Packet::InternalOperationResponse(
        InternalOperation::PingResponse {
            local_time: 11601,
            server_time: -1836592187,
        },
        0,
        None
    )
);

// Simulates auth request to name server
gen_test!(
    auth_request_no_token,
    Direction::Send,
    vec![
        0xf3, 0x2, 0xe6, 0x0, 0x3, 0xdc, 0x73, 0x0, 0xb, 0x31, 0x2e, 0x36, 0x37, 0x2e, 0x30, 0x5f, 0x31, 0x2e, 0x39, 0x39, 0xe0, 0x73, 0x0, 0x24,
        0x38, 0x63, 0x32, 0x63, 0x61, 0x64, 0x33, 0x65, 0x2d, 0x32, 0x65, 0x33, 0x66, 0x2d, 0x34, 0x39, 0x34, 0x31, 0x2d, 0x39, 0x30, 0x34, 0x34,
        0x2d, 0x62, 0x33, 0x39, 0x30, 0x66, 0x66, 0x32, 0x63, 0x34, 0x39, 0x35, 0x36, 0xd2, 0x73, 0x0, 0x2, 0x75, 0x73,
    ],
    Packet::OperationRequest(Operation::AuthenticateRequestNoToken {
        lobby_stats: false,
        app_version: "1.67.0_1.99",
        app_id: "8c2cad3e-2e3f-4941-9044-b390ff2c4956",
        region: Some("us"),
        user_id: None,
        client_auth_type: None,
        client_auth_params: None,
        client_auth_data: None,
    })
);

// Simulates auth response from name server
gen_test!(
    auth_response_name,
    Direction::Recv,
    vec![
        0xf3, 0x3, 0xe6, 0x0, 0x0, 0x2a, 0x0, 0x4, 0xc4, 0x73, 0x0, 0x7, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0xe6, 0x73, 0x0, 0x22, 0x77, 0x73,
        0x73, 0x3a, 0x2f, 0x2f, 0x47, 0x43, 0x41, 0x53, 0x48, 0x30, 0x31, 0x33, 0x2e, 0x65, 0x78, 0x69, 0x74, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x2e,
        0x63, 0x6f, 0x6d, 0x3a, 0x31, 0x39, 0x30, 0x39, 0x30, 0xdd, 0x73, 0x1, 0x6c, 0x41, 0x75, 0x6c, 0x43, 0x64, 0x47, 0x35, 0x56, 0x6e, 0x41,
        0x48, 0x5a, 0x76, 0x4e, 0x2b, 0x73, 0x7a, 0x6c, 0x68, 0x45, 0x64, 0x51, 0x30, 0x35, 0x74, 0x6b, 0x43, 0x6a, 0x70, 0x34, 0x31, 0x78, 0x30,
        0x65, 0x2f, 0x4b, 0x67, 0x78, 0x55, 0x65, 0x7a, 0x70, 0x66, 0x45, 0x76, 0x68, 0x67, 0x47, 0x73, 0x75, 0x52, 0x35, 0x41, 0x4f, 0x45, 0x62,
        0x59, 0x75, 0x75, 0x72, 0x53, 0x6d, 0x31, 0x6f, 0x6d, 0x78, 0x53, 0x44, 0x75, 0x77, 0x66, 0x50, 0x55, 0x4e, 0x6c, 0x67, 0x48, 0x6f, 0x4f,
        0x42, 0x64, 0x50, 0x58, 0x6a, 0x32, 0x41, 0x6a, 0x36, 0x43, 0x63, 0x42, 0x37, 0x41, 0x31, 0x33, 0x43, 0x2f, 0x6f, 0x30, 0x2f, 0x6a, 0x53,
        0x6e, 0x49, 0x77, 0x78, 0x66, 0x51, 0x51, 0x69, 0x2b, 0x72, 0x4a, 0x52, 0x38, 0x50, 0x2b, 0x64, 0x48, 0x38, 0x45, 0x48, 0x74, 0x62, 0x4c,
        0x45, 0x7a, 0x5a, 0x68, 0x4c, 0x54, 0x48, 0x65, 0x2f, 0x6c, 0x57, 0x4c, 0x6e, 0x58, 0x6d, 0x45, 0x4b, 0x50, 0x6c, 0x4c, 0x33, 0x78, 0x42,
        0x6c, 0x72, 0x57, 0x4d, 0x34, 0x32, 0x2f, 0x69, 0x53, 0x71, 0x75, 0x31, 0x2f, 0x4c, 0x32, 0x4c, 0x4e, 0x2f, 0x2b, 0x32, 0x53, 0x74, 0x50,
        0x4e, 0x59, 0x6e, 0x58, 0x4a, 0x58, 0x51, 0x54, 0x47, 0x63, 0x61, 0x63, 0x4b, 0x70, 0x36, 0x70, 0x69, 0x46, 0x6d, 0x2f, 0x48, 0x62, 0x56,
        0x73, 0x38, 0x45, 0x62, 0x77, 0x51, 0x42, 0x53, 0x6b, 0x33, 0x6c, 0x4f, 0x78, 0x56, 0x2f, 0x44, 0x31, 0x78, 0x75, 0x5a, 0x50, 0x6b, 0x54,
        0x72, 0x65, 0x42, 0x50, 0x51, 0x4d, 0x74, 0x4f, 0x61, 0x32, 0x69, 0x2b, 0x5a, 0x2f, 0x38, 0x45, 0x42, 0x31, 0x45, 0x79, 0x4e, 0x4e, 0x52,
        0x42, 0x6f, 0x63, 0x75, 0x34, 0x52, 0x50, 0x7a, 0x4d, 0x79, 0x2b, 0x6a, 0x68, 0x59, 0x37, 0x47, 0x49, 0x4d, 0x33, 0x5a, 0x53, 0x68, 0x6f,
        0x79, 0x69, 0x62, 0x2b, 0x6d, 0x69, 0x4d, 0x55, 0x63, 0x6f, 0x79, 0x77, 0x56, 0x34, 0x75, 0x4f, 0x6b, 0x71, 0x4e, 0x34, 0x67, 0x4e, 0x7a,
        0x30, 0x77, 0x77, 0x65, 0x56, 0x67, 0x74, 0x6b, 0x2f, 0x74, 0x54, 0x33, 0x67, 0x5a, 0x2b, 0x34, 0x54, 0x55, 0x51, 0x75, 0x51, 0x5a, 0x74,
        0x4e, 0x75, 0x33, 0x5a, 0x2b, 0x73, 0x41, 0x62, 0x61, 0x70, 0x6c, 0x51, 0x6b, 0x72, 0x51, 0x51, 0x61, 0x51, 0x4a, 0x30, 0x75, 0x6c, 0x32,
        0x57, 0x63, 0x58, 0x77, 0x2f, 0x57, 0x64, 0x37, 0x56, 0x6f, 0x65, 0x33, 0x4d, 0x66, 0x49, 0x77, 0x2b, 0x45, 0x6e, 0x55, 0x76, 0x34, 0x6a,
        0x62, 0x53, 0x69, 0x59, 0x57, 0x32, 0x6c, 0x2b, 0x44, 0xe1, 0x73, 0x0, 0x24, 0x38, 0x64, 0x62, 0x62, 0x36, 0x31, 0x39, 0x61, 0x2d, 0x61,
        0x34, 0x31, 0x62, 0x2d, 0x34, 0x65, 0x32, 0x32, 0x2d, 0x38, 0x62, 0x64, 0x36, 0x2d, 0x65, 0x62, 0x31, 0x34, 0x32, 0x66, 0x66, 0x30, 0x65,
        0x65, 0x37, 0x65,
    ],
    Packet::OperationResponse {
        parameters: Operation::AuthenticateResponseName {
            user_id: Some("8dbb619a-a41b-4e22-8bd6-eb142ff0ee7e"),
            nickname: None,
            encryption_data: None,
            custom_data: None,
            cluster: Some("default"),
            address: "wss://GCASH013.exitgames.com:19090"
        },
        return_code: 0,
        debug_string: None,
        secret: Some(TEST_TOKEN_NAME),
    }
);

// Simulates auth request to master server
gen_test!(
    auth_request_token,
    Direction::Send,
    vec![
        0xf3, 0x2, 0xe6, 0x0, 0x1, 0xdd, 0x73, 0x1, 0x6c, 0x41, 0x75, 0x6c, 0x43, 0x64, 0x47, 0x35, 0x56, 0x6e, 0x41, 0x48, 0x5a, 0x76, 0x4e, 0x2b,
        0x73, 0x7a, 0x6c, 0x68, 0x45, 0x64, 0x51, 0x30, 0x35, 0x74, 0x6b, 0x43, 0x6a, 0x70, 0x34, 0x31, 0x78, 0x30, 0x65, 0x2f, 0x4b, 0x67, 0x78,
        0x55, 0x65, 0x7a, 0x70, 0x66, 0x45, 0x76, 0x68, 0x67, 0x47, 0x73, 0x75, 0x52, 0x35, 0x41, 0x4f, 0x45, 0x62, 0x59, 0x75, 0x75, 0x72, 0x53,
        0x6d, 0x31, 0x6f, 0x6d, 0x78, 0x53, 0x44, 0x75, 0x77, 0x66, 0x50, 0x55, 0x4e, 0x6c, 0x67, 0x48, 0x6f, 0x4f, 0x42, 0x64, 0x50, 0x58, 0x6a,
        0x32, 0x41, 0x6a, 0x36, 0x43, 0x63, 0x42, 0x37, 0x41, 0x31, 0x33, 0x43, 0x2f, 0x6f, 0x30, 0x2f, 0x6a, 0x53, 0x6e, 0x49, 0x77, 0x78, 0x66,
        0x51, 0x51, 0x69, 0x2b, 0x72, 0x4a, 0x52, 0x38, 0x50, 0x2b, 0x64, 0x48, 0x38, 0x45, 0x48, 0x74, 0x62, 0x4c, 0x45, 0x7a, 0x5a, 0x68, 0x4c,
        0x54, 0x48, 0x65, 0x2f, 0x6c, 0x57, 0x4c, 0x6e, 0x58, 0x6d, 0x45, 0x4b, 0x50, 0x6c, 0x4c, 0x33, 0x78, 0x42, 0x6c, 0x72, 0x57, 0x4d, 0x34,
        0x32, 0x2f, 0x69, 0x53, 0x71, 0x75, 0x31, 0x2f, 0x4c, 0x32, 0x4c, 0x4e, 0x2f, 0x2b, 0x32, 0x53, 0x74, 0x50, 0x4e, 0x59, 0x6e, 0x58, 0x4a,
        0x58, 0x51, 0x54, 0x47, 0x63, 0x61, 0x63, 0x4b, 0x70, 0x36, 0x70, 0x69, 0x46, 0x6d, 0x2f, 0x48, 0x62, 0x56, 0x73, 0x38, 0x45, 0x62, 0x77,
        0x51, 0x42, 0x53, 0x6b, 0x33, 0x6c, 0x4f, 0x78, 0x56, 0x2f, 0x44, 0x31, 0x78, 0x75, 0x5a, 0x50, 0x6b, 0x54, 0x72, 0x65, 0x42, 0x50, 0x51,
        0x4d, 0x74, 0x4f, 0x61, 0x32, 0x69, 0x2b, 0x5a, 0x2f, 0x38, 0x45, 0x42, 0x31, 0x45, 0x79, 0x4e, 0x4e, 0x52, 0x42, 0x6f, 0x63, 0x75, 0x34,
        0x52, 0x50, 0x7a, 0x4d, 0x79, 0x2b, 0x6a, 0x68, 0x59, 0x37, 0x47, 0x49, 0x4d, 0x33, 0x5a, 0x53, 0x68, 0x6f, 0x79, 0x69, 0x62, 0x2b, 0x6d,
        0x69, 0x4d, 0x55, 0x63, 0x6f, 0x79, 0x77, 0x56, 0x34, 0x75, 0x4f, 0x6b, 0x71, 0x4e, 0x34, 0x67, 0x4e, 0x7a, 0x30, 0x77, 0x77, 0x65, 0x56,
        0x67, 0x74, 0x6b, 0x2f, 0x74, 0x54, 0x33, 0x67, 0x5a, 0x2b, 0x34, 0x54, 0x55, 0x51, 0x75, 0x51, 0x5a, 0x74, 0x4e, 0x75, 0x33, 0x5a, 0x2b,
        0x73, 0x41, 0x62, 0x61, 0x70, 0x6c, 0x51, 0x6b, 0x72, 0x51, 0x51, 0x61, 0x51, 0x4a, 0x30, 0x75, 0x6c, 0x32, 0x57, 0x63, 0x58, 0x77, 0x2f,
        0x57, 0x64, 0x37, 0x56, 0x6f, 0x65, 0x33, 0x4d, 0x66, 0x49, 0x77, 0x2b, 0x45, 0x6e, 0x55, 0x76, 0x34, 0x6a, 0x62, 0x53, 0x69, 0x59, 0x57,
        0x32, 0x6c, 0x2b, 0x44,
    ],
    Packet::OperationRequest(Operation::AuthenticateRequestToken {
        lobby_stats: false,
        secret: TEST_TOKEN_NAME,
    })
);

// Simulates auth response from master server
gen_test!(
    auth_response_master,
    Direction::Recv,
    vec![
        0xf3, 0x3, 0xe6, 0x0, 0x0, 0x2a, 0x0, 0x2, 0xdf, 0x69, 0x0, 0x0, 0x0, 0x0, 0xdd, 0x73, 0x1, 0x2c, 0x65, 0x48, 0x57, 0x58, 0x31, 0x6c, 0x56,
        0x56, 0x32, 0x56, 0x43, 0x38, 0x77, 0x66, 0x69, 0x77, 0x5a, 0x50, 0x31, 0x31, 0x4e, 0x6b, 0x4d, 0x57, 0x38, 0x56, 0x6e, 0x6e, 0x69, 0x34,
        0x7a, 0x5a, 0x71, 0x4a, 0x66, 0x65, 0x49, 0x77, 0x68, 0x64, 0x71, 0x59, 0x46, 0x4e, 0x41, 0x69, 0x71, 0x59, 0x39, 0x4a, 0x36, 0x53, 0x68,
        0x7a, 0x2f, 0x55, 0x76, 0x42, 0x44, 0x34, 0x36, 0x6f, 0x79, 0x70, 0x41, 0x79, 0x2b, 0x55, 0x30, 0x5a, 0x74, 0x68, 0x75, 0x57, 0x4a, 0x53,
        0x6a, 0x6c, 0x77, 0x52, 0x76, 0x4a, 0x32, 0x59, 0x49, 0x32, 0x75, 0x4f, 0x53, 0x61, 0x42, 0x6a, 0x73, 0x4f, 0x50, 0x58, 0x58, 0x61, 0x48,
        0x43, 0x49, 0x56, 0x7a, 0x59, 0x4c, 0x57, 0x74, 0x6b, 0x4c, 0x6e, 0x30, 0x6f, 0x77, 0x30, 0x47, 0x43, 0x44, 0x55, 0x61, 0x4a, 0x34, 0x74,
        0x54, 0x79, 0x4e, 0x30, 0x6c, 0x42, 0x4b, 0x35, 0x73, 0x34, 0x67, 0x54, 0x51, 0x52, 0x61, 0x49, 0x43, 0x4a, 0x44, 0x65, 0x79, 0x36, 0x59,
        0x4f, 0x55, 0x31, 0x76, 0x45, 0x50, 0x52, 0x76, 0x79, 0x30, 0x44, 0x44, 0x74, 0x35, 0x7a, 0x34, 0x52, 0x6b, 0x4c, 0x63, 0x4d, 0x2b, 0x47,
        0x6b, 0x6c, 0x5a, 0x64, 0x45, 0x56, 0x7a, 0x76, 0x58, 0x4d, 0x45, 0x52, 0x77, 0x6f, 0x53, 0x71, 0x42, 0x42, 0x65, 0x41, 0x69, 0x48, 0x59,
        0x79, 0x42, 0x63, 0x78, 0x6c, 0x44, 0x71, 0x44, 0x64, 0x65, 0x4c, 0x4c, 0x59, 0x6d, 0x63, 0x52, 0x50, 0x46, 0x71, 0x34, 0x51, 0x69, 0x6e,
        0x48, 0x31, 0x76, 0x79, 0x31, 0x68, 0x38, 0x59, 0x74, 0x6a, 0x70, 0x6c, 0x70, 0x54, 0x58, 0x59, 0x7a, 0x59, 0x65, 0x36, 0x77, 0x44, 0x4d,
        0x32, 0x71, 0x56, 0x41, 0x33, 0x46, 0x2b, 0x4b, 0x4d, 0x58, 0x6c, 0x42, 0x55, 0x53, 0x4d, 0x39, 0x70, 0x5a, 0x58, 0x7a, 0x46, 0x6c, 0x49,
        0x36, 0x4f, 0x72, 0x49, 0x68, 0x55, 0x50, 0x4e, 0x6c, 0x36, 0x30, 0x55, 0x6b, 0x54, 0x6c, 0x7a, 0x43, 0x63, 0x57, 0x41, 0x73, 0x46, 0x4a,
        0x54, 0x75, 0x53, 0x76, 0x43, 0x67, 0x7a, 0x4c, 0x6e, 0x75, 0x46, 0x5a, 0x65, 0x76, 0x71, 0x38, 0x3d,
    ],
    Packet::OperationResponse {
        parameters: Operation::AuthenticateResponseMasterOrGame {
            user_id: None,
            nickname: None,
            encryption_data: None,
            custom_data: None,
            position: Some(0),
        },
        return_code: 0,
        debug_string: None,
        secret: Some(TEST_TOKEN_MASTER),
    }
);

gen_test!(
    auth_request_game,
    Direction::Send,
    vec![
        0xf3, 0x2, 0xe6, 0x0, 0x1, 0xdd, 0x73, 0x1, 0x2c, 0x65, 0x48, 0x57, 0x58, 0x31, 0x6c, 0x56, 0x56, 0x32, 0x56, 0x43, 0x38, 0x77, 0x66, 0x69,
        0x77, 0x5a, 0x50, 0x31, 0x31, 0x4e, 0x72, 0x6f, 0x58, 0x68, 0x71, 0x35, 0x64, 0x30, 0x59, 0x69, 0x79, 0x35, 0x47, 0x31, 0x30, 0x74, 0x6c,
        0x53, 0x64, 0x39, 0x30, 0x7a, 0x6f, 0x74, 0x49, 0x34, 0x30, 0x4d, 0x65, 0x68, 0x47, 0x54, 0x44, 0x37, 0x57, 0x73, 0x71, 0x50, 0x69, 0x78,
        0x70, 0x73, 0x31, 0x46, 0x61, 0x7a, 0x62, 0x65, 0x38, 0x54, 0x73, 0x48, 0x31, 0x79, 0x30, 0x55, 0x32, 0x50, 0x71, 0x69, 0x79, 0x4e, 0x79,
        0x64, 0x78, 0x43, 0x32, 0x75, 0x72, 0x46, 0x5a, 0x64, 0x78, 0x5a, 0x2b, 0x6e, 0x32, 0x68, 0x65, 0x72, 0x50, 0x44, 0x76, 0x44, 0x66, 0x71,
        0x79, 0x33, 0x73, 0x45, 0x72, 0x32, 0x6a, 0x39, 0x63, 0x62, 0x37, 0x4a, 0x61, 0x54, 0x30, 0x62, 0x46, 0x65, 0x39, 0x48, 0x69, 0x76, 0x43,
        0x6a, 0x36, 0x48, 0x53, 0x30, 0x46, 0x57, 0x35, 0x41, 0x50, 0x69, 0x2f, 0x2b, 0x69, 0x63, 0x75, 0x65, 0x51, 0x73, 0x2f, 0x54, 0x78, 0x63,
        0x6a, 0x41, 0x64, 0x66, 0x6e, 0x69, 0x4e, 0x78, 0x45, 0x62, 0x48, 0x68, 0x41, 0x45, 0x42, 0x65, 0x6f, 0x69, 0x76, 0x30, 0x69, 0x6b, 0x69,
        0x61, 0x4d, 0x63, 0x78, 0x43, 0x49, 0x4f, 0x71, 0x6d, 0x64, 0x64, 0x62, 0x43, 0x67, 0x49, 0x66, 0x75, 0x30, 0x48, 0x6b, 0x67, 0x62, 0x65,
        0x31, 0x6f, 0x79, 0x6c, 0x5a, 0x38, 0x44, 0x4c, 0x43, 0x2b, 0x78, 0x34, 0x77, 0x4a, 0x30, 0x71, 0x61, 0x76, 0x52, 0x79, 0x6a, 0x35, 0x41,
        0x74, 0x6a, 0x76, 0x73, 0x35, 0x2b, 0x32, 0x7a, 0x73, 0x54, 0x67, 0x55, 0x51, 0x31, 0x77, 0x4e, 0x63, 0x78, 0x6f, 0x33, 0x4b, 0x7a, 0x70,
        0x34, 0x2f, 0x31, 0x39, 0x67, 0x35, 0x55, 0x35, 0x4b, 0x6c, 0x6c, 0x45, 0x44, 0x58, 0x48, 0x76, 0x51, 0x57, 0x45, 0x43, 0x49, 0x6f, 0x38,
        0x37, 0x37, 0x75, 0x58, 0x63, 0x76, 0x39, 0x6d, 0x66, 0x75, 0x53, 0x41, 0x5a, 0x43, 0x74, 0x56, 0x41, 0x39, 0x43, 0x31, 0x6c, 0x41, 0x56,
        0x39, 0x59, 0x46, 0x51, 0x68, 0x34, 0x39, 0x6b, 0x3d
    ],
    Packet::OperationRequest(Operation::AuthenticateRequestToken {
        lobby_stats: false,
        secret: TEST_TOKEN_MASTER2, // I don't know either
    })
);

gen_test!(
    auth_response_game,
    Direction::Recv,
    vec![0xf3, 0x3, 0xe6, 0x0, 0x0, 0x2a, 0x0, 0x0],
    Packet::OperationResponse {
        parameters: Operation::AuthenticateResponseMasterOrGame {
            user_id: None,
            nickname: None,
            encryption_data: None,
            custom_data: None,
            position: None,
        },
        return_code: 0,
        debug_string: None,
        secret: None
    }
);
