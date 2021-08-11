use uuid::{Builder, Uuid, Variant, Version};

use crate::Dummy;

#[derive(Debug)]
pub struct UuidV3Faker<'a> {
    namespace: &'a uuid::Uuid,
    name: &'a [u8],
}

impl<'a> Dummy<UuidV3Faker<'a>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV3Faker<'a>, r: &mut R) -> Self {
        Builder::from_bytes(r.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Md5)
            .build()
    }
    
    fn dummy(config: &UuidV3Faker<'a>) -> Self {
      Uuid::new_v3(config.namespace, config.name)
    }
}
