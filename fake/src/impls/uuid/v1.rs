use uuid::Uuid;

use crate::Dummy;

#[derive(Debug)]
pub struct UuidV1Faker<'a> {
    timestamp: &'a uuid::v1::Timestamp,
    node_id: &'a [u8],
}

impl<'a> Dummy<UuidV1Faker<'a>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1Faker<'a>, r: &mut R) -> Self {
        let bytes: [u8;6] = r.gen();
        let (ticks, _) = config.timestamp.to_rfc4122();
        let time_low = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
        let time_high_and_version = (((ticks >> 48) & 0x0FFF) as u16) | (1 << 12);
        Uuid::from_fields(time_low, time_mid, time_high_and_version, &bytes).expect("generate uuid::Uuid::from_fields")
    }
    
    fn dummy(config: &UuidV1Faker<'a>) -> Self {
        Uuid::new_v1(*config.timestamp, config.node_id).expect("generate uuid::Uuid::new_v1")
    }
}
