/**
 * @jest-environment jsdom
 */

import '@testing-library/jest-dom';
import Login from '../routes/login.svelte';
import { render } from '@testing-library/svelte';

describe('Login page', () => {
	test('should display inform text', () => {
		const { getByText } = render(Login);
		expect(getByText('Please specify valid credentials!')).toBeInTheDocument();
	});
});
