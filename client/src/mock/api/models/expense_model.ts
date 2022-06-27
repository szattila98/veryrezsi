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
	predefinedExpenseId: number | null; // TODO what to do with this?
	startDate: string;
	value: string;
	userId: number;
	transactions: Transaction[];
}

export interface PredefinedExpense {
	id: number;
	name: string;
	description: string;
	recurrenceType: RecurrenceType;
	currencyType: CurrencyType;
	value: number;
}

type GetPredefinedExpensesResponseData = PredefinedExpense[];

export type GetPredefinedExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetPredefinedExpensesResponseData;
};

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

export type NewExpense = Omit<
	Expense,
	'id' | 'transactions' | 'recurrenceType' | 'currencyType'
> & {
	recurrenceTypeId: number;
	currencyTypeId: number;
};

export type NewExpenseRequestData = {
	newExpense: NewExpense;
};

type NewExpenseResponseData = {
	expenseId: number;
};

export type NewExpenseResponse = Omit<AxiosResponse, 'data'> & {
	data: NewExpenseResponseData;
};

type GetCurrencyTypesResponseData = CurrencyType[];

export type GetCurrencyTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetCurrencyTypesResponseData;
};

type GetRecurrenceTypesResponseData = RecurrenceType[];

export type GetRecurrenceTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetRecurrenceTypesResponseData;
};

export type NewTransactionRequestData = {
	newTransaction: NewTransaction;
};

export type NewTransactionResponse = Omit<AxiosResponse, 'data'>;

export type DeleteTransactionRequestData = {
	transactionId: number;
};

export type DeleteTransactionResponse = Omit<AxiosResponse, 'data'>;
