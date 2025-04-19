var sortBy = function (arr, fn) {
  arr.sort((a, b) => fn(a) - fn(b));
  return arr;
};

describe("sort by", () => {
  test("case 1", () => {
    const arr = [5, 4, 1, 2, 3];
    const fn = (x) => x;
    const got = sortBy(arr, fn);
    const expected = [1, 2, 3, 4, 5];
    expect(got).toEqual(expected);
  });

  test("case 2", () => {
    const arr = [{ x: 1 }, { x: 0 }, { x: -1 }];
    const fn = (d) => d.x;
    const got = sortBy(arr, fn);
    const expected = [{ x: -1 }, { x: 0 }, { x: 1 }];
    expect(got).toEqual(expected);
  });

  test("case 3", () => {
    const arr = [
      [3, 4],
      [5, 2],
      [10, 1],
    ];
    const fn = (x) => x[1];
    const got = sortBy(arr, fn);
    const expected = [
      [10, 1],
      [5, 2],
      [3, 4],
    ];
    expect(got).toEqual(expected);
  });
});
