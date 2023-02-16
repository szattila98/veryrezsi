import type { AxiosResponse } from '$shared/axios';
import type { RecurrenceType } from '$shared/domain';

type GetRecurrenceTypesResponseData = RecurrenceType[];

export type GetRecurrenceTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetRecurrenceTypesResponseData;
};
