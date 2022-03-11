import type { ExpensesRequestData, ExpensesResponse } from '$mock/api/models/expense_model';

import { mockGetExpenses } from '$mock/api/expense/expenses';
import { RequestEvent } from '@sveltejs/kit';

export async function getExpenses(userId: number): Promise<ExpensesResponse> {
	return Promise.resolve(mockGetExpenses({userId: userId}));
}
