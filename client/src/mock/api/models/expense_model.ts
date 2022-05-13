import type { AxiosResponse } from '../_common/axios_response';

export interface RecurrenceType {
	id: number;
	name: string;
	perYear: number;
}

export interface CurrencyType {
	id: number;
	abbreviation: string;
	name: string;
}

export type Transaction = {
	id: number;
	donorName: string;
	currencyType: CurrencyType;
	value: number;
	date: string;
	expenseId: number;
};

export interface Expense {
	id: number;
	name: string;
	description: string;
	recurrenceType: RecurrenceType;
	currencyType: CurrencyType;
	predefinedExpenseId: number | null;
	startDate: string;
	value: string;
	userId: number;
	transactions: Transaction[];
}

export type NewTransaction = Omit<Transaction, 'id' | 'currencyType'> & {
	currencyTypeId: number;
};

export type GetExpensesRequestData = {
	userId: number;
};

type GetExpensesResponseData = {
	expenses: Expense[];
};

export type GetExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetExpensesResponseData;
};

type GetCurrencyTypesResponseData = CurrencyType[];

export type GetCurrencyTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetCurrencyTypesResponseData;
};

export type NewTransactionRequestData = {
	newTransaction: NewTransaction;
};

export type NewTransactionResponse = Omit<AxiosResponse, 'data'>;

export type DeleteTransactionRequestData = {
	transactionId: number;
};

export type DeleteTransactionResponse = Omit<AxiosResponse, 'data'>;
