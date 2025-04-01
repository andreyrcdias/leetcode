var ArrayWrapper = function (nums) {
  this.nums = nums;
};

ArrayWrapper.prototype.valueOf = function () {
  return this.nums.reduce((acc, num) => acc + num, 0);
};

ArrayWrapper.prototype.toString = function () {
  return `[${this.nums.join(",")}]`;
};

describe("ArrayWrapper", () => {
  test("case 1", () => {
    const obj1 = new ArrayWrapper([1, 2]);
    const obj2 = new ArrayWrapper([3, 4]);
    expect(obj1 + obj2).toBe(10);
    expect(String(obj1)).toBe(`[1,2]`);
    expect(String(obj2)).toBe(`[3,4]`);
  });
  test("case 2", () => {
    const obj = new ArrayWrapper([[23, 98, 42, 70]]);
    expect(String(obj)).toBe(`[23,98,42,70]`);
  });
  test("case 3", () => {
    const obj1 = new ArrayWrapper([]);
    const obj2 = new ArrayWrapper([]);
    expect(obj1 + obj2).toBe(0);
  });
});
