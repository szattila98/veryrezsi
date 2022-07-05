import type { AxiosResponse } from '$mock/api/_common/axios_response';
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
