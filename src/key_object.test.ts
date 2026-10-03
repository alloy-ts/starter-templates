import assert from 'node:assert/strict'
import test from 'node:test'
import { createSecretKey, CryptoKeyPair } from '../dist/index.js'

test('KeyObject type and export', () => {
  const secretKey = createSecretKey(Buffer.from('0123456789abcdef0123456789abcdef'))
  assert.equal(secretKey.type, 'secret')
  assert.equal(secretKey.symmetricKeySize, 32)
  const exported = secretKey.export('hex')
  assert.equal(exported, Buffer.from('0123456789abcdef0123456789abcdef').toString('hex'))
})

test('CryptoKeyPair holds key objects', () => {
  const k1 = createSecretKey('key1')
  const k2 = createSecretKey('key2')
  const pair = new CryptoKeyPair(k1, k2)
  assert.equal(pair.publicKey.type, 'secret')
  assert.equal(pair.privateKey.type, 'secret')
})
