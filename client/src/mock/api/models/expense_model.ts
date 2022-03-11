import type { AxiosResponse } from '../_common/axios_response';

export type ExpensesRequestData = {
	userId: number;
};

export interface Expense {
    id: number,
    name: string,
    description: string,
    recurrence_type_id: number
    currency_type_id: number,
    predefined_expense_id: number | null,
    startDate: string,
    value: string,
    user_id: number
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

