import * as This from './endpoints';
import ssh_conn from '$lib/server/ssh_conn';

export async function GET() {
	const conn = await ssh_conn();
	if (conn.err) return This.GET.error(conn.err);
	if (!conn.ok.isConnected()) return This.GET.error({ message: "Not connected!" });
	return This.GET.send({ message: "Connected!" });
}