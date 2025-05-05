var isEmpty = function (obj) {
  return Object.keys(obj).length == 0;
};

describe("is object empty", () => {
  test("case 1", () => {
    const obj = { x: 5, y: 42 };
    const got = isEmpty(obj);
    const expected = false;
    expect(got).toBe(expected);
  });

  test("case 2", () => {
    const obj = {};
    const got = isEmpty(obj);
    const expected = true;
    expect(got).toBe(expected);
  });

  test("case 3", () => {
    const obj = [null, false, 0];
    const got = isEmpty(obj);
    const expected = false;
    expect(got).toBe(expected);
  });
});
