import type { AxiosResponse } from '$shared/axios';

export type NewExpense = {
	name: string;
	description: string;
	recurrenceTypeId: number;
	currencyTypeId: number;
	predefinedExpenseId: number | null;
	startDate: string;
	value: number;
	userId: number; // TODO: not needed with actual backend
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
