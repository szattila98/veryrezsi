import { response } from '../_common/axios_response';

import expenses from '$mock/entities/user_expense.json';
import recurrenceTypes from '$mock/entities/recurrence_type.json';
import currencyTypes from '$mock/entities/currency_type.json';
import transactionsJson from '$mock/entities/transaction.json';

import type {
	DeleteTransactionRequestData,
	DeleteTransactionResponse,
	Expense,
	GetCurrencyTypesResponse,
	GetExpensesRequestData,
	GetExpensesResponse,
	NewTransactionRequestData,
	NewTransactionResponse,
	Transaction,
} from '../models/expense_model';

let transactions: Transaction[] | null = null;

export const mockGetExpenses = (data: GetExpensesRequestData): GetExpensesResponse => {
	loadTransactions();

	if (!data.userId) {
		return response(400, { expenses: [] }) as GetExpensesResponse;
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
	return response(200, { expenses: userExpenses }) as GetExpensesResponse;
};

export const mockGetCurrencyTypes = (): GetCurrencyTypesResponse => {
	return response(200, currencyTypes) as GetCurrencyTypesResponse;
};

export const mockNewTransaction = (data: NewTransactionRequestData): NewTransactionResponse => {
	loadTransactions();
	if (transactions) {
		const newTransaction = {
			id: Math.floor(Math.random() * 100000),
			donorName: data.newTransaction.donorName,
			currencyType: currencyTypes.find(
				(currencyType) => currencyType.id === data.newTransaction.currencyTypeId
			),
			value: data.newTransaction.value,
			date: data.newTransaction.date,
			expenseId: data.newTransaction.expenseId,
		};
		transactions.push(newTransaction);
		return response(200) as NewTransactionResponse;
	}
	console.log('Transactions not initialized!');
	return response(500) as NewTransactionResponse;
};

export const mockDeleteTransaction = (
	data: DeleteTransactionRequestData
): DeleteTransactionResponse => {
	loadTransactions();
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
				value: parseInt(transaction.value),
				date: transaction.date,
				expenseId: transaction.expense_id,
			} as Transaction;
		});
	}
}
