// Between 8-30, at least one uppercase letter, one lowercase letter, one number and one special character
export const PASSWORD_REGEXP =
	/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,30}$/;
