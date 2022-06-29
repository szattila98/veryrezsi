import { mockGetCurrencyTypes } from '$mock/api/expense/expenses';
import type { GetCurrencyTypesResponse } from '$shared/api/getCurrencyTypes';

export async function getCurrencyTypes(): Promise<GetCurrencyTypesResponse> {
	return mockGetCurrencyTypes();
}
