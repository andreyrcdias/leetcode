var map = function (arr, fn) {
  const result = [];
  arr.forEach(function (element, index) {
    result.push(fn(element, index));
  });
  return result;
};

describe("map", () => {
  test("case 1", () => {
    const plusOne = function (n) {
      return n + 1;
    };
    let got = map([1, 2, 3], plusOne);
    expected = [2, 3, 4];
    expect(got).toEqual(expected);
  });

  test("case 2", () => {
    const plusI = function (n, i) {
      return n + i;
    };
    let got = map([1, 2, 3], plusI);
    expected = [1, 3, 5];
    expect(got).toEqual(expected);
  });

  test("case 3", () => {
    const constant = function () {
      return 42;
    };
    let got = map([10, 20, 30], constant);
    expected = [42, 42, 42];
    expect(got).toEqual(expected);
  });
});
