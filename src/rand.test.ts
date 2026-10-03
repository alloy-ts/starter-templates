import assert from 'node:assert/strict'
import test from 'node:test'
import { getRandomValues, randomBytes, randomFillSync, randomInt, randomUuid, randomUuidv7 } from '../dist/index.js'

test('randomBytes generates requested buffer size', () => {
  const buf = randomBytes(16)
  assert.ok(Buffer.isBuffer(buf))
  assert.equal(buf.length, 16)
})

test('randomFillSync populates buffer', () => {
  const buf = new Uint8Array(10)
  randomFillSync(buf, 0, 10)
  assert.notDeepEqual(buf, new Uint8Array(10))
})

test('randomInt returns value within range', () => {
  const num = randomInt(5, 10)
  assert.ok(num >= 5 && num < 10)
})

test('randomUuid and randomUuidv7 return v4 and v7 UUID strings', () => {
  const u4 = randomUuid()
  const u7 = randomUuidv7()
  assert.equal(typeof u4, 'string')
  assert.equal(typeof u7, 'string')
  assert.equal(u4.length, 36)
  assert.equal(u7.length, 36)
})

test('getRandomValues fills typed array', () => {
  const arr = new Uint8Array(8)
  getRandomValues(arr)
  assert.equal(arr.length, 8)
})
