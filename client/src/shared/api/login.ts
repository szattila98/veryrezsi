import type { AxiosResponse, FailureResponseBody } from '$shared/axios';

export type LoginRequestData = {
	email: string;
	password: string;
};

type LoginResponseData = Record<string, never> | FailureResponseBody;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type LoginResponse = Omit<AxiosResponse, 'data'> & {
	data: LoginResponseData;
};
