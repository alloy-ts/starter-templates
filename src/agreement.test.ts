import { test, expect } from 'vitest'
import {
  createECDH,
  createDiffieHellman,
  createSecretKey,
  createPublicKey,
  createPrivateKey,
  createSign,
  createVerify,
  X509Certificate,
} from '../index.js'

test('ECDH computes shared secret', () => {
  const alice = createECDH('x25519')
  const aliceKey = alice.generateKeys()

  const bob = createECDH('x25519')
  const bobKey = bob.generateKeys()

  const secret1 = alice.computeSecret(bobKey)
  const secret2 = bob.computeSecret(aliceKey)

  expect(secret1.toString('hex')).toBe(secret2.toString('hex'))
})

test('KeyObject creation and export', () => {
  const secretKey = createSecretKey(Buffer.alloc(16, 1))
  expect(secretKey.type).toBe('secret')
  expect(secretKey.symmetricKeySize).toBe(16)

  const pubKey = createPublicKey(Buffer.from('pub'))
  expect(pubKey.type).toBe('public')

  const privKey = createPrivateKey(Buffer.from('priv'))
  expect(privKey.type).toBe('private')
})

test('Sign and Verify', () => {
  const sign = createSign('sha256')
  sign.update('message')
  const sig = sign.sign(Buffer.from('privkey'))

  const verify = createVerify('sha256')
  verify.update('message')
  const isValid = verify.verify(Buffer.from('pubkey'), sig)

  expect(isValid).toBe(true)
})

test('X509Certificate', () => {
  const cert = new X509Certificate(Buffer.from('CERT DATA'))
  expect(cert.subject).toBe('CN=Test Subject')
  expect(cert.fingerprint256).toBeDefined()
})
