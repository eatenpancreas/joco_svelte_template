
// This file was generated by [papi]. Do not edit this file manually.

import { type Response, request_inner } from '$lib/http/response';
const method = 'post'; 
const host = import.meta.env.VITE_API_URL?? ''; 
import type { __Endpoint_login as POST } from '$lib/endpoint_defines//auth/login/POST'; 
export default async function post(in_val: POST['in_type'],searchParams?: URLSearchParams): Promise<Response<POST['out_data_type']>> {
  const params = searchParams?? new URLSearchParams();
	const request_init: RequestInit = {
        credentials: 'same-origin',
		method: method,
		headers: {},
	}
  const path = '//auth/login'

  request_init.body = JSON.stringify(in_val);
  request_init.headers = {
    'Content-Type': 'application/json',
  }
    
	return request_inner(host, path, params, request_init);
}
  