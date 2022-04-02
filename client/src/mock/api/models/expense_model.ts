import type { AxiosResponse } from '../_common/axios_response';

export type ExpensesRequestData = {
	userId: number;
};

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
	value: string;
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

type ExpensesResponseData = {
	expenses: Expense[];
};

// Specializing common Axios response to use response data type for its data field
// This can be safely reused for every concrete response.
// There is no chance 'data' field name will change in Axios
export type ExpensesResponse = Omit<AxiosResponse, 'data'> & {
	data: ExpensesResponseData;
};
