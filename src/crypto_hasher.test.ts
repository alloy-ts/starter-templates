import test from 'node:test';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import { Hash, createHash, hash, getHashes } from './main.ts';

test('createHash sha256 hex digest matches node:crypto', () => {
  const data = 'hello world';
  const expected = crypto.createHash('sha256').update(data).digest('hex');
  const actual = createHash('sha256').update(data).digest('hex');

  assert.equal(actual, expected);
});

test('Hash class sha512 base64 digest matches node:crypto', () => {
  const data = Buffer.from('test data');
  const expected = crypto.createHash('sha512').update(data).digest('base64');
  const actual = new Hash('sha512').update(data).digest('base64');

  assert.equal(actual, expected);
});

test('one-shot hash function matches node:crypto', () => {
  const data = 'one shot hash test';
  const expected = crypto.createHash('sha256').update(data).digest('hex');
  const actual = hash('sha256', data, 'hex');

  assert.equal(actual, expected);
});

test('getHashes returns supported algorithms', () => {
  const hashes = getHashes();
  assert.ok(Array.isArray(hashes));
  assert.ok(hashes.includes('sha256'));
  assert.ok(hashes.includes('sha512'));
});
