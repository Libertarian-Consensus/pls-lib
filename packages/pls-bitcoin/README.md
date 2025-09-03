# `pls-bitcoin`

TypeScript/WebAssembly library for Bitcoin operations, ported from Rust with WASM.

## Installation

```bash
npm install pls-bitcoin
# or
yarn add pls-bitcoin
# or
pnpm add pls-bitcoin
```

## Usage

### New API (Recommended)

The library now supports a cleaner API using parameter objects:

```typescript
import {
    getP2pkhAddress,
    getTaprootAddress,
    getMultisigAddress,
    getCollateralOutputScript,
    signPsbt,
    finalizePsbt,
    extractTransaction
} from 'pls-bitcoin';

// Generate P2PKH address
const p2pkhAddress = await getP2pkhAddress({
    publicKeyBytes: new Uint8Array([/* public key bytes */]),
    network: 'mainnet'
});

// Generate Taproot address
const taprootAddress = await getTaprootAddress({
    internalPublicKeyBytes: new Uint8Array([/* internal key bytes */]),
    network: 'mainnet'
    // tapTreeLeaves is optional
});

// Generate Multisig address
const multisigAddress = await getMultisigAddress({
    m: 2,
    publicKeys: [
        new Uint8Array([/* key 1 */]),
        new Uint8Array([/* key 2 */]),
        new Uint8Array([/* key 3 */])
    ],
    network: 'mainnet',
    addressType: 'p2wsh'
});
```

### Legacy API (Deprecated)

The old functions still work but are marked as deprecated:

```typescript
// ⚠️ DEPRECATED - Use the new API with parameter objects
const address = await getP2pkhAddress(publicKeyBytes, 'mainnet');
```

## Available Functions

### getP2pkhAddress

Generates a P2PKH (Pay-to-Public-Key-Hash) address.

```typescript
interface GetP2pkhAddressParams {
    publicKeyBytes: Uint8Array;
    network: string; // 'mainnet' | 'testnet' | 'regtest'
}

const address = await getP2pkhAddress({
    publicKeyBytes: new Uint8Array([/* ... */]),
    network: 'mainnet'
});
```

### getTaprootAddress

Generates a Taproot (P2TR) address.

```typescript
interface GetTaprootAddressParams {
    internalPublicKeyBytes: Uint8Array;
    tapTreeLeaves?: any; // Optional
    network: string;
}

const address = await getTaprootAddress({
    internalPublicKeyBytes: new Uint8Array([/* ... */]),
    network: 'mainnet'
    // tapTreeLeaves is optional
});
```

### getMultisigAddress

Generates a multisig address.

```typescript
interface GetMultisigAddressParams {
    m: number; // Number of required signatures
    publicKeys: Uint8Array[];
    network: string;
    addressType: 'p2sh' | 'p2wsh';
}

const address = await getMultisigAddress({
    m: 2,
    publicKeys: [/* array of public keys */],
    network: 'mainnet',
    addressType: 'p2wsh'
});
```

### getCollateralOutputScript

Generates an output script for collateral.

```typescript
interface GetCollateralOutputScriptParams {
    arbitratorXOnlyPubkeys: Uint8Array[];
    mQuorum: number;
    userInternalXOnlyPubkeyBytes: Uint8Array;
}

const script = await getCollateralOutputScript({
    arbitratorXOnlyPubkeys: [/* array of arbitrator keys */],
    mQuorum: 2,
    userInternalXOnlyPubkeyBytes: new Uint8Array([/* ... */])
});
```

### signPsbt

Signs a PSBT (Partially Signed Bitcoin Transaction).

```typescript
interface SignPsbtParams {
    psbtBase64: string;
    signRequests: any; // Array of signature requests
}

const signedPsbt = await signPsbt({
    psbtBase64: 'base64_encoded_psbt',
    signRequests: [/* array of requests */]
});
```

### finalizePsbt

Finalizes a PSBT.

```typescript
interface FinalizePsbtParams {
    psbtBase64: string;
}

const finalizedPsbt = await finalizePsbt({
    psbtBase64: 'base64_encoded_psbt'
});
```

### extractTransaction

Extracts a transaction from a PSBT.

```typescript
interface ExtractTransactionParams {
    psbtBase64: string;
}

const transactionHex = await extractTransaction({
    psbtBase64: 'base64_encoded_psbt'
});
```

## Supported Networks

- `mainnet` - Bitcoin mainnet
- `testnet` - Bitcoin testnet
- `regtest` - Bitcoin regtest

## Multisig Address Types

- `p2sh` - Pay-to-Script-Hash (addresses starting with 3)
- `p2wsh` - Pay-to-Witness-Script-Hash (addresses starting with bc1q)

## Migration from Old API

If you're using the old API, see the [Migration Guide](./MIGRATION_GUIDE.md) for detailed instructions on how to migrate to the new API.

## Examples

See complete usage examples in [`examples/new-api-usage.ts`](./examples/new-api-usage.ts).

## Development

### Project Structure

```
pls-bitcoin/
├── wasm/                 # Rust/WASM code
│   ├── src/
│   │   ├── lib.rs       # Main functions
│   │   └── pls_bitcoin/ # Specific modules
├── index.ts             # TypeScript API
├── examples/            # Usage examples
└── MIGRATION_GUIDE.md   # Migration guide
```

### Build

```bash
# Build único (gera WASM + TS)
npm run build
# ou
pnpm run build
```

Notas:
- O comando acima executa o build do WASM com `wasm-pack --target bundler` e, em seguida, faz o bundle TypeScript (CJS/ESM/d.ts).
- O artefato `.wasm` gerado em `pkg_wasm/` é copiado para `dist/` para facilitar o consumo via bundlers.

### Testes (WASM)

```bash
# Executar testes de integração do WASM
cd wasm
wasm-pack test --node
# (opcional)
wasm-pack test --chrome
```

### Tratamento de erros (WASM → JS)

- As funções expostas pelo WASM lançam exceptions JS diretamente utilizando `unwrap_throw`/`expect_throw`/`throw_str` do `wasm-bindgen`.
- A camada consumidora (TS/JS) deve validar entradas antes de chamar a lib e capturar exceptions via `try/catch` quando necessário.

Exemplo em TypeScript:

```ts
try {
  const addr = await getP2pkhAddress({ publicKeyBytes, network: 'mainnet' });
} catch (e) {
  // e é uma exception JS lançada pelo módulo WASM
  console.error('Failed:', e);
}
```

## License

[Add license information here]