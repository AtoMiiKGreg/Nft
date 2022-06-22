#![no_std]

const ROYALTIES_MAX: u32 = 10_000;

use core::convert::TryInto;
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod nft_module;
mod storage;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct ExampleAttributes {
    pub creation_timestamp: u64,
}

#[elrond_wasm::contract]
pub trait NftMinter: nft_module::NftModule + storage::StorageModule {
    #[init]
    fn init(&self,
    amount_of_tokens:u32,
    royalties:BigUint<Self::Api>,
    selling_price:BigUint<Self::Api>) {

        require!(royalties <= ROYALTIES_MAX,"royalties cannot exceed 100%!");
        require!(amount_of_tokens >= 1, "amount of tokens to mint should be at least 1!");

        self.amount_of_tokens_total().set_if_empty(&amount_of_tokens);
        self.royalties().set_if_empty(&royalties);
        self.selling_price().set_if_empty(&selling_price)
    }


    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer<Self::Api>,
        royalties: BigUint<Self::Api>,
        uri: ManagedBuffer<Self::Api>,
        selling_price: BigUint<Self::Api>,
        opt_token_used_as_payment: OptionalValue<TokenIdentifier<Self::Api>>,
        opt_token_used_as_payment_nonce: OptionalValue<u64>,
    ) {
        let token_used_as_payment = match opt_token_used_as_payment {
            OptionalValue::Some(token) => EgldOrEsdtTokenIdentifier::esdt(token),
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };
        require!(
            token_used_as_payment.is_valid_esdt_identifier(),
            "Invalid token_used_as_payment arg, not a valid token ID"
        );

        let token_used_as_payment_nonce = if token_used_as_payment.is_egld() {
            0
        } else {
            match opt_token_used_as_payment_nonce {
                OptionalValue::Some(nonce) => nonce,
                OptionalValue::None => 0,
            }
        };

        let attributes = ExampleAttributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        self.create_nft_with_attributes(
            name,
            royalties,
            attributes,
            uri,
            selling_price,
            token_used_as_payment,
            token_used_as_payment_nonce,
        );
    }

    // The marketplace SC will send the funds directly to the initial caller, i.e. the owner
    // The caller has to know which tokens they have to claim,
    // by giving the correct token ID and token nonce
    #[only_owner]
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress<Self::Api>,
        token_id: TokenIdentifier<Self::Api>,
        token_nonce: u64,
    ) {
        let caller = self.blockchain().get_caller();
        self.marketplace_proxy(marketplace_address)
            .claim_tokens(token_id, token_nonce, caller)
            .async_call()
            .call_and_exit()
    }

    #[proxy]
    fn marketplace_proxy(
        &self,
        sc_address: ManagedAddress<Self::Api>,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;


}

mod nft_marketplace_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait NftMarketplace {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            token_id: TokenIdentifier<Self::Api>,
            token_nonce: u64,
            claim_destination: ManagedAddress<Self::Api>,
        );
    }




}

