import type { AxiosResponse } from '../../mock/api/_common/axios_response';
import type { MessageResponseBody } from '../../mock/api/_common/response_body';

export type RegisterRequestData = {
	user: string;
	password: string;
	email: string;
};

type RegisterResponseData = MessageResponseBody;

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type RegisterResponse = Omit<AxiosResponse, 'data'> & {
	data: RegisterResponseData;
};
