import { readFileSync } from "node:fs";

const lines = readFileSync("./lines")
  .toString("utf-8")
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4);

lines.map((line) => console.log(line));
