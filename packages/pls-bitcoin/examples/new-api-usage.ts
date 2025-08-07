import {
    getP2pkhAddress,
    getTaprootAddress,
    getMultisigAddress,
    getCollateralOutputScript,
    signPsbt,
    finalizePsbt,
    extractTransaction,
    GetP2pkhAddressParams,
    GetTaprootAddressParams,
    GetMultisigAddressParams,
    GetCollateralOutputScriptParams,
    SignPsbtParams,
    FinalizePsbtParams,
    ExtractTransactionParams
} from '../index';

// Example usage of the new API with parameter objects

async function exampleGetP2pkhAddress() {
    const params: GetP2pkhAddressParams = {
        publicKeyBytes: new Uint8Array([/* public key bytes */]),
        network: 'mainnet'
    };
    
    const address = await getP2pkhAddress(params);
    console.log('P2PKH Address:', address);
}

async function exampleGetTaprootAddress() {
    const params: GetTaprootAddressParams = {
        internalPublicKeyBytes: new Uint8Array([/* internal key bytes */]),
        tapTreeLeaves: null, // or array of leaves if needed
        network: 'mainnet'
    };
    
    const address = await getTaprootAddress(params);
    console.log('Taproot Address:', address);
}

async function exampleGetMultisigAddress() {
    const params: GetMultisigAddressParams = {
        m: 2,
        publicKeys: [
            new Uint8Array([/* key 1 */]),
            new Uint8Array([/* key 2 */]),
            new Uint8Array([/* key 3 */])
        ],
        network: 'mainnet',
        addressType: 'p2wsh'
    };
    
    const address = await getMultisigAddress(params);
    console.log('Multisig Address:', address);
}

async function exampleGetCollateralOutputScript() {
    const params: GetCollateralOutputScriptParams = {
        arbitratorXOnlyPubkeys: [
            new Uint8Array([/* arbitrator 1 */]),
            new Uint8Array([/* arbitrator 2 */])
        ],
        mQuorum: 2,
        userInternalXOnlyPubkeyBytes: new Uint8Array([/* user key */])
    };
    
    const script = await getCollateralOutputScript(params);
    console.log('Collateral Script:', script);
}

async function exampleSignPsbt() {
    const params: SignPsbtParams = {
        psbtBase64: 'base64_encoded_psbt_string',
        signRequests: [/* array of signature requests */]
    };
    
    const signedPsbt = await signPsbt(params);
    console.log('Signed PSBT:', signedPsbt);
}

async function exampleFinalizePsbt() {
    const params: FinalizePsbtParams = {
        psbtBase64: 'base64_encoded_psbt_string'
    };
    
    const finalizedPsbt = await finalizePsbt(params);
    console.log('Finalized PSBT:', finalizedPsbt);
}

async function exampleExtractTransaction() {
    const params: ExtractTransactionParams = {
        psbtBase64: 'base64_encoded_psbt_string'
    };
    
    const transactionHex = await extractTransaction(params);
    console.log('Transaction Hex:', transactionHex);
}

// Example usage with destructuring
async function exampleWithDestructuring() {
    // You can destructure the parameters if you want
    const { publicKeyBytes, network } = {
        publicKeyBytes: new Uint8Array([/* bytes */]),
        network: 'testnet'
    };
    
    const address = await getP2pkhAddress({ publicKeyBytes, network });
    console.log('Address:', address);
}

// Example usage with optional parameters
async function exampleWithOptionalParameters() {
    const params: GetTaprootAddressParams = {
        internalPublicKeyBytes: new Uint8Array([/* bytes */]),
        network: 'mainnet'
        // tapTreeLeaves is optional, so doesn't need to be specified
    };
    
    const address = await getTaprootAddress(params);
    console.log('Taproot Address (key path):', address);
}

export {
    exampleGetP2pkhAddress,
    exampleGetTaprootAddress,
    exampleGetMultisigAddress,
    exampleGetCollateralOutputScript,
    exampleSignPsbt,
    exampleFinalizePsbt,
    exampleExtractTransaction,
    exampleWithDestructuring,
    exampleWithOptionalParameters
};
