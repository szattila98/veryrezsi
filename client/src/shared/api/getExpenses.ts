import type { AxiosResponse } from '$shared/axios';
import type { Expense } from '$shared/domain';

// TODO not needed with actual backend
export type GetExpensesRequestData = {
	userId: number;
};

type GetExpensesResponseData = {
	expenses: Expense[];
};

export type GetExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetExpensesResponseData;
};
