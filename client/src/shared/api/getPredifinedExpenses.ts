import type { AxiosResponse } from '$shared/axios';
import type { PredefinedExpense } from '$shared/domain';

type GetPredefinedExpensesResponseData = PredefinedExpense[];

export type GetPredefinedExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetPredefinedExpensesResponseData;
};
