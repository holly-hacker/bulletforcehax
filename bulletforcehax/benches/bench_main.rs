use bulletforcehax::photon_core::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

fn bench_photon_base(c: &mut Criterion) {
    c.bench_function("serialize OperationRequest with float value", |b| {
        b.iter(|| {
            let mut params = HashMap::new();
            params.insert(0x42, ProtocolValue::Float(13.37));
            let packet = PhotonPacket::OperationRequest(0x20, black_box(params));
            let _bytes: Vec<u8> = packet.try_into().unwrap();
        })
    });
    c.bench_function("deserialize OperationRequest with float value", |b| {
        b.iter(|| {
            let bytes = vec![0xF3, 0x02, 0x20, 0, 0x01, 0x42, 0x66, 0x41, 0x55, 0xeb, 0x85];
            let _packet = PhotonPacket::try_from(black_box(bytes.as_slice())).expect("Deserializing failed");
        })
    });
}

criterion_group!(benches, bench_photon_base);
criterion_main!(benches);
