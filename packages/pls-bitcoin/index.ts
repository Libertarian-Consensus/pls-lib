// Importa a função de inicialização e os tipos do Wasm gerado
import init, {
    getP2pkhAddress as getP2pkhAddressWasm,
    getTaprootAddress as getTaprootAddressWasm,
    getMultisigAddress as getMultisigAddressWasm,
    getCollateralOutputScript as getCollateralOutputScriptWasm,
    signPsbt as signPsbtWasm,
    finalizePsbt as finalizePsbtWasm,
    extractTransaction as extractTransactionWasm,
    // Adicione outras funções exportadas do Rust aqui
} from './pkg_wasm/pls_bitcoin_wasm'; // Caminho para o JS gerado pelo wasm-pack

// Variável para armazenar o estado de inicialização
let wasmInitialized = false;

// Função de inicialização assíncrona para o módulo Wasm
async function initializeWasm() {
    if (!wasmInitialized) {
        await init(); // Chama a função de inicialização do Wasm
        wasmInitialized = true;
    }
}

// Reexportar as funções, agora envolvendo a inicialização do Wasm
// e adaptando as assinaturas se necessário.

export async function getP2pkhAddress(publicKeyBytes: Uint8Array, network: string): Promise<string> {
    await initializeWasm();
    return getP2pkhAddressWasm(publicKeyBytes, network);
}

export async function getTaprootAddress(
    internalPublicKeyBytes: Uint8Array,
    tapTreeLeavesJs: any, // Adapte o tipo conforme a estrutura esperada pelo Rust
    network: string
): Promise<string> {
    await initializeWasm();
    return getTaprootAddressWasm(internalPublicKeyBytes, tapTreeLeavesJs, network);
}

export async function getMultisigAddress(
    m: number,
    publicKeysJs: Uint8Array[],
    network: string,
    addressType: string
): Promise<string> {
    await initializeWasm();
    return getMultisigAddressWasm(m, publicKeysJs, network, addressType);
}

export async function getCollateralOutputScript(
    arbitratorXOnlyPubkeysJs: Uint8Array[],
    mQuorum: number,
    userInternalXOnlyPubkeyBytes: Uint8Array
): Promise<Uint8Array> {
    await initializeWasm();
    return getCollateralOutputScriptWasm(arbitratorXOnlyPubkeysJs, mQuorum, userInternalXOnlyPubkeyBytes);
}

// Lembre-se que signPsbt é um stub e precisa de implementação no Rust.
export async function signPsbt(psbtBase64: string, signRequestsJs: any): Promise<string> {
    await initializeWasm();
    // Atenção: A função Rust original retorna um Erro indicando que precisa de implementação.
    // Você precisará tratar isso ou implementar a lógica no Rust.
    try {
        return await signPsbtWasm(psbtBase64, signRequestsJs);
    } catch (error) {
        console.error("Error in signPsbtWasm:", error);
        throw error; // Re-throw ou trate o erro apropriadamente
    }
}

export async function finalizePsbt(psbtBase64: string): Promise<string> {
    await initializeWasm();
    return finalizePsbtWasm(psbtBase64);
}

export async function extractTransaction(psbtBase64: string): Promise<string> {
    await initializeWasm();
    return extractTransactionWasm(psbtBase64);
}

// Adicione wrappers para outras funções conforme necessário.

// Opcional: Exportar a função de inicialização se precisar ser chamada manualmente de fora.
export { initializeWasm };
