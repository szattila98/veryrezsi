import type { AxiosResponse, FailureResponseBody } from '$shared/axios';

export type RegisterRequestData = {
	user: string;
	password: string;
	email: string;
};

type RegisterResponseData = Record<string, never> | FailureResponseBody;

export type RegisterResponse = Omit<AxiosResponse, 'data'> & {
	data: RegisterResponseData;
};
