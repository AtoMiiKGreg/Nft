WALLET="wallet-owner.pem" # PEM path
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WASM_PATH=output/atomiik-nft.wasm


#string to hexa=0x$(xxd -pu <<< "arguments")
#integer to hexa=0x$(printf '%x\n' arguments)

# source snippets.sh && deploy

deploy() {
    local AMOUNT_OF_TOKENS=0x$(printf '%x\n' 10)
    local ROYALTIES=0x$(printf '%x\n' 5)
    local SELLING_PRICE=0x$(printf '%x\n' 1)

    erdpy --verbose contract deploy --recall-nonce --bytecode=${WASM_PATH} --pem=${WALLET} \
    --gas-limit=100000000 \
    --arguments "${AMOUNT_OF_TOKENS}" "${ROYALTIES}" "${SELLING_PRICE}" \
    --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value="${ADDRESS}"
    erdpy data store --key=deployTransaction-devnet --value="${TRANSACTION}"

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

issueToken() {
    local TOKEN_DISPLAY_NAME=AtoMiiK
    local TOKEN_TICKER=AMK

    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=100000000 --value=50000000000000000 --function="issueToken" \
    --arguments str:${TOKEN_DISPLAY_NAME} str:${TOKEN_TICKER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=100000000 --function="setLocalRoles" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createNft() {
    local TOKEN_NAME=0x526573746f0a # "Name"
    local ROYALTIES=0x$(printf '%x\n' 5)
    local URI="0x68747470733a2f2f676174657761792e70696e6174612e636c6f75642f69" # randuri.com
    local SELLING_PRICE=0x$(printf '%x\n' 1)


    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=50000000 --function="createNft" \
    --arguments ${TOKEN_NAME} "${ROYALTIES}" ${URI} "${SELLING_PRICE}"  \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

buyNft() {
    local NFT_NONCE=1

    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=10000000 --function="buyNft" \
    --arguments ${NFT_NONCE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

upgradeSC() {
      local AMOUNT_OF_TOKENS=0x$(printf '%x\n' 10)
      local ROYALTIES=0x$(printf '%x\n' 5)
      local SELLING_PRICE=0x$(printf '%x\n' 1)

    erdpy --verbose contract upgrade "${ADDRESS}" --recall-nonce \
        --bytecode=${WASM_PATH} \
        --pem=${WALLET} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments ${AMOUNT_OF_TOKENS} ${ROYALTIES} ${SELLING_PRICE} \
        --send || return
}
