import type { ErrorResponse } from '$bindings/handshake/ErrorResponse';
import { undefined } from 'zod';
import type { OkResponse } from '../../../papi/bindings/OkResponse';

export type Method = 'GET' | 'POST' | 'DELETE';
export type Response<T> = { type: "ok", data: OkResponse<T> } | { type: "err", error: ErrorResponse };

export default async function<T> (path: string, method: Method, data?: object): Promise<Response<T>> {
	const searchParams = new URLSearchParams();
	const request_init: RequestInit = {
		method: method,
		headers: {
			
		},
	}
	
	if (method === 'POST') {
		request_init.body = JSON.stringify(data);
		request_init.headers = {
			'Content-Type': 'application/json',
		}
	}
	
	if (method === 'GET' || method === 'DELETE') {
		for (const key in data) {
			// @ts-ignore
			searchParams.set(key, data[key]);
		}
	}
	
	const response = await fetch("http://localhost:8080/" + path + "?" + searchParams.toString(), request_init);
	
	if (!response.ok) {
		try {
			const error = await response.json();
			if (error.message) {
				return { type: "err", error: error.message as ErrorResponse };
			}
		} catch {
			return {
				type: "err",
				error: {
					message: response.statusText,
					errors: [
						{
							message: response.statusText,
							"kind": "public_fatal",
							"origin": "fetch",
						},
					],
				},
			}
		}
	}
	
	const json = await response.json();
	return { type: "ok", data: json as OkResponse<T>}
}