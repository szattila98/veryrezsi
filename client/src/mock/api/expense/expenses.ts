import { response } from '../_common/axios_response';

import expensesJson from '$mock/entities/user_expense.json';
import recurrenceTypes from '$mock/entities/recurrence_type.json';
import currencyTypes from '$mock/entities/currency_type.json';
import transactionsJson from '$mock/entities/transaction.json';
import predefinedExpenses from '$mock/entities/predefined_expense.json';

import type {
	DeleteTransactionRequestData,
	DeleteTransactionResponse,
	Expense,
	GetCurrencyTypesResponse,
	GetExpensesRequestData,
	GetExpensesResponse,
	GetPredefinedExpensesResponse,
	GetRecurrenceTypesResponse,
	NewExpenseRequestData,
	NewExpenseResponse,
	NewTransactionRequestData,
	NewTransactionResponse,
	Transaction,
} from '../models/expense_model';

let expenses: Expense[] | null = null;
let transactions: Transaction[] | null = null;

export const mockGetPredefinedExpenses = (): GetPredefinedExpensesResponse => {
	const data = predefinedExpenses.map((expense) => {
		const recurrenceType = recurrenceTypes.find(
			(recurrenceType) => recurrenceType.id === expense.recurrence_type_id
		);
		const currencyType = currencyTypes.find(
			(currencyType) => currencyType.id === expense.currency_type_id
		);
		return {
			id: expense.id,
			name: expense.name,
			description: expense.description,
			recurrenceType: recurrenceType ? recurrenceType : { id: 0, name: '', perYear: 0 },
			currencyType: currencyType ? currencyType : { id: 0, abbreviation: '', name: '' },
			value: expense.value,
		};
	});

	return response(200, { predefinedExpenses: data }) as GetPredefinedExpensesResponse;
};

export const mockGetExpenses = (data: GetExpensesRequestData): GetExpensesResponse => {
	loadExpenses();

	if (!data.userId) {
		return response(400, { expenses: [] }) as GetExpensesResponse;
	}

	const userExpenses = expenses?.filter((expense) => expense.userId === data.userId);
	userExpenses?.forEach((expense) => {
		const expenseTransactions = transactions?.filter(
			(transaction) => transaction.expenseId === expense.id
		);
		if (expenseTransactions) {
			expense.transactions = expenseTransactions;
		}
	});
	return response(200, { expenses: userExpenses }) as GetExpensesResponse;
};

export const mockGetCurrencyTypes = (): GetCurrencyTypesResponse => {
	return response(200, currencyTypes) as GetCurrencyTypesResponse;
};

export const mockGetRecurrenceTypes = (): GetRecurrenceTypesResponse => {
	return response(200, recurrenceTypes) as GetRecurrenceTypesResponse;
};

export const mockNewExpense = (data: NewExpenseRequestData): NewExpenseResponse => {
	loadExpenses();
	const newExpense = data.newExpense;
	const expenseId = Math.floor(Math.random() * 100000);
	const recurrenceType = recurrenceTypes.find(
		(recurrenceType) => recurrenceType.id === newExpense.recurrenceTypeId
	);
	const currencyType = currencyTypes.find(
		(currencyType) => currencyType.id === newExpense.currencyTypeId
	);
	const expense: Expense = {
		id: expenseId,
		name: newExpense.name,
		description: newExpense.description,
		recurrenceType: recurrenceType ? recurrenceType : { id: 0, name: '', perYear: 0 },
		currencyType: currencyType ? currencyType : { id: 0, abbreviation: '', name: '' },
		predefinedExpenseId: newExpense.predefinedExpenseId,
		startDate: newExpense.startDate,
		value: newExpense.value,
		userId: newExpense.userId,
		transactions: [],
	};
	expenses?.push(expense);
	return response(200, expenseId) as NewExpenseResponse;
};

export const mockNewTransaction = (data: NewTransactionRequestData): NewTransactionResponse => {
	loadExpenses();
	if (transactions) {
		const currencyType = currencyTypes.find(
			(currencyType) => currencyType.id === data.newTransaction.currencyTypeId
		);
		const newTransaction = {
			id: Math.floor(Math.random() * 100000),
			donorName: data.newTransaction.donorName,
			currencyType: currencyType ? currencyType : { id: 0, abbreviation: '', name: '' },
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
	loadExpenses();
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

function loadExpenses() {
	loadTransactions();
	if (!expenses) {
		expenses = expensesJson.map((expense) => {
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
				transactions: [],
			} as Expense;
		});
	}
}

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
