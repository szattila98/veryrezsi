import { mockGetRecurrenceTypes } from '$mock/api/expense/expenses';
import type { GetRecurrenceTypesResponse } from '$shared/api/getRecurrenceTypes';

export async function getRecurrenceTypes(): Promise<GetRecurrenceTypesResponse> {
	return mockGetRecurrenceTypes();
}
