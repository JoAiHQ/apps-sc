use crate::types::{ShopInfo, ShopService};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[storage_mapper("shop_info")]
    fn shop_info(&self, slug: &ManagedBuffer) -> SingleValueMapper<ShopInfo<Self::Api>>;

    #[storage_mapper("shop_service_slugs")]
    fn shop_service_slugs(&self, shop_slug: &ManagedBuffer) -> UnorderedSetMapper<ManagedBuffer>;

    #[storage_mapper("shop_service")]
    fn shop_service(&self, shop_slug: &ManagedBuffer, service_slug: &ManagedBuffer) -> SingleValueMapper<ShopService<Self::Api>>;
}
