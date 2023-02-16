import type { RequestEvent } from '@sveltejs/kit';
import Backend from '$services/BackendApi';
import type { DeleteTransactionResponse } from '$shared/api/deleteTransaction';

/** @type {import('./[id]').RequestHandler} */
export async function POST({ params }: RequestEvent): Promise<DeleteTransactionResponse> {
	return mockDeleteTransaction({ transactionId: parseInt(params.id) });
}
