const fs = require("fs");
const path = require("path");

const input = fs
  .readFileSync(path.resolve(__dirname, "../input.txt"))
  .toString();

const rows = input.split("\n");

const slopes = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
];

let slopeProduct;

slopes.forEach((slope) => {
  const [right, down] = slope;
  let treeEncounters = 0;

  let currentColumnIndex = right;
  for (let rowIndex = down; rowIndex < rows.length; rowIndex += down) {
    const currentRow = rows[rowIndex];
    const currentPosition = currentRow[currentColumnIndex % currentRow.length];

    if (currentPosition === "#") {
      treeEncounters++;
    }

    currentColumnIndex += right;
  }

  if (!slopeProduct) {
    slopeProduct = treeEncounters;
  } else {
    slopeProduct *= treeEncounters;
  }

  console.log(`Slope: [${right}, ${down}]:`, treeEncounters);
});

console.log("SlopeProduct", slopeProduct);

process.exit(0);
