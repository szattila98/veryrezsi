import type { RequestEvent } from '@sveltejs/kit';
import type {
	GetPredefinedExpensesResponse,
	NewExpense,
	NewExpenseResponse,
} from '$mock/api/models/expense_model';
import { mockGetPredefinedExpenses, mockNewExpense } from '$mock/api/expense/expenses';

/** @type {import('./[id]').RequestHandler} */
export async function post({ request }: RequestEvent): Promise<NewExpenseResponse> {
	const body: NewExpense = await request.json();
	return mockNewExpense({ newExpense: body });
}

export async function getPredefinedExpenses(): Promise<GetPredefinedExpensesResponse> {
	return mockGetPredefinedExpenses();
}
