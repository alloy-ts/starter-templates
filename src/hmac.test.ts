import { test, expect } from 'vitest'
import { createHmac, createMac } from '../index.js'

test('createHmac updates and digests', () => {
  const hmac = createHmac('sha256', 'secret-key')
  hmac.update('hello world')
  const digestHex = hmac.digest('hex')
  expect(typeof digestHex).toBe('string')
  expect(digestHex.length).toBe(64)
})

test('createMac updates and finalizes', () => {
  const mac = createMac('sha256', 'secret-key')
  mac.update('hello world')
  const tagHex = mac.final('hex')
  expect(typeof tagHex).toBe('string')
  expect(tagHex.length).toBe(64)
})
