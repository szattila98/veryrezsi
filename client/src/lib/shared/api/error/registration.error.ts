import type { ApiError, ApiErrorDetail } from './error';

export type InvalidRegistrationPassword = Omit<ApiError, 'details'> & {
	details: {
		password: Array<ApiErrorDetail>;
	};
};
