const assert = require("node:assert/strict");
const test = require("node:test");
const crypto = require("node:crypto");

const {
  init,
  version,
  base64Encode,
  base64Decode,
  digest,
  hmac,
  encryptData,
  decryptData,
  BoringSshCodec,
  BtlsCodec,
} = require("./index.js");

test("init and version", () => {
  init();
  const ver = version();
  assert.equal(typeof ver, "string");
  assert.ok(ver.length > 0, "Version string should non-empty");
});

test("base64 encoding and decoding", () => {
  const inputBuf = Buffer.from("Hello BoringSSL / btls!");
  const encoded = base64Encode(inputBuf);
  assert.equal(typeof encoded, "string");

  const decoded = base64Decode(encoded);
  assert.equal(decoded.toString("utf8"), "Hello BoringSSL / btls!");
});

test("digest computation", () => {
  const data = Buffer.from("BoringSSL digest test");
  const hashVal = digest("sha256", data);
  const expectedHash = crypto.createHash("sha256").update(data).digest();
  assert.deepEqual(hashVal, expectedHash);
});

test("HMAC computation", () => {
  const key = Buffer.from("secret-key");
  const data = Buffer.from("BoringSSL hmac test");
  const hmacVal = hmac("sha256", key, data);
  const expectedHmac = crypto.createHmac("sha256", key).update(data).digest();
  assert.deepEqual(hmacVal, expectedHmac);
});

test("encryptData and decryptData with aes-256-cbc", () => {
  const key = crypto.randomBytes(32);
  const iv = crypto.randomBytes(16);
  const plaintext = Buffer.from("Secret payload for symmetric cipher encryption test");

  const encrypted = encryptData("aes-256-cbc", key, iv, plaintext);
  assert.ok(encrypted.length > 0);
  assert.notDeepEqual(encrypted, plaintext);

  const decrypted = decryptData("aes-256-cbc", key, iv, encrypted);
  assert.equal(decrypted.toString("utf8"), plaintext.toString("utf8"));
});

test("BoringSshCodec encode and decode", () => {
  const key = crypto.randomBytes(32);
  const iv = crypto.randomBytes(16);
  const codec = new BoringSshCodec("aes-256-cbc", key, iv);

  const payload = Buffer.from("BoringSSH raw payload message");
  const encoded = codec.encode(payload);
  assert.equal(codec.sequenceNumber(), 1);

  const codecDec = new BoringSshCodec("aes-256-cbc", key, iv);
  const decoded = codecDec.decode(encoded);
  assert.equal(decoded.toString("utf8"), payload.toString("utf8"));
});

test("BoringSshCodec packet framing encodePacket and decodePacket", () => {
  const key = crypto.randomBytes(32);
  const iv = crypto.randomBytes(16);
  const encoder = new BoringSshCodec("aes-256-cbc", key, iv);
  const decoder = new BoringSshCodec("aes-256-cbc", key, iv);

  const sshPayload = Buffer.from("SSH_MSG_CHANNEL_DATA payload content");
  const encryptedPacket = encoder.encodePacket(sshPayload);
  assert.equal(encoder.sequenceNumber(), 1);

  const decodedPayload = decoder.decodePacket(encryptedPacket);
  assert.equal(decoder.sequenceNumber(), 1);
  assert.equal(decodedPayload.toString("utf8"), sshPayload.toString("utf8"));
});

test("BtlsCodec packet framing and sequence numbers", () => {
  const key = crypto.randomBytes(16);
  const iv = crypto.randomBytes(16);
  const encoder = new BtlsCodec("aes-128-ctr", key, iv);
  const decoder = new BtlsCodec("aes-128-ctr", key, iv);

  assert.equal(encoder.sequenceNumber(), 0);
  encoder.incrementSequenceNumber();
  assert.equal(encoder.sequenceNumber(), 1);

  const msg = Buffer.from("BtlsCodec packet framing test message");
  const packet = encoder.encodePacket(msg);
  const unpacked = decoder.decodePacket(packet);
  assert.equal(unpacked.toString("utf8"), msg.toString("utf8"));
});
