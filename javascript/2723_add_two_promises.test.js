var addTwoPromises = async function (promise1, promise2) {
  const result1 = await promise1;
  const result2 = await promise2;
  return result1 + result2;
};

describe("add two promises", () => {
  test("case 1", async () => {
    const promise1 = Promise.resolve(2);
    const promise2 = Promise.resolve(5);
    const got = await addTwoPromises(promise1, promise2);
    const expected = 7;
    expect(got).toBe(expected);
  });

  test("case 2", async () => {
    const promise1 = Promise.resolve(10);
    const promise2 = Promise.resolve(-12);
    const got = await addTwoPromises(promise1, promise2);
    const expected = -2;
    expect(got).toBe(expected);
  });
});
