import { NodeSSH } from 'node-ssh';
import timeout from '$lib/api/timeout';
import { env } from '$env/dynamic/private';

export default async function() {
	const ssh = new NodeSSH();
	
	const password = env.SSH_KEY;
	if (!password) return { err: { message: "No SSH key provided!" }};
	const username = env.SSH_USER;
	if (!username) return { err: { message: "No SSH user provided!" }};
	const host = env.SSH_HOST;
	if (!host) return { err: { message: "No SSH host provided!" }};
	const port = env.SSH_PORT;
	if (!port) return { err: { message: "No SSH port provided!" }};

	return timeout(3000, () => ssh.connect({
		host, username, password,
		port: parseInt(port),
	}));
}