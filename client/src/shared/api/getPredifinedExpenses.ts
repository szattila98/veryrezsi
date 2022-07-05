import type { AxiosResponse } from '$mock/api/_common/axios_response';
import type { PredefinedExpense } from '$shared/domain';

type GetPredefinedExpensesResponseData = PredefinedExpense[];

export type GetPredefinedExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetPredefinedExpensesResponseData;
};
