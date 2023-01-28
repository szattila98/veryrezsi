export type AxiosResponse = {
	status: number;
	statusText: string;
	data: object;
	headers: object;
	config: object;
	request: object;
};

export type FailureResponseBody = {
	reason: string;
};
