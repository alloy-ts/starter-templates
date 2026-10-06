import test from 'node:test';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import { pbkdf2, pbkdf2Sync } from './main.ts';

test('pbkdf2Sync matches node:crypto', () => {
  const password = 'password123';
  const salt = 'salt456';
  const iterations = 1000;
  const keylen = 32;
  const digest = 'sha256';

  const nodeKey = crypto.pbkdf2Sync(password, salt, iterations, keylen, digest);
  const customKey = pbkdf2Sync(password, salt, iterations, keylen, digest);

  assert.ok(Buffer.isBuffer(customKey));
  assert.equal(customKey.length, keylen);
  assert.deepEqual(customKey, nodeKey);
});

test('pbkdf2 async matches node:crypto', async () => {
  const password = 'password123';
  const salt = 'salt456';
  const iterations = 1000;
  const keylen = 64;
  const digest = 'sha512';

  const nodeKey = await new Promise<Buffer>((resolve, reject) => {
    crypto.pbkdf2(password, salt, iterations, keylen, digest, (err, derivedKey) => {
      if (err) reject(err);
      else resolve(derivedKey);
    });
  });

  const customKey = await pbkdf2(password, salt, iterations, keylen, digest);

  assert.ok(Buffer.isBuffer(customKey));
  assert.equal(customKey.length, keylen);
  assert.deepEqual(customKey, nodeKey);
});
