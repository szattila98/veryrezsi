import { mockGetCurrencyTypes } from "$mock/api/expense/expenses";
import type { GetCurrencyTypesResponse } from "$mock/api/models/expense_model";

export async function getCurrencyTypes(): Promise<GetCurrencyTypesResponse> {
	return mockGetCurrencyTypes();
}