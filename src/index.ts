import { readFileSync } from "node:fs";

const file = process.argv[2] ?? "";

try {
  const content = readFileSync(file).toString("utf-8");

  content.split("\n").map((value) => {
    const print = parseInt(value);

    if (isNaN(print)) {
      console.log("Line not a number");
    } else {
      console.log(print);
    }
  });
} catch (err) {
  console.log("Ooops, the file does not exist!");
}
