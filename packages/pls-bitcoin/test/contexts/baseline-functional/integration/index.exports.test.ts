import { describe, it, expect } from 'vitest'
import {
  combine,
  createBitcoinMultisig,
  startTxSpendingFromMultisig,
  bitcoinSchemas,
  H
} from '../../../../index'

describe('index exports (baseline)', () => {
  it('should export expected symbols with correct types', () => {
    expect(typeof combine).toBe('function')
    expect(typeof createBitcoinMultisig).toBe('function')
    expect(typeof startTxSpendingFromMultisig).toBe('function')
    expect(typeof bitcoinSchemas).toBe('object')
    expect(H).toBeInstanceOf(Buffer)
  })

  it('bitcoinSchemas should have mainnet and testnet', () => {
    expect(bitcoinSchemas).toHaveProperty('mainnet')
    expect(bitcoinSchemas).toHaveProperty('testnet')
  })
})


