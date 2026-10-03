import assert from 'node:assert/strict'
import test from 'node:test'
import { createCipheriv, createDecipheriv } from '../dist/index.js'

test('AES-256-GCM encryption and decryption', () => {
  const key = Buffer.alloc(32, 'a')
  const iv = Buffer.alloc(12, 'b')

  const cipher = createCipheriv('aes-256-gcm', key, iv)
  cipher.setAad(Buffer.from('aad-data'))
  cipher.update('plaintext message')
  const ciphertext = cipher.finalCipher('hex')
  const tag = cipher.getAuthTag()

  assert.ok(typeof ciphertext === 'string')
  assert.ok(Buffer.isBuffer(tag))

  const decipher = createDecipheriv('aes-256-gcm', key, iv)
  decipher.setAad(Buffer.from('aad-data'))
  decipher.setAuthTag(tag)
  decipher.update(Buffer.from(ciphertext as string, 'hex'))
  const plaintext = decipher.finalDecipher('utf8')

  assert.equal(plaintext, 'plaintext message')
})
