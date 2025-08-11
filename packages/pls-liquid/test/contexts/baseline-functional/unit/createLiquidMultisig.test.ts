import { describe, it, expect } from 'vitest'
import { createLiquidMultisig, H } from '../../../../index'
import { networks } from 'liquidjs-lib'

function hexPubkey(hex: string) {
  return hex
}

describe('createLiquidMultisig baseline', () => {
  it('builds multisig with 2 parts and 1 arbitrator, quorum=1', () => {
    const parts = [
      hexPubkey('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      hexPubkey('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      hexPubkey('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const result = createLiquidMultisig(
      parts,
      arbitrators,
      1,
      networks.liquid,
      H
    )

    expect(Array.isArray(result.multisigScripts)).toBe(true)
    expect(result.multisigScripts.length).toBe(3)

    expect(typeof result.address).toBe('string')
    expect(result.address.length).toBeGreaterThan(0)
    expect(typeof result.confidentialAddress).toBe('string')
    expect(result.confidentialAddress.length).toBeGreaterThan(0)

    expect(result.hashTree).toBeDefined()
    expect(Array.isArray(result.leaves)).toBe(true)
    expect(result.leaves.length).toBe(result.multisigScripts.length)
  })

  it('when quorum > arbitrators, only the parts-only script remains', () => {
    const parts = [
      hexPubkey('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      hexPubkey('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      hexPubkey('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const { multisigScripts } = createLiquidMultisig(
      parts,
      arbitrators,
      2,
      networks.liquid,
      H
    )

    expect(multisigScripts.length).toBe(1)
  })

  it('quorum=0 yields no arbitrator combinations but keeps parts-only', () => {
    const parts = [
      hexPubkey('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      hexPubkey('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      hexPubkey('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const { multisigScripts } = createLiquidMultisig(
      parts,
      arbitrators,
      0,
      networks.liquid,
      H
    )

    expect(multisigScripts.length).toBe(1)
  })
})


