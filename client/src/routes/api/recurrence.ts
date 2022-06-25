import { mockGetRecurrenceTypes } from '$mock/api/expense/expenses';
import type { GetRecurrenceTypesResponse } from '$mock/api/models/expense_model';

export async function getCurrencyTypes(): Promise<GetRecurrenceTypesResponse> {
	return mockGetRecurrenceTypes();
}
