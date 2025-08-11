import { describe, it, expect } from 'vitest'
import { liquidSchemas } from '../../../../index'

describe('liquidSchemas baseline', () => {
  it('validates a correct mainnet object', () => {
    const data = {
      network: 'liquid',
      arbitratorsQuorum: 1,
      multisigAddress: 'ExAMPLELiquidAddress',
      privateBlindingKey: '0000000000000000000000000000000000000000000000000000000000000001',
      pubkeys: {
        clients: [
          '0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
          '02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5'
        ],
        arbitrators: [
          '02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9'
        ]
      },
      type: 'taproot-v0'
    }

    const result = liquidSchemas.mainnet.safeParse(data)
    expect(result.success).toBe(true)
  })

  it('validates a correct testnet object', () => {
    const data = {
      network: 'liquid_testnet',
      arbitratorsQuorum: 2,
      multisigAddress: 'ExAMPLELiquidTestnetAddress',
      privateBlindingKey: '0000000000000000000000000000000000000000000000000000000000000001',
      pubkeys: {
        clients: [
          '0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
          '02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5'
        ],
        arbitrators: [
          '02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9'
        ]
      },
      type: 'taproot-v0'
    }

    const result = liquidSchemas.testnet.safeParse(data)
    expect(result.success).toBe(true)
  })

  it('rejects wrong type literal', () => {
    const data = {
      network: 'liquid',
      arbitratorsQuorum: 1,
      multisigAddress: 'ExAMPLELiquidAddress',
      privateBlindingKey: '0000000000000000000000000000000000000000000000000000000000000001',
      pubkeys: {
        clients: [
          '0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
          '02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5'
        ],
        arbitrators: [
          '02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9'
        ]
      },
      type: 'taproot-v1'
    }

    const result = liquidSchemas.mainnet.safeParse(data)
    expect(result.success).toBe(false)
  })

  it('rejects invalid pubkeys structure (too few clients)', () => {
    const data = {
      network: 'liquid',
      arbitratorsQuorum: 1,
      multisigAddress: 'ExAMPLELiquidAddress',
      privateBlindingKey: '0000000000000000000000000000000000000000000000000000000000000001',
      pubkeys: {
        clients: [
          '0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'
        ],
        arbitrators: [
          '02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9'
        ]
      },
      type: 'taproot-v0'
    }

    const result = liquidSchemas.mainnet.safeParse(data)
    expect(result.success).toBe(false)
  })

  it('rejects invalid pubkeys structure (no arbitrators)', () => {
    const data = {
      network: 'liquid',
      arbitratorsQuorum: 1,
      multisigAddress: 'ExAMPLELiquidAddress',
      privateBlindingKey: '0000000000000000000000000000000000000000000000000000000000000001',
      pubkeys: {
        clients: [
          '0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798',
          '02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5'
        ],
        arbitrators: []
      },
      type: 'taproot-v0'
    }

    const result = liquidSchemas.mainnet.safeParse(data)
    expect(result.success).toBe(false)
  })

  it('rejects when required fields are missing', () => {
    const data = {
      network: 'liquid'
      // missing arbitratorsQuorum, multisigAddress, privateBlindingKey, pubkeys, type
    } as any

    const result = liquidSchemas.mainnet.safeParse(data)
    expect(result.success).toBe(false)
  })
})


