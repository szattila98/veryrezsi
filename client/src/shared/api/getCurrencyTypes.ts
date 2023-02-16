import type { AxiosResponse } from '$shared/axios';
import type { CurrencyType } from '$shared/domain';

type GetCurrencyTypesResponseData = CurrencyType[];

export type GetCurrencyTypesResponse = Omit<AxiosResponse, 'data'> & {
	data: GetCurrencyTypesResponseData;
};
