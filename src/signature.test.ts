import assert from 'node:assert/strict'
import test from 'node:test'
import { createSign, createVerify, generateKeyPairSync } from '../dist/index.js'

test('Sign and Verify classes', () => {
  const { publicKey, privateKey } = generateKeyPairSync('rsa', { modulusLength: 2048 })

  const signer = createSign('SHA256')
  signer.update('message to sign')
  const signature = signer.sign(privateKey, 'hex')

  assert.equal(typeof signature, 'string')

  const verifier = createVerify('SHA256')
  verifier.update('message to sign')
  const isValid = verifier.verify(publicKey, signature as string, 'hex')

  assert.equal(isValid, true)
})
