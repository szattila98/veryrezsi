import type { AxiosResponse } from '../_common/axios_response'; 

export type LoginRequestData = {
  user: string,
	password: string,
};

type LoginResponseData = {};

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type LoginResponse = Omit<AxiosResponse, 'data'> & {
	data: LoginResponseData
};