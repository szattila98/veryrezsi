import type { AxiosResponse } from '$shared/axios';

export type NewTransaction = {
	donorName: string;
	currencyTypeId: number;
	value: number;
	date: string;
	expenseId: number;
};

export type NewTransactionRequestData = {
	newTransaction: NewTransaction;
};

export type NewTransactionResponse = Omit<AxiosResponse, 'data'>;
