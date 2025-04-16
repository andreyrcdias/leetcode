async function sleep(millis) {
  return new Promise((r) => setTimeout(r, millis));
}

describe("sleep", () => {
  test("case 1", async () => {
    let millis = 100;
    const start = Date.now();
    await sleep(millis);
    const duration = Date.now() - start;
    expect(duration).toBeGreaterThanOrEqual(millis);
  });

  test("case 2", async () => {
    let millis = 200;
    const start = Date.now();
    await sleep(millis);
    const duration = Date.now() - start;
    expect(duration).toBeGreaterThanOrEqual(millis);
  });
});
