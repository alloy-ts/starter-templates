import test from 'node:test';
import assert from 'node:assert/strict';
import { main, createHash } from './main.ts';

test('main function', () => {
  assert.equal(main(), 'Hello, world!');
});

test('crypto exports from main', () => {
  const h = createHash('sha256').update('hello').digest('hex');
  assert.equal(h, '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824');
});
