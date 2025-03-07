var createHelloWorld = function(...args) {
	return function(...args) {
		return "Hello World";
	}
}

describe('createHelloWorld', () => {
	test('case 1: empty arguments', () => {
		const args = [];
		const got = createHelloWorld(...args)();
		const expected = "Hello World";
		expect(got).toBe(expected);
	});

	test('case 2: multiple arguments', () => {
		const args = [{}, null, 42];
		const got = createHelloWorld(...args)();
		const expected = "Hello World";
		expect(got).toBe(expected);
	});
});

