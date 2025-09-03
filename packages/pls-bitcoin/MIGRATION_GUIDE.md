# Migration Guide - New API with Parameter Objects

This guide shows how to migrate from the old code (with multiple positional parameters) to the new API (with parameter objects).

## Why change?

The new API offers several advantages:
- **Readability**: Named parameters make the code clearer
- **Maintainability**: Easier to add new parameters without breaking existing code
- **Flexibility**: Optional parameters are more natural
- **Type Safety**: Better type inference in TypeScript

## Migration Examples

### 1. getP2pkhAddress

**Before:**
```typescript
const address = await getP2pkhAddress(publicKeyBytes, 'mainnet');
```

**After:**
```typescript
const address = await getP2pkhAddress({
    publicKeyBytes,
    network: 'mainnet'
});
```

### 2. getTaprootAddress

**Before:**
```typescript
const address = await getTaprootAddress(
    internalPublicKeyBytes,
    tapTreeLeaves,
    'mainnet'
);
```

**After:**
```typescript
const address = await getTaprootAddress({
    internalPublicKeyBytes,
    tapTreeLeaves, // optional
    network: 'mainnet'
});
```

### 3. getMultisigAddress

**Before:**
```typescript
const address = await getMultisigAddress(
    2,
    publicKeys,
    'mainnet',
    'p2wsh'
);
```

**After:**
```typescript
const address = await getMultisigAddress({
    m: 2,
    publicKeys,
    network: 'mainnet',
    addressType: 'p2wsh'
});
```

### 4. getCollateralOutputScript

**Before:**
```typescript
const script = await getCollateralOutputScript(
    arbitratorPubkeys,
    2,
    userPubkey
);
```

**After:**
```typescript
const script = await getCollateralOutputScript({
    arbitratorXOnlyPubkeys: arbitratorPubkeys,
    mQuorum: 2,
    userInternalXOnlyPubkeyBytes: userPubkey
});
```

### 5. signPsbt

**Before:**
```typescript
const signedPsbt = await signPsbt(psbtBase64, signRequests);
```

**After:**
```typescript
const signedPsbt = await signPsbt({
    psbtBase64,
    signRequests
});
```

### 6. finalizePsbt

**Before:**
```typescript
const finalizedPsbt = await finalizePsbt(psbtBase64);
```

**After:**
```typescript
const finalizedPsbt = await finalizePsbt({
    psbtBase64
});
```

### 7. extractTransaction

**Before:**
```typescript
const txHex = await extractTransaction(psbtBase64);
```

**After:**
```typescript
const txHex = await extractTransaction({
    psbtBase64
});
```

## Migration Strategy

### Phase 1: Compatibility (Current)
- Old functions still work (marked as `@deprecated`)
- You can migrate gradually
- No breaking changes to existing code

### Error Handling Strategy (WASM)

- A camada WASM lança exceptions via `unwrap_throw`/`expect_throw`/`throw_str`.
- Valide entradas no TS/JS antes de chamar a lib. Use `try/catch` para capturar exceptions quando necessário.
- Essa abordagem simplifica a fronteira Rust↔JS e reduz boilerplate no código Rust.

### Phase 2: Gradual Migration
- Use the new API in new code
- Migrate existing code during maintenance
- Keep legacy functions during transition period

### Phase 3: Legacy Function Removal (Future)
- After an adequate period, legacy functions can be removed
- This will be announced in advance

## Benefits of the New API

### 1. Optional Parameters
```typescript
// Before: had to pass null explicitly
getTaprootAddress(pubkey, null, 'mainnet');

// After: can omit optional parameters
getTaprootAddress({
    internalPublicKeyBytes: pubkey,
    network: 'mainnet'
    // tapTreeLeaves is optional, doesn't need to be specified
});
```

### 2. Better Readability
```typescript
// Before: hard to know what each parameter means
getMultisigAddress(2, keys, 'mainnet', 'p2wsh');

// After: clear what each value represents
getMultisigAddress({
    m: 2,
    publicKeys: keys,
    network: 'mainnet',
    addressType: 'p2wsh'
});
```

### 3. Ease of Adding New Parameters
```typescript
// If you need to add a new optional parameter in the future:
getP2pkhAddress({
    publicKeyBytes,
    network: 'mainnet',
    // newParameter: value // doesn't break existing code
});
```

## TypeScript Types

The new API includes well-defined types:

```typescript
import {
    GetP2pkhAddressParams,
    GetTaprootAddressParams,
    GetMultisigAddressParams,
    // ... other types
} from 'pls-bitcoin';

// You can use these types for validation and autocomplete
const params: GetP2pkhAddressParams = {
    publicKeyBytes: new Uint8Array([/* ... */]),
    network: 'mainnet'
};
```

## Support

If you encounter problems during migration:
1. Check the examples in `examples/new-api-usage.ts`
2. Legacy functions still work during transition
3. Consult the type documentation to understand the expected structure
