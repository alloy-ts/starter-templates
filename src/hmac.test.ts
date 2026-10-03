import assert from 'node:assert/strict'
import test from 'node:test'
import { createHmac, Hmac } from '../dist/index.js'

test('HMAC sha256 digest hex', () => {
  const hmac = createHmac('sha256', 'secret-key')
  hmac.update('hello world')
  const digest = hmac.digest('hex')
  assert.equal(typeof digest, 'string')
  assert.equal((digest as string).length, 64)
})

test('HMAC class instantiation', () => {
  const hmac = new Hmac('sha512', 'key')
  hmac.update('test data')
  const digest = hmac.digest()
  assert.ok(Buffer.isBuffer(digest))
  assert.equal((digest as Buffer).length, 64)
})
