var createCounter = function (n) {
  let c = n;
  return function () {
    return c++;
  };
};

describe("counter", () => {
  test("case 1", () => {
    let n = 10;
    let counter = createCounter(n);
    let got = [counter(), counter(), counter()];
    expected = [10, 11, 12];
    expect(got).toEqual(expected);
  });
  test("case 2", () => {
    let n = -2;
    let counter = createCounter(n);
    let got = [counter(), counter(), counter(), counter(), counter()];
    expected = [-2, -1, 0, 1, 2];
    expect(got).toEqual(expected);
  });
});
