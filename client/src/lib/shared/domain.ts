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
	value: number;
	date: string;
	currency: Currency;
};

export interface PredefinedExpense {
	id: number;
	name: string;
	description: string;
	value: number;
	currency: Currency;
	recurrence: Recurrence;
}

export interface Expense {
	id: number;
	name: string;
	description: string;
	value: string;
	startDate: string;
	userId: number;
	currency: Currency;
	recurrence: Recurrence;
	predefinedExpenseId: number | null;
	transactions: Transaction[];
}
