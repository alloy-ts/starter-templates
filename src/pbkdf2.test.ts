import assert from 'node:assert/strict'
import test from 'node:test'
import { pbkdf2, pbkdf2Sync } from '../dist/index.js'

test('pbkdf2Sync derives key', () => {
  const derived = pbkdf2Sync('secret', 'salt', 1000, 32, 'sha256')
  assert.ok(Buffer.isBuffer(derived))
  assert.equal(derived.length, 32)
})

test('pbkdf2 async derive key', async () => {
  const derived = await pbkdf2('secret', 'salt', 1000, 64, 'sha512')
  assert.ok(Buffer.isBuffer(derived))
  assert.equal(derived.length, 64)
})
