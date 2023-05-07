export type User = {
	id: number;
	email: string;
	username: string;
};

export interface Recurrence {
	id: number;
	name: string;
	perYear: number;
}

export interface Currency {
	id: number;
	abbreviation: string;
	name: string;
}

export type Transaction = {
	id: number;
	donorName: string;
	currency: Currency;
	value: number;
	date: string;
	expenseId: number;
};

export interface PredefinedExpense {
	id: number;
	name: string;
	description: string;
	recurrence: Recurrence;
	currency: Currency;
	value: number;
}

export interface Expense {
	id: number;
	name: string;
	description: string;
	recurrence: Recurrence;
	currency: Currency;
	predefinedExpenseId: number | null;
	startDate: string;
	value: string;
	userId: number;
	transactions: Transaction[];
}
