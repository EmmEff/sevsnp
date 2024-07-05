use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod sevsnp {
    include!(concat!(env!("OUT_DIR"), "/sevsnp.rs"));
}

pub fn serialize_report(report: &sevsnp::Attestation) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(report.encoded_len());
    report.encode(&mut buf).unwrap();
    buf
}
