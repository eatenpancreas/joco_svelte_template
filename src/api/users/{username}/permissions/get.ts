
// This file was generated by [papi]. Do not edit this file manually.
	import user from '$lib/http/user';
	import { get as getStore } from 'svelte/store';
	
import { type Response, request_inner } from '$lib/http/response';
const method = 'get'; 
const host = import.meta.env.VITE_API_URL?? ''; 
import type { __Endpoint_get_user_permissions as GET } from '$lib/endpoint_defines//users/{username}/permissions/GET'; 
export default async function get(in_val: GET['in_type'], username: string,query?: GET['query_type']): Promise<Response<GET['out_data_type']>> {
  const params = new URLSearchParams();
  for (const key in query) {
    params.set(key, query[key]);
  }
	const request_init: RequestInit = {
    credentials: 'include',
		method: method,
		headers: {
        'Authorization-User': getStore(user)?.username,
		},
	}
  const path = '/users/' + username + '/permissions'

	return request_inner(host, path, params, request_init);
}
  