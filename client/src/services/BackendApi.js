import axios from 'axios';

const axiosAPI = axios.create({
	baseURL: 'http://localhost:8000/api',
});

// implement a method to execute all the request from here.
const apiRequest = (method, url, request) => {
	const headers = {
		authorization: '',
	};
	//using the axios instance to perform the request that received from each http method
	return axiosAPI({
		method,
		url,
		data: request,
		headers,
	})
		.then((res) => {
			console.log('API Success');
			return Promise.resolve(res.data);
		})
		.catch((err) => {
			console.warn('API Fail');
			return Promise.reject(err);
		});
};

// function to execute the http get request
const get = (url, request) => apiRequest('GET', url, request);

// function to execute the http delete request
const deleteRequest = (url, request) => apiRequest('DELETE', url, request);

// function to execute the http post request
const post = (url, request) => apiRequest('post', url, request);

// function to execute the http put request
const put = (url, request) => apiRequest('PUT', url, request);

// function to execute the http path request
const patch = (url, request) => apiRequest('PATCH', url, request);

// expose your method to other services or actions
const API = {
	get,
	delete: deleteRequest,
	post,
	put,
	patch,
};
export default API;
