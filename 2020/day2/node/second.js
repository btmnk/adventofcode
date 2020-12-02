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

  const firstPos = range.split("-")[0];
  const secondPos = range.split("-")[1];

  const char = charPart.replace(":", "");

  const firstPosIsTargetChar = password.charAt(firstPos - 1) === char;
  const secondPosIsTargetChar = password.charAt(secondPos - 1) === char;

  if (
    (firstPosIsTargetChar && !secondPosIsTargetChar) ||
    (!firstPosIsTargetChar && secondPosIsTargetChar)
  ) {
    validPasswords++;
  }
});

console.log(validPasswords);
process.exit(0);
