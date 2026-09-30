// @ts-ignore
import binding from "../index.js";

export const parse = binding.parse as (src: string | Buffer) => any;
export const decode = binding.decode as (src: string | Buffer) => any;
export const stringify = binding.stringify as (val: any) => string;
export const encode = binding.encode as (val: any) => string;
