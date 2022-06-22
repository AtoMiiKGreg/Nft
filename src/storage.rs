use elrond_wasm::api::StorageMapperApi;
use crate::nft_module::PriceTag;
elrond_wasm:: imports!();
elrond_wasm:: derive_imports!();


#[elrond_wasm::module]
pub trait StorageModule {

    // storage

    #[storage_mapper("amountOfTokensTotal")]
    fn amount_of_tokens_total(&self) -> SingleValueMapper<Self::Api,u32>;

    #[storage_mapper("royalties")]
    fn royalties(&self)-> SingleValueMapper<Self::Api,BigUint<Self::Api>>;

    #[storage_mapper("sellingPrice")]
    fn selling_price(&self)-> SingleValueMapper<Self::Api,BigUint<Self::Api>>;

    #[storage_mapper("nftTokenId")]
    fn nft_token_id(&self) -> SingleValueMapper<Self::Api,TokenIdentifier<Self::Api>>;

    #[storage_mapper("priceTag")]
    fn price_tag(&self, nft_nonce: u64) -> SingleValueMapper<Self::Api,PriceTag<Self::Api>>;

    #[storage_mapper("tokensLeftToMint")]
    fn tokens_left_to_mint(&self) -> UniqueIdMapper<Self::Api>;



}
    //Append function random to uniqueIdMapper

pub trait Random<SA>
where
    SA : StorageMapperApi,{

    fn swap_remove_random(&mut self) -> usize;
}

impl<SA> Random<SA> for UniqueIdMapper<SA>
    where
        SA : StorageMapperApi,
{
    fn swap_remove_random(&mut self) -> usize {
        let mut rand_source=RandomnessSource::<SA>::new();
        let random_id = rand_source.next_usize_in_range(1,self.len()+1);
        self.swap_remove(random_id)
    }

}