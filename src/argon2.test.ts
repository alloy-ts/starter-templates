import test from 'node:test';
import assert from 'node:assert/strict';
import {
  argon2Hash,
  argon2HashSync,
  argon2HashRaw,
  argon2HashRawSync,
  argon2Verify,
  argon2VerifySync,
  argon2ParseOptions,
  argon2,
  argon2Sync,
  Algorithm,
} from './main.ts';

test('argon2 hash and verify sync', () => {
  const password = 'my-secret-password';
  const hashed = argon2HashSync(password, {
    algorithm: Algorithm.Argon2id,
    timeCost: 2,
    memoryCost: 19456,
  });

  assert.ok(typeof hashed === 'string');
  assert.ok(hashed.startsWith('$argon2id$'));

  const isValid = argon2VerifySync(hashed, password);
  assert.equal(isValid, true);

  const isInvalid = argon2VerifySync(hashed, 'wrong-password');
  assert.equal(isInvalid, false);
});

test('crypto.argon2 and crypto.argon2Sync aliases', async () => {
  const password = 'alias-password';
  const hashedSync = argon2Sync(password, {
    timeCost: 2,
    memoryCost: 4096,
  });
  assert.ok(hashedSync.startsWith('$argon2id$'));

  const hashedAsync = await argon2(password, {
    timeCost: 2,
    memoryCost: 4096,
  });
  assert.ok(hashedAsync.startsWith('$argon2id$'));
});

test('argon2 raw hash sync', () => {
  const password = Buffer.from('raw-password');
  const salt = Buffer.from('0123456789abcdef');
  const rawHash = argon2HashRawSync(password, {
    salt,
    outputLen: 32,
  });

  assert.ok(Buffer.isBuffer(rawHash));
  assert.equal(rawHash.length, 32);
});

test('argon2 parse options', () => {
  const password = 'test-password';
  const hashed = argon2HashSync(password, {
    timeCost: 3,
    memoryCost: 4096,
  });

  const parsed = argon2ParseOptions(hashed);
  assert.equal(parsed.timeCost, 3);
  assert.equal(parsed.memoryCost, 4096);
});
