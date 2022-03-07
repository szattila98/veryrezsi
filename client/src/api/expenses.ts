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

import data from "../mock/entities/user_expense.json"

const listAllExpenses = async (userId: number): Promise<Expense[]>  => { 
    return Promise.resolve(data)
}

export const expense_api = {
    listAllExpenses
}