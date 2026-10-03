import assert from 'node:assert/strict'
import test from 'node:test'
import { Tls } from '../dist/index.js'

test('TLS class provider selection', () => {
  const defaultTls = new Tls()
  assert.equal(defaultTls.provider, 'ring')
  assert.equal(defaultTls.isSupported(), true)

  const opensslTls = new Tls('openssl')
  assert.equal(opensslTls.provider, 'openssl')
  assert.equal(opensslTls.isSupported(), true)

  const boringTls = new Tls('boring')
  assert.equal(boringTls.provider, 'boringssl')
  assert.equal(boringTls.isSupported(), true)
})
