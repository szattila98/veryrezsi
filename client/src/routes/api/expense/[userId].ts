import { mockGetExpenses } from '$mock/api/expense/expenses';
import type { GetExpensesResponse, GetExpensesRequestData } from '$shared/api/getExpenses';
import type { RequestEvent } from '@sveltejs/kit';

export async function get({
	params,
}: RequestEvent): Promise<{ status: number; body: GetExpensesResponse }> {
	const res = mockGetExpenses({ userId: parseInt(params.userId) });
	return {
		status: 200,
		body: res,
	};
}

export async function getExpenses(data: GetExpensesRequestData): Promise<GetExpensesResponse> {
	return mockGetExpenses({ userId: data.userId });
}
