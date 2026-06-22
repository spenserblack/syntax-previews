#!/usr/bin/env node
// Yes, JavaScript supports shebangs
/**
 *
 * @param {number} a
 * @param {number} b
 */
async function main(a, b) {
  console.log(`${a} + ${b} =`, a + b);
}

const obj = {
  a: 1,
  ["b"]: true,
};

main(1, 2);
