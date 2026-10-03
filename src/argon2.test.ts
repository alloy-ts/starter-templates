import assert from 'node:assert/strict'
import test from 'node:test'
import { argon2Sync, hashSync, verifySync } from '../dist/index.js'

test('argon2Sync raw hash generation', () => {
  const hash = argon2Sync('password123', 'somesalt123')
  assert.ok(Buffer.isBuffer(hash))
  assert.ok(hash.length > 0)
})

test('argon2 hashSync and verifySync', () => {
  const hashed = hashSync('password123')
  assert.equal(typeof hashed, 'string')
  assert.ok(hashed.startsWith('$argon2'))

  const valid = verifySync(hashed, 'password123')
  assert.equal(valid, true)

  const invalid = verifySync(hashed, 'wrongpassword')
  assert.equal(invalid, false)
})
