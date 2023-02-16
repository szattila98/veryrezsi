import type { AxiosResponse, FailureResponseBody } from '$shared/axios';

type LogoutResponseData = Record<string, never> | FailureResponseBody;
export type LogoutResponse = Omit<AxiosResponse, 'data'> & {
	data: LogoutResponseData;
};
