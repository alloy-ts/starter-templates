import { test, expect } from 'vitest'
import { createCipheriv, createDecipheriv } from '../index.js'

test('Cipheriv and Decipheriv aes-128-gcm roundtrip', () => {
  const key = Buffer.alloc(16, 'k')
  const iv = Buffer.alloc(12, 'i')

  const cipher = createCipheriv('aes-128-gcm', key, iv)
  cipher.setAad(Buffer.from('aad-data'))
  cipher.update('hello aead')
  const ct = cipher.final()
  const tag = cipher.getAuthTag()

  expect(tag).toBeInstanceOf(Buffer)

  const decipher = createDecipheriv('aes-128-gcm', key, iv)
  decipher.setAad(Buffer.from('aad-data'))
  decipher.setAuthTag(tag)
  decipher.update(ct)
  const pt = decipher.final('utf8')

  expect(pt).toBe('hello aead')
})
