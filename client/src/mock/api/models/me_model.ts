import type { AxiosResponse } from '../_common/axios_response';

export type MeRequestData = {
	sessionId: string;
};

type MeResponseData = object;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type MeResponse = Omit<AxiosResponse, 'data'> & {
	data: MeResponseData;
};
