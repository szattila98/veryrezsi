import Backend from '$services/BackendApi';
import type { GetRecurrenceTypesResponse } from '$shared/api/getRecurrenceTypes';

export async function getRecurrenceTypes(): Promise<GetRecurrenceTypesResponse> {
	return mockGetRecurrenceTypes();
}
