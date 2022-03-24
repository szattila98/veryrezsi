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
		.filter((expense) => expense.user_id === data.userId)
		.map((expense) => {
			return {
				id: expense.id,
				name: expense.name,
				description: expense.description,
				recurrence_type: recurrenceTypes.find((rt) => rt.id === expense.recurrence_type_id),
				currency_type: currencyTypes.find((ct) => ct.id === expense.currency_type_id),
				predefined_expense_id: expense.predefined_expense_id,
				startDate: expense.startDate,
				value: expense.value,
				user_id: expense.user_id,
			} as Expense;
		});
	return response(200, userExpenses) as ExpensesResponse;
};

export { mockGetExpenses };
