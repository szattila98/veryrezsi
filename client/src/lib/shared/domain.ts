export type User = {
	id: number;
	email: string;
	username: string;
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
	currencyTypeId: number;
	value: number;
	date: string;
	expenseId: number;
};

export interface PredefinedExpense {
	id: number;
	name: string;
	description: string;
	value: number;
	recurrenceTypeId: number;
	currencyTypeId: number;
}

export interface Expense {
	id: number;
	name: string;
	description: string;
	value: string;
	startDate: string;
	userId: number;
	recurrenceTypeId: number;
	currencyTypeId: number;
	predefinedExpenseId: number | null;
	transactions: Transaction[];
}
