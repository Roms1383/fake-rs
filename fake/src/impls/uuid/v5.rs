use uuid::{Builder, Variant, Version};

use crate::Dummy;

#[derive(Debug)]
pub struct UuidV5Faker<'a> {
    namespace: &'a uuid::Uuid,
    name: &'a [u8],
}

impl<'a> Dummy<UuidV5Faker<'a>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV5Faker<'a>, r: &mut R) -> Self {
      Builder::from_bytes(r.gen())
          .set_variant(Variant::RFC4122)
          .set_version(Version::Sha1)
          .build()
    }
    
    fn dummy(config: &UuidV5Faker<'a>) -> Self {
        uuid::Uuid::new_v5(config.namespace, config.name)
    }
}
