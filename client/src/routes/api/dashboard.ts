import type { ExpensesRequestData, ExpensesResponse } from '$mock/api/models/expense_model';

import { mockGetExpenses } from '$mock/api/expense/expenses';

export async function getExpenses(data: ExpensesRequestData): Promise<ExpensesResponse> {
	return Promise.resolve(mockGetExpenses({ userId: data.userId }));
}
