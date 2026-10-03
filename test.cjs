const test = require('node:test')
const assert = require('node:assert/strict')
const crypto = require('./index.js')

test('crypto native exports', () => {
  assert.equal(typeof crypto.createHash, 'function')
  assert.equal(typeof crypto.createHmac, 'function')
  assert.equal(typeof crypto.pbkdf2Sync, 'function')
  assert.equal(typeof crypto.argon2HashSync, 'function')
  assert.equal(typeof crypto.Tls, 'function')

  const hash = crypto.createHash('sha256').update('hello').digest('hex')
  assert.equal(hash, '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
})
