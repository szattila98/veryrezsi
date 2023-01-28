import type { AxiosResponse } from '$shared/axios';
import type { FailureResponseBody } from '$shared/axios';

type LogoutHeaders = {
	get(headerName: string): string | null;
};

export type LogoutRequest = {
	headers: LogoutHeaders;
};

type LogoutResponseData = Record<string, never> | FailureResponseBody;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type LogoutResponse = Omit<AxiosResponse, 'data'> & {
	data: LogoutResponseData;
};
