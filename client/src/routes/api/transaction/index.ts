import Backend from '$services/BackendApi';
import type { NewTransactionResponse, NewTransaction } from '$shared/api/newTransaction';
import type { RequestEvent } from '@sveltejs/kit';

/** @type {import('./[id]').RequestHandler} */
export async function POST({ request }: RequestEvent): Promise<NewTransactionResponse> {
	const body: NewTransaction = await request.json();
}
