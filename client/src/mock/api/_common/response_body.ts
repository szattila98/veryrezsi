export type MessageResponseBody = {
	message: string;
};

/**
 * Generates our common response body object.
 * @param msg Message of the response body.
 */
const response_message = (msg = ''): MessageResponseBody => {
	return {
		message: msg,
	};
};

export { response_message };
