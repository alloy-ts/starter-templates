import { describe, expect, it } from "vite-plus/test";
import { Certificate } from "../index.js";

describe("Certificate (SPKAC)", () => {
  const sampleSpkacHex =
    "3030301a300d06092a864886f70d01010105000311000102030405060708090a0b0c0d0e0f10160a73616d706c652d636861";

  it("verifies valid/invalid spkac data via static and instance methods", () => {
    expect((Certificate as any).verifySpkac(Buffer.from(sampleSpkacHex, "hex"))).toBe(true);

    const cert = new Certificate();
    expect(cert.verifySpkac(Buffer.from(sampleSpkacHex, "hex"))).toBe(true);
    expect(cert.verifySpkac(Buffer.from("invalid-spkac-data"))).toBe(false);
  });

  it("exports challenge from spkac data", () => {
    const challengeBuf = (Certificate as any).exportChallenge(Buffer.from(sampleSpkacHex, "hex"));
    expect(challengeBuf.toString("utf8")).toBe("sample-cha");

    const cert = new Certificate();
    const instanceChallenge = cert.exportChallenge(Buffer.from(sampleSpkacHex, "hex"));
    expect(instanceChallenge.toString("utf8")).toBe("sample-cha");
  });

  it("exports public key from spkac data", () => {
    const pkBuf = (Certificate as any).exportPublicKey(Buffer.from(sampleSpkacHex, "hex"));
    expect(pkBuf.length).toBeGreaterThan(0);

    const cert = new Certificate();
    const instancePk = cert.exportPublicKey(Buffer.from(sampleSpkacHex, "hex"));
    expect(instancePk.length).toBeGreaterThan(0);
  });
});
