import { test, expect } from 'vitest'
import { pbkdf2Sync, pbkdf2, hkdfSync, hkdf, scryptSync, scrypt } from '../index.js'

test('pbkdf2Sync derives key', () => {
  const res = pbkdf2Sync('secret', 'salt', 1000, 32, 'sha256')
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('pbkdf2 async derives key', async () => {
  const res = await pbkdf2('secret', 'salt', 1000, 32, 'sha256')
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('hkdfSync derives key', () => {
  const res = hkdfSync('sha256', 'ikm', 'salt', 'info', 32)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('hkdf async derives key', async () => {
  const res = await hkdf('sha256', 'ikm', 'salt', 'info', 32)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('scryptSync derives key', () => {
  const res = scryptSync('password', 'salt', 32)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})

test('scrypt async derives key', async () => {
  const res = await scrypt('password', 'salt', 32)
  expect(res).toBeInstanceOf(Buffer)
  expect(res.length).toBe(32)
})
