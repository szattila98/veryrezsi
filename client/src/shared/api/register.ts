import type { AxiosResponse } from '$shared/axios';
import type { FailureResponseBody } from '$shared/axios';

export type RegisterRequestData = {
	user: string;
	password: string;
	email: string;
};

type RegisterResponseData = Record<string, never> | FailureResponseBody;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type RegisterResponse = Omit<AxiosResponse, 'data'> & {
	data: RegisterResponseData;
};
