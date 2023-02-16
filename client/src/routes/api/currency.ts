import Backend from '$services/BackendApi';
import type { GetCurrencyTypesResponse } from '$shared/api/getCurrencyTypes';

export async function getCurrencyTypes(): Promise<GetCurrencyTypesResponse> {
	return mockGetCurrencyTypes();
}
