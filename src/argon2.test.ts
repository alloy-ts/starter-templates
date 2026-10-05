import { test, expect } from 'vitest'
import { argon2Sync, argon2 } from '../index.js'

test('argon2Sync computes derived key', () => {
  const params = {
    message: 'password',
    nonce: Buffer.from('salt1234567890'),
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 2,
  }
  const res = argon2Sync('argon2id', params)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('argon2 async computes derived key', async () => {
  const params = {
    message: 'password',
    nonce: Buffer.from('salt1234567890'),
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 2,
  }
  const res = await argon2('argon2id', params)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})
