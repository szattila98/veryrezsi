
interface AxiosResponse {
  status: Number;
  statusText: String;
	data: Object;
	headers: Object;
	config: Object;
	request: Object;
}


/**
 * Generates mock axios response objects
 * @see https://axios-http.com/docs/res_schema
 * @param {AxiosResponse.status} status 
 * @param {AxiosResponse.data} data 
 * @param {AxiosResponse.headers} headers 
 * @param {AxiosResponse.config} conf 
 * @param {AxiosResponse.request} req 
 * @returns {AxiosResponse}
 */
let axios_response = (status = 200, data = {}, headers = {}, conf = {}, req = {}) => {
	let statusText;
	switch (status) {
		case 200: {
			statusText = 'OK';
			break;
		}
		case 401: {
			statusText = 'Unauthorized';
			break;
		}
		default: {
			statusText = 'Unknown mock response statusText';
		}
	}

	return {
		// `data` is the response that was provided by the server
		data: {},
	
		// `status` is the HTTP status code from the server response
		status: 200,
	
		// `statusText` is the HTTP status message from the server response
		// As of HTTP/2 status text is blank or unsupported.
		// (HTTP/2 RFC: https://www.rfc-editor.org/rfc/rfc7540#section-8.1.2.4)
		statusText: 'OK',
	
		// `headers` the HTTP headers that the server responded with
		// All header names are lower cased and can be accessed using the bracket notation.
		// Example: `response.headers['content-type']`
		headers: {},
	
		// `config` is the config that was provided to `axios` for the request
		config: {},
	
		// `request` is the request that generated this response
		// It is the last ClientRequest instance in node.js (in redirects)
		// and an XMLHttpRequest instance in the browser
		request: {}
	};
};

export {
	AxiosResponse,
	axios_response as response,
};