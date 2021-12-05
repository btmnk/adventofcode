const { input } = require("../input.json");

input.forEach((first, firstIndex) => {
  input.forEach((second, secondIndex) => {
    input.forEach((third, thirdIndex) => {
      if (firstIndex === secondIndex) return;
      if (firstIndex === thirdIndex) return;
      if (secondIndex === thirdIndex) return;

      const sum = first + second + third;
      if (sum === 2020) {
        console.log(first * second * third);
        process.exit(0);
      }
    });
  });
});

console.log("Not found");
