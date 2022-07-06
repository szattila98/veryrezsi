import type { AxiosResponse } from '$mock/api/_common/axios_response';

export type DeleteTransactionRequestData = {
	transactionId: number;
};

export type DeleteTransactionResponse = Omit<AxiosResponse, 'data'>;
