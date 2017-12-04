const fs = require('fs');

function sort_unique(arr) {
  if (arr.length === 0) return arr;
  arr = arr.sort();
  var ret = [arr[0]];
  for (var i = 1; i < arr.length; i++) {
    if (arr[i-1] !== arr[i]) {
      ret.push(arr[i]);
    }
  }
  return ret;
}

function is_valid_passphrase(passphrase) {
  if (passphrase.length == 0) return false;
  let words = passphrase.split(' ')
  let unique_words = sort_unique(words);
  return words.length == unique_words.length;
}

function assert(cond) {
  if (cond) {
    console.log("✓");
  } else {
    console.log("X");
  }
}
function runTest() {
  assert(is_valid_passphrase("aa bb cc dd ee"));
  assert(!is_valid_passphrase("aa bb cc dd aa"));
  assert(is_valid_passphrase("aa bb cc dd aaa"));
}

function main() {
  const file = process.argv[2];
  const data = fs.readFileSync(file, 'utf-8');
  const lines = data.split("\n");
  const valid = lines.map(line => is_valid_passphrase(line) ? 1 : 0).reduce((sum,val) => sum + val);
  console.log("Valid passphrases: " + valid);
}

if (process.argv[2] == "--test") {
  runTest();
} else {
  main();
}
