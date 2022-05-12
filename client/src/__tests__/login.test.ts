/**
 * @jest-environment jsdom
 */

import '@testing-library/jest-dom';
import Login from '../routes/login.svelte';
import { render } from '@testing-library/svelte';

describe('Jest is working with Svelte', () => {
	test('Trivial text should appear in the DOM', () => {
		const { getByText } = render(Login);
		expect(getByText('Please specify valid credentials!')).toBeInTheDocument();
	});
});
