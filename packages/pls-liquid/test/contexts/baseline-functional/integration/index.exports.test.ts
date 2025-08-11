import { describe, it, expect } from 'vitest'
import {
  createLiquidMultisig,
  startSpendFromLiquidMultisig,
  finalizeTxSpendingFromLiquidMultisig,
  getUnblindedUtxoValues,
  getUnblindedUtxoValue,
  liquidSchemas,
  H
} from '../../../../index'

describe('index exports (baseline)', () => {
  it('should export expected symbols with correct types', () => {
    expect(typeof createLiquidMultisig).toBe('function')
    expect(typeof startSpendFromLiquidMultisig).toBe('function')
    expect(typeof finalizeTxSpendingFromLiquidMultisig).toBe('function')
    expect(typeof getUnblindedUtxoValues).toBe('function')
    expect(typeof getUnblindedUtxoValue).toBe('function')
    expect(typeof liquidSchemas).toBe('object')
    expect(H).toBeInstanceOf(Buffer)
  })

  it('liquidSchemas should have mainnet and testnet', () => {
    expect(liquidSchemas).toHaveProperty('mainnet')
    expect(liquidSchemas).toHaveProperty('testnet')
  })
})


