import * as Endpoint from './endpoints';
import { env } from '$env/dynamic/private';
import { prisma } from '$lib/gen/db';
import { safeguard, safeguard_async } from '$lib/api/result';
import jwt from 'jsonwebtoken';

export async function POST({ url}) {
	const This = Endpoint.POST;
	
	const data = This.validateDataServer(url);
	
	if (data.err) { return This.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.errorMsg("Environments are not correct!", "env_not_correct");

	const u = await prisma.user.findFirst({
		where: { username: data.ok.username }
	})
	if (!u) return This.errorMsg("Username does not exist!", "username_not_exists");
	
	const unverif = await prisma.unverifiedUser.findFirst({
		where: { user_id: u.id }
	});
	if (!unverif) return This.errorMsg("User is already verified!", "user_already_verified");

	const token_verify = safeguard(() => jwt.verify(data.ok.token, secret));
	if (token_verify.err) return This.errorMsg("Token is not valid!", "token_not_valid");
	const token_verify2 = safeguard(() => jwt.verify(unverif.auth_jwt, secret));
	if (token_verify2.err) return This.errorMsg("Token is not valid!", "token_not_valid");
	
	// @ts-expect-error - This is a safeguard
	const n1 = token_verify.ok.random_number;
	// @ts-expect-error - This is a safeguard
	const n2 = token_verify2.ok.random_number;
	
	if (n1 && n2 && n1 !== n2) return This.errorMsg("Token is not equal!" , "token_not_valid");
	
	const result = await safeguard_async(() => prisma.unverifiedUser.delete({
		where: { user_id: u.id }
	}));
	if (result.err) return This.error(result.err);
	
	return This.send({ message: `User ${u.username} Logged In!`, token: u.jwt, username: u.username });
}