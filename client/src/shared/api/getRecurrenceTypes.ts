import type { AxiosResponse } from '$mock/api/_common/axios_response';
import type { RecurrenceType } from '$shared/domain';

type GetRecurrenceTypesResponseData = RecurrenceType[];

export type GetRecurrenceTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetRecurrenceTypesResponseData;
};
