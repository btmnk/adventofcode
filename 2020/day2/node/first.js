const fs = require("fs");
const path = require("path");

const input = fs
  .readFileSync(path.resolve(__dirname, "../input.txt"))
  .toString();

const entries = input.split(/\n/);

let validPasswords = 0;

entries.forEach((entry) => {
  const entryParts = entry.split(" ");
  const [range, charPart, password] = entryParts;

  const min = range.split("-")[0];
  const max = range.split("-")[1];

  const char = charPart.replace(":", "");

  let targetCharInPasswordCount = 0;
  for (let i = 0; i < password.length; i++) {
    if (password.charAt(i) === char) targetCharInPasswordCount++;
  }

  if (targetCharInPasswordCount <= max && targetCharInPasswordCount >= min) {
    validPasswords++;
  }
});

console.log(validPasswords);
process.exit(0);
