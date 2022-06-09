import type { RequestEvent } from '@sveltejs/kit';
import type { DeleteTransactionResponse } from '$mock/api/models/expense_model';
import { mockDeleteTransaction } from '$mock/api/expense/expenses';

/** @type {import('./[id]').RequestHandler} */
export async function post({ params }: RequestEvent): Promise<DeleteTransactionResponse> {
	return mockDeleteTransaction({ transactionId: parseInt(params.id) });
}
