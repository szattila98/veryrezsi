import { response } from '../_common/axios_response';

import expenses from '$mock/entities/user_expense.json';
import recurrenceTypes from '$mock/entities/recurrence_type.json';
import currencyTypes from '$mock/entities/currency_type.json';

import type { Expense, ExpensesRequestData, ExpensesResponse } from '../models/expense_model';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 */
const mockGetExpenses = (data: ExpensesRequestData): ExpensesResponse => {
	if (!data.userId) {
		return response(400, []) as ExpensesResponse;
	}

	const userExpenses = expenses
		.filter((e) => e.user_id === data.userId)
		.map((e) => {
			return {
				id: e.id,
				name: e.name,
				description: e.description,
				recurrence_type: recurrenceTypes.find((rt) => rt.id === e.recurrence_type_id),
				currency_type: currencyTypes.find((ct) => ct.id === e.currency_type_id),
				predefined_expense_id: e.predefined_expense_id,
				startDate: e.startDate,
				value: e.value,
				user_id: e.user_id,
			} as Expense;
		});
	return response(200, userExpenses) as ExpensesResponse;
};

export { mockGetExpenses };
