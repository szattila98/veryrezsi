import { response } from '../_common/axios_response';

import expenses from '$mock/entities/user_expense.json';
import recurrenceTypes from '$mock/entities/recurrence_type.json';
import currencyTypes from '$mock/entities/currency_type.json';
import transactions from '$mock/entities/transaction.json';

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
				recurrenceType: recurrenceTypes.find(
					(recurrenceType) => recurrenceType.id === expense.recurrence_type_id
				),
				currencyType: currencyTypes.find(
					(currencyType) => currencyType.id === expense.currency_type_id
				),
				predefinedExpenseId: expense.predefined_expense_id,
				startDate: expense.startDate,
				value: expense.value,
				userId: expense.user_id,
				transactions: transactions
					.filter((transaction) => expense.id === transaction.expense_id)
					.map((transaction) => {
						return {
							id: transaction.id,
							donorName: transaction.donor_name,
							currencyType: currencyTypes.find(
								(currencyType) => currencyType.id === transaction.currency_type_id
							),
							value: transaction.value,
							date: transaction.date,
							expenseId: transaction.expense_id,
						};
					}),
			} as Expense;
		});
	return response(200, userExpenses) as ExpensesResponse;
};

export { mockGetExpenses };
