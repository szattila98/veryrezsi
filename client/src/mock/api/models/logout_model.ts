import type { AxiosResponse } from '../_common/axios_response';

type LogoutHeaders = {
	cookie: string;
};

export type LogoutRequest = {
	header: LogoutHeaders;
};

type LogoutResponseData = object;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type LogoutResponse = Omit<AxiosResponse, 'data'> & {
	data: LogoutResponseData;
};
