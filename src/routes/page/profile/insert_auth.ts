// import { client_auth } from '$api/auth';
//
// export default function ({token, username}: {token?: string, username?: string}) {
// 	if (!token || !username) {
// 		return;
// 	}
//	
// 	localStorage.setItem("jwt", token);
// 	localStorage.setItem("username", username);
// 	client_auth.set({ jwt: token, username });
//
// 	const queryParams = new URLSearchParams(window.location.search);
// 	const from = queryParams.get("from");
// 	if (from != null) {
// 		window.location.href= from;
// 	}
// }