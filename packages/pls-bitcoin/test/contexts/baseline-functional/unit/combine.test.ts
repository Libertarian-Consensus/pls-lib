import { describe, it, expect } from 'vitest'
import { combine } from '../../../../index'

describe('combine baseline', () => {
  it('returns all 2-combinations in deterministic order', () => {
    const result = combine([1, 2, 3], 2)
    expect(result).toEqual([
      [1, 2],
      [1, 3],
      [2, 3]
    ])
  })

  it('returns all 1-combinations preserving original order', () => {
    const result = combine(['a', 'b', 'c'], 1)
    expect(result).toEqual([
      ['a'],
      ['b'],
      ['c']
    ])
  })

  it('returns empty array when size is 0', () => {
    const result = combine([1, 2, 3], 0)
    expect(result).toEqual([])
  })

  it('returns empty array when size greater than length', () => {
    const result = combine([1, 2], 3)
    expect(result).toEqual([])
  })

  it('handles duplicate items by reflecting them in output', () => {
    const result = combine([1, 1, 2], 2)
    expect(result).toEqual([
      [1, 1],
      [1, 2],
      [1, 2]
    ])
  })

  it('works with non-primitive items preserving references', () => {
    const a = { x: 1 }
    const b = { x: 2 }
    const c = { x: 3 }
    const result = combine([a, b, c], 2)
    expect(result).toEqual([
      [a, b],
      [a, c],
      [b, c]
    ])
    // ensure same references
    expect(result[0][0]).toBe(a)
    expect(result[2][1]).toBe(c)
  })

  it('does not mutate the input array', () => {
    const items = [1, 2, 3]
    const copy = items.slice()
    combine(items, 2)
    expect(items).toEqual(copy)
  })
})


