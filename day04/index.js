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

function contains_anagram(words, word) {
  const chars = word.split("").sort();
  for (let w of words) {
    if (w == word) {
      continue;
    }
    const sorted = w.split("").sort();
    if (chars.join('') == sorted.join('')) {
      return true;
    }
  }

  return false;
}

function is_valid_passphrase(passphrase) {
  if (passphrase.length == 0) return false;
  let words = passphrase.split(' ');
  let unique_words = sort_unique(words);
  return words.length == unique_words.length;
}

function is_valid_anagramfree_passphrase(passphrase) {
  if (passphrase.length == 0) return false;
  if (!is_valid_passphrase(passphrase)) return false;

  let words = passphrase.split(' ');
  for (let word of words) {
    if (contains_anagram(words, word)) {
      return false;
    }
  }

  return true;
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

  console.log();
  assert(is_valid_anagramfree_passphrase("abcde fghij"));
  assert(!is_valid_anagramfree_passphrase("abcde xyz ecdab"));
  assert(is_valid_anagramfree_passphrase("iiii oiii ooii oooi oooo"));
  assert(!is_valid_anagramfree_passphrase("oiii ioii iioi iiio"));
}

function main() {
  const file = process.argv[2];
  const data = fs.readFileSync(file, 'utf-8');
  const lines = data.split("\n");
  const valid = lines.map(line => is_valid_passphrase(line) ? 1 : 0).reduce((sum,val) => sum + val);
  console.log("Valid passphrases: " + valid);
  const valid_af = lines.map(line => is_valid_anagramfree_passphrase(line) ? 1 : 0)
  .reduce((sum,val) => sum + val);
  console.log("Valid anagram-free passphrases: " + valid_af);
}

if (process.argv[2] == "--test") {
  runTest();
} else {
  main();
}
