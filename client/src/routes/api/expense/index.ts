import { mockNewExpense, mockGetPredefinedExpenses } from '$mock/api/expense/expenses';
import type { GetPredefinedExpensesResponse } from '$shared/api/getPredifinedExpenses';
import type { NewExpenseResponse, NewExpense } from '$shared/api/newExpense';
import type { RequestEvent } from '@sveltejs/kit';

/** @type {import('./[id]').RequestHandler} */
export async function POST({ request }: RequestEvent): Promise<NewExpenseResponse> {
	const body: NewExpense = await request.json();
	return mockNewExpense({ newExpense: body });
}

export async function getPredefinedExpenses(): Promise<GetPredefinedExpensesResponse> {
	return mockGetPredefinedExpenses();
}
