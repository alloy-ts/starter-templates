import assert from 'node:assert/strict'
import test from 'node:test'
import { createEcdh, Ecdh } from '../dist/index.js'

test('ECDH shared secret computation (prime256v1)', () => {
  const alice = createEcdh('prime256v1')
  const bob = createEcdh('prime256v1')

  const alicePub = alice.getPublicKey()
  const bobPub = bob.getPublicKey()

  const aliceSecret = alice.computeSecret(bobPub, null, 'hex')
  const bobSecret = bob.computeSecret(alicePub, null, 'hex')

  assert.equal(typeof aliceSecret, 'string')
  assert.equal(aliceSecret, bobSecret)
})

test('ECDH X25519 shared secret', () => {
  const alice = new Ecdh('x25519')
  const bob = new Ecdh('x25519')

  const aliceSecret = alice.computeSecret(bob.getPublicKey())
  const bobSecret = bob.computeSecret(alice.getPublicKey())

  assert.ok(Buffer.isBuffer(aliceSecret))
  assert.deepEqual(aliceSecret, bobSecret)
})
