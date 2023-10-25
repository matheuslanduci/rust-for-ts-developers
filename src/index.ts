function multiply(number: number | undefined): number | undefined {
  if (number === undefined) {
    return undefined;
  }

  return number * 5;
}

console.log("By 0: ", multiply(0));
console.log("By 5: ", multiply(5));
console.log("By undefined: ", multiply(undefined));
