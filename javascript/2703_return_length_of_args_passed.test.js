var argumentsLength = function(...args) {
	return args.length;
}

describe('argumentsLength', () => {
	test('case 1: single argument', () => {
		const args = [5];
		const got = argumentsLength(...args);
		const expected = 1;
		expect(got).toBe(expected);
	});

	test('case 2: multiple arguments', () => {
		const args = [{}, null, "3"];
		const got = argumentsLength(...args);
		const expected = 3;
		expect(got).toBe(expected);
	});
});

