export type ApiError = {
	reason: string;
	details: null;
};

export type ApiErrorDetail = {
	code: string;
	message: null | string;
	params: {
		value: string;
	};
};
