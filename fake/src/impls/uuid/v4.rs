use uuid::{Builder, Variant, Version};

use crate::Dummy;

#[derive(Debug)]
pub struct UuidV4Faker;

impl Dummy<UuidV4Faker> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV4Faker, r: &mut R) -> Self {
          let bytes: [u8; 16] = r.gen();
          Builder::from_slice(&bytes).expect("generate uuid::Uuid::from_slice")
              .set_variant(Variant::RFC4122)
              .set_version(Version::Random)
              .build()
    }
    
    fn dummy(_: &UuidV4Faker) -> Self {
        uuid::Uuid::new_v4()
    }
}
