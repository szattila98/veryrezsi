import type { AxiosResponse } from '$shared/axios';
import type { User } from '$shared/domain';

type MeResponseData = {
	user: User;
};

export type MeResponse = Omit<AxiosResponse, 'data'> & {
	data: MeResponseData;
};

export const response_user = (id: number, email: string, username: string): MeResponseData => {
	return {
		user: {
			id,
			email,
			username,
		},
	};
};
