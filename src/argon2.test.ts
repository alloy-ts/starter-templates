import test from 'node:test';
import assert from 'node:assert/strict';
import { argon2, argon2Sync } from './main.ts';

test('crypto.argon2 and crypto.argon2Sync node:crypto contract', async () => {
  const params = {
    message: 'password',
    nonce: '0123456789abcdef',
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 2,
  };

  const derivedSync = argon2Sync('argon2id', params);
  assert.ok(Buffer.isBuffer(derivedSync));
  assert.equal(derivedSync.length, 32);

  const derivedAsync = await argon2('argon2id', params);
  assert.ok(Buffer.isBuffer(derivedAsync));
  assert.equal(derivedAsync.length, 32);
  assert.deepEqual(derivedAsync, derivedSync);
});

test('crypto.argon2 accepts secret and associatedData options', () => {
  const params = {
    message: 'password',
    nonce: '0123456789abcdef',
    parallelism: 1,
    tagLength: 64,
    memory: 4096,
    passes: 2,
    secret: 'my-pepper',
    associatedData: 'my-ad',
  };

  const derivedSync = argon2Sync('argon2id', params);
  assert.ok(Buffer.isBuffer(derivedSync));
  assert.equal(derivedSync.length, 64);
});
