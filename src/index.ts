function practice(numbers: number[], index: number) {
  return (numbers[index] ?? index) * 5;
}

console.log("practice([1, 2, 3], 1)", practice([1, 2, 3], 0));
console.log("practice([1, 2, 3], 10)", practice([1, 2, 3], 10));
