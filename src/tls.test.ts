import test from 'node:test';
import assert from 'node:assert/strict';
import { TLS, TlsProvider } from './main.ts';

test('TLS default provider is ring', () => {
  const tls = new TLS();
  assert.equal(tls.provider, 'ring');
  assert.equal(tls.isSupported(), true);

  const suites = tls.getCipherSuites();
  assert.ok(Array.isArray(suites));
  assert.ok(suites.length > 0);
});

test('TLS with OpenSSL provider', () => {
  const tls = new TLS(TlsProvider.OpenSSL);
  assert.ok(typeof tls.provider === 'string');
  assert.equal(tls.isSupported(), true);

  const suites = tls.getCipherSuites();
  assert.ok(Array.isArray(suites));
  assert.ok(suites.length > 0);
});

test('TLS with BTLS provider', () => {
  const tls = new TLS(TlsProvider.Btls);
  assert.equal(tls.provider, 'btls');
  assert.equal(tls.isSupported(), true);

  const suites = tls.getCipherSuites();
  assert.ok(Array.isArray(suites));
  assert.ok(suites.length > 0);
});

test('TLS with MbedTLS provider', () => {
  const tls = new TLS(TlsProvider.MbedTLS);
  assert.equal(tls.provider, 'mbedtls');
  assert.equal(tls.isSupported(), true);

  const suites = tls.getCipherSuites();
  assert.ok(Array.isArray(suites));
  assert.ok(suites.length > 0);
});
