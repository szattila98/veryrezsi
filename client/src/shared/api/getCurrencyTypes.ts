import type { AxiosResponse } from '$mock/api/_common/axios_response';
import type { CurrencyType } from '$shared/domain';

type GetCurrencyTypesResponseData = CurrencyType[];

export type GetCurrencyTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetCurrencyTypesResponseData;
};
