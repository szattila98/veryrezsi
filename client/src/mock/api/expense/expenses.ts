import { response } from '../_common/axios_response';

import expenses from '$mock/entities/user_expense.json';
import recurrenceTypes from '$mock/entities/recurrence_type.json';
import currencyTypes from '$mock/entities/currency_type.json';
import transactionsJson from '$mock/entities/transaction.json';

import type {
	DeleteTransactionRequestData,
	DeleteTransactionResponse,
	Expense,
	GetExpensesRequestData,
	GetExpensesResponse,
	Transaction,
} from '../models/expense_model';

let transactions: Transaction[] | null = null;

const mockGetExpenses = (data: GetExpensesRequestData): GetExpensesResponse => {
	loadTransactions();

	if (!data.userId) {
		return response(400, []) as GetExpensesResponse;
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
				transactions: transactions?.filter((transaction) => expense.id === transaction.expenseId),
			} as Expense;
		});
	return response(200, userExpenses) as GetExpensesResponse;
};

const mockDeleteTransaction = (data: DeleteTransactionRequestData): DeleteTransactionResponse => {
	if (transactions) {
		const toDeleteIndex = transactions.findIndex(
			(transaction) => transaction.id === data.transactionId
		);
		if (toDeleteIndex === -1) return response(404) as DeleteTransactionResponse;
		transactions.splice(toDeleteIndex, 1);
		return response(200) as DeleteTransactionResponse;
	}
	console.log('Transactions not initialized!');
	return response(500) as DeleteTransactionResponse;
};

function loadTransactions() {
	if (!transactions) {
		transactions = transactionsJson.map((transaction) => {
			return {
				id: transaction.id,
				donorName: transaction.donor_name,
				currencyType: currencyTypes.find(
					(currencyType) => currencyType.id === transaction.currency_type_id
				),
				value: transaction.value,
				date: transaction.date,
				expenseId: transaction.expense_id,
			} as Transaction;
		});
	}
}

export { mockGetExpenses, mockDeleteTransaction };
