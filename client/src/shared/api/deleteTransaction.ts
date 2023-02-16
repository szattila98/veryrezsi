import type { AxiosResponse } from '$shared/axios';

export type DeleteTransactionRequestData = {
	transactionId: number;
};

export type DeleteTransactionResponse = Omit<AxiosResponse, 'data'>;
