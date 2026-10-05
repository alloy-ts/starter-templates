import { test, expect } from 'vitest'
import { createHash, hash } from '../index.js'

test('createHash sha256', () => {
  const h = createHash('sha256')
  h.update('hello')
  const digest = h.digest('hex')
  expect(digest).toBe('2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
})

test('one-shot hash', () => {
  const digest = hash('sha256', 'hello', 'hex')
  expect(digest).toBe('2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
})
