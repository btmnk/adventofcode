const { input } = require("./task1input.json");

input.forEach((left, leftIndex) => {
  input.forEach((right, rightIndex) => {
    if (leftIndex === rightIndex) return;
    const sum = left + right;
    if (sum === 2020) {
      console.log(left * right);
      process.exit(0);
    }
  });
});

console.log("Not found");
