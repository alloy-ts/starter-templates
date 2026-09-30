import Sqids from "../index.js";

export const main = () => {
  const sqids = new Sqids();
  const id = sqids.encode([1, 2, 3]);
  return id;
};

console.log(main());
