import { response } from '../_common/axios_response';

import expenses from "$mock/entities/user_expense.json"

import type { ExpensesRequestData, ExpensesResponse } from '../models/expense_model';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 */
const mockGetExpenses = (data: ExpensesRequestData): ExpensesResponse => {
	if (!data.userId) {
		return response(400, []) as ExpensesResponse;
	}

	const userExpenses = expenses.filter((e)=> e.user_id === data.userId);
	return response(
		200,
		userExpenses
	) as ExpensesResponse;
};

export { mockGetExpenses };
