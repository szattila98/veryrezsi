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
	currencyType: CurrencyType;
	value: number;
	date: string;
	expenseId: number;
};

export interface PredefinedExpense {
	id: number;
	name: string;
	description: string;
	recurrenceType: RecurrenceType;
	currencyType: CurrencyType;
	value: number;
}

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
