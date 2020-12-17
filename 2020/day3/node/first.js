const fs = require("fs");
const path = require("path");

const input = fs
  .readFileSync(path.resolve(__dirname, "../input.txt"))
  .toString();

const rows = input.split("\n");

let treeEncounters = 0;

let currentColumnIndex = 3;
for (let rowIndex = 1; rowIndex < rows.length; rowIndex++) {
  const currentRow = rows[rowIndex];
  const currentPosition = currentRow[currentColumnIndex % currentRow.length];

  if (currentPosition === "#") {
    treeEncounters++;
  }

  currentColumnIndex += 3;
}

console.log(treeEncounters);
process.exit(0);
