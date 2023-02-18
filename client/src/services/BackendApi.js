import axios from 'axios';

const axiosAPI = axios.create({
	baseURL: 'http://localhost:8000/api',
	withCredentials: true,
});

// implement a method to execute all the request from here.
const apiRequest = (method, url, request) => {
	const headers = {
		JSESSIONID: 'VYKEVrmu9L6B+3WA6SVoOzWeta67pPjlHV4F+OQ=',
	};
	//using the axios instance to perform the request that received from each http method
	return axiosAPI({
		method,
		url,
		data: request,
		headers,
	})
		.then((res) => {
			console.log('Backend request succeeded');
			//console.debug('Data {}', res.status);
			return Promise.resolve(res);
		})
		.catch((err) => {
			console.warn('Backend request failed');
			return Promise.reject({ error: err.toJSON() });
		});
};

const get = (url, request = {}) => apiRequest('GET', url, request);
const deleteRequest = (url, request = {}) => apiRequest('DELETE', url, request);
const post = (url, request = {}) => apiRequest('post', url, request);
const put = (url, request = {}) => apiRequest('PUT', url, request);
const patch = (url, request = {}) => apiRequest('PATCH', url, request);

export default {
	get,
	delete: deleteRequest,
	post,
	put,
	patch,
};
