import assert from 'node:assert/strict'
import test from 'node:test'
import { createHash, getCiphers, getCurves, getHashes, getMacs, hash, hkdfSync, timingSafeEqual } from '../dist/index.js'

test('createHash and digest', () => {
  const h = createHash('sha256')
  h.update('hello')
  const digest = h.digest('hex')
  assert.equal(digest, '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
})

test('hash helper function', () => {
  const digest = hash('sha256', 'hello', 'hex')
  assert.equal(digest, '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
})

test('getHashes, getCiphers, getCurves, getMacs', () => {
  const hashes = getHashes()
  assert.ok(hashes.includes('sha256'))

  const ciphers = getCiphers()
  assert.ok(ciphers.includes('aes-256-gcm'))

  const curves = getCurves()
  assert.ok(curves.includes('prime256v1'))

  const macs = getMacs()
  assert.ok(macs.includes('hmac'))
})

test('timingSafeEqual comparison', () => {
  const buf1 = Buffer.from('abc')
  const buf2 = Buffer.from('abc')
  const buf3 = Buffer.from('def')

  assert.equal(timingSafeEqual(buf1, buf2), true)
  assert.equal(timingSafeEqual(buf1, buf3), false)
})

test('hkdfSync key derivation', () => {
  const key = hkdfSync('sha256', 'ikm', 'salt', 'info', 32)
  assert.ok(Buffer.isBuffer(key))
  assert.equal(key.length, 32)
})
