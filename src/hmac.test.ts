import test from 'node:test';
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import { Hmac, createHmac } from './main.ts';

test('Hmac create and digest hex', () => {
  const secret = 'secret-key';
  const data = 'hello world';

  const nodeHmac = crypto.createHmac('sha256', secret).update(data).digest('hex');
  const customHmac = createHmac('sha256', secret).update(data).digest('hex');

  assert.equal(customHmac, nodeHmac);
});

test('Hmac base64 digest', () => {
  const secret = 'secret-key';
  const data = 'hello world';

  const nodeHmac = crypto.createHmac('sha256', secret).update(data).digest('base64');
  const customHmac = createHmac('sha256', secret).update(data).digest('base64');

  assert.equal(customHmac, nodeHmac);
});

test('Hmac buffer digest', () => {
  const secret = 'secret-key';
  const data = Buffer.from('hello world');

  const nodeHmac = crypto.createHmac('sha512', secret).update(data).digest();
  const customHmac = new Hmac('sha512', secret).update(data).digest();

  assert.ok(Buffer.isBuffer(customHmac));
  assert.deepEqual(customHmac, nodeHmac);
});
