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

export interface Expense {
	id: number;
	name: string;
	description: string;
	recurrence_type: RecurrenceType;
	currency_type: CurrencyType;
	predefined_expense_id: number | null;
	startDate: string;
	value: string;
	user_id: number;
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
