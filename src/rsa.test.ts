import assert from 'node:assert/strict'
import test from 'node:test'
import { generateKeyPairSync, privateDecrypt, publicEncrypt } from '../dist/index.js'

test('RSA keypair generation and OAEP encrypt/decrypt', () => {
  const { publicKey, privateKey } = generateKeyPairSync('rsa', { modulusLength: 2048 })
  assert.ok(publicKey.includes('PUBLIC KEY'))
  assert.ok(privateKey.includes('PRIVATE KEY'))

  const encrypted = publicEncrypt(publicKey, Buffer.from('hello rsa'))
  assert.ok(Buffer.isBuffer(encrypted))

  const decrypted = privateDecrypt(privateKey, encrypted)
  assert.equal(decrypted.toString('utf8'), 'hello rsa')
})
