var fibGenerator = function*() {
  let current = 0;
  let next = 1;
  while (true) {
    yield current;
    [current, next] = [next, current + next];
  }
};

function generateFibonacci(callCount) {
  const gen = fibGenerator();
  let result = [];
  for (let i = 0; i < callCount; i++) {
    result.push(gen.next().value);
  }
  return result;
}

describe("generate fibonacci sequence", () => {
  test("case 1", () => {
    const callCount = 5;
    const got = generateFibonacci(callCount);
    const expected = [0, 1, 1, 2, 3];
    expect(got).toEqual(expected);
  });
  test("case 2", () => {
    const callCount = 0;
    const got = generateFibonacci(callCount);
    const expected = [];
    expect(got).toEqual(expected);
  });
});
