import { test, expect } from 'vitest'
import { TLS, TlsProvider } from '../index.js'

test('TLS class defaults to Ring provider', () => {
  const tls = new TLS()
  expect(tls.getProviderName()).toBe('ring')
  expect(tls.isSupported()).toBe(true)
})

test('TLS class with explicit provider', () => {
  const tls = new TLS(TlsProvider.OpenSSL)
  expect(tls.getProviderName()).toBe('openssl')
})
