import assert from 'node:assert/strict'
import test from 'node:test'
import { createMac, decapsulate, encapsulate, X509Certificate } from '../dist/index.js'

test('Mac creation and digest', () => {
  const mac = createMac('hmac-sha256', 'secret')
  mac.update('data')
  const digest = mac.digest('hex')
  assert.equal(typeof digest, 'string')
})

test('Encapsulate and decapsulate', () => {
  const enc = encapsulate('pubkey') as { sharedKey: Buffer; ciphertext: Buffer }
  assert.ok(Buffer.isBuffer(enc.sharedKey))
  assert.ok(Buffer.isBuffer(enc.ciphertext))

  const dec = decapsulate('privkey', enc.ciphertext)
  assert.ok(Buffer.isBuffer(dec))
  assert.equal(dec.length, 32)
})

test('X509Certificate parsing', () => {
  const certPem = `-----BEGIN CERTIFICATE-----
MIICXDCCAg2gAwIBAgIJAK9...
-----END CERTIFICATE-----`
  const cert = new X509Certificate(certPem)
  assert.ok(cert.fingerprint.length > 0)
  assert.ok(cert.fingerprint256.length > 0)
})
