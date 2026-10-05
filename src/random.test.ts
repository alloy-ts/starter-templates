import { test, expect } from 'vitest'
import {
  randomBytes,
  randomFillSync,
  randomInt,
  randomUUID,
  randomUUIDv7,
  timingSafeEqual,
  getCiphers,
  getHashes,
  getCurves,
  getMacs,
  getCipherInfo,
} from '../index.js'

test('randomBytes', () => {
  const buf = randomBytes(16)
  expect(buf).toBeInstanceOf(Buffer)
  expect(buf.length).toBe(16)
})

test('randomFillSync', () => {
  const buf = Buffer.alloc(10)
  randomFillSync(buf, 0, 10)
  expect(buf).toBeInstanceOf(Buffer)
})

test('randomInt', () => {
  const num = randomInt(1, 10)
  expect(num).toBeGreaterThanOrEqual(1)
  expect(num).toBeLessThan(10)
})

test('randomUUID and randomUUIDv7', () => {
  const u4 = randomUUID()
  expect(u4.length).toBe(36)
  const u7 = randomUUIDv7()
  expect(u7.length).toBe(36)
})

test('timingSafeEqual', () => {
  const a = Buffer.from('hello')
  const b = Buffer.from('hello')
  const c = Buffer.from('world')
  expect(timingSafeEqual(a, b)).toBe(true)
  expect(timingSafeEqual(a, c)).toBe(false)
})

test('getCiphers, getHashes, getCurves, getMacs, getCipherInfo', () => {
  expect(getCiphers().length).toBeGreaterThan(0)
  expect(getHashes().length).toBeGreaterThan(0)
  expect(getCurves().length).toBeGreaterThan(0)
  expect(getMacs().length).toBeGreaterThan(0)

  const info = getCipherInfo('aes-128-gcm')
  expect(info.keyLength).toBe(16)
  expect(info.mode).toBe('gcm')
})
