var chunk = function (arr, size) {
  let chunks = [];
  for (let i = 0; i < arr.length; i += size) {
    chunks.push(arr.slice(i, i + size));
  }
  return chunks;
};

describe("chunk", () => {
  test("case 1", () => {
    const arr = [1, 2, 3, 4, 5];
    const size = 1;
    const got = chunk(arr, size);
    const expected = [[1], [2], [3], [4], [5]];
    expect(got).toEqual(expected);
  });
  test("case 2", () => {
    const arr = [1, 9, 6, 3, 2];
    const size = 3;
    const got = chunk(arr, size);
    const expected = [
      [1, 9, 6],
      [3, 2],
    ];
    expect(got).toEqual(expected);
  });
  test("case 3", () => {
    const arr = [8, 5, 3, 2, 6];
    const size = 6;
    const got = chunk(arr, size);
    const expected = [[8, 5, 3, 2, 6]];
    expect(got).toEqual(expected);
  });
  test("case 4", () => {
    const arr = [];
    const size = 1;
    const got = chunk(arr, size);
    const expected = [];
    expect(got).toEqual(expected);
  });
});
