import { describe, it, expect } from 'vitest'
import { createBitcoinMultisig } from '../../../../index'
import { networks } from 'bitcoinjs-lib'

function ecPairLike(pubkeyHex: string) {
  return { publicKey: Buffer.from(pubkeyHex, 'hex') } as any
}

describe('createBitcoinMultisig baseline', () => {
  it('builds multisig with 2 parts and 1 arbitrator, quorum=1', () => {
    const parts = [
      ecPairLike('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      ecPairLike('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      ecPairLike('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const { multisigScripts, multisig } = createBitcoinMultisig(
      parts,
      arbitrators,
      1,
      networks.bitcoin
    )

    // combinations: [partsOnly] + [p1+arb] + [p2+arb] => 3
    expect(Array.isArray(multisigScripts)).toBe(true)
    expect(multisigScripts.length).toBe(3)
    // basic shape checks
    expect(multisigScripts[0]).toHaveProperty('leaf')
    expect(multisigScripts[0]).toHaveProperty('weight')

    // p2tr result should have output/scriptTree set
    expect(multisig.output).toBeInstanceOf(Buffer)
    expect(multisig.scriptTree).toBeDefined()
  })

  it('when quorum > arbitrators, only the parts-only script remains', () => {
    const parts = [
      ecPairLike('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      ecPairLike('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      ecPairLike('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const { multisigScripts } = createBitcoinMultisig(
      parts,
      arbitrators,
      2, // quorum > number of arbitrators
      networks.bitcoin
    )

    // With no valid arbitrator combinations, only the parts-only entry is present
    expect(multisigScripts.length).toBe(1)
  })

  it('quorum=0 yields no arbitrator combinations but keeps parts-only', () => {
    const parts = [
      ecPairLike('0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798'),
      ecPairLike('02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5')
    ]
    const arbitrators = [
      ecPairLike('02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9')
    ]

    const { multisigScripts } = createBitcoinMultisig(
      parts,
      arbitrators,
      0,
      networks.bitcoin
    )

    expect(multisigScripts.length).toBe(1)
  })
})


