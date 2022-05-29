/**
 * @jest-environment jsdom
 */

import type { User } from '$shared/domain';

import '@testing-library/jest-dom';
import Profile from '../routes/profile.svelte';
import { render } from '@testing-library/svelte';

describe('Profile page', () => {
	test('should display Username and Email', () => {
		const testUser: User = {
			id: 11,
			email: 'mwhichera@hao123.com',
			username: 'tkrzyzanowskia',
		};

		const { getByText } = render(Profile, { user: testUser });

		expect(getByText('Username')).toBeInTheDocument();
		expect(getByText('tkrzyzanowskia')).toBeInTheDocument();

		expect(getByText('Email')).toBeInTheDocument();
		expect(getByText('mwhichera@hao123.com')).toBeInTheDocument();
	});
});
