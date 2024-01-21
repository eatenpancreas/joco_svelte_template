import * as This from './endpoints';
import { PrismaClient } from '@prisma/client';
import { env } from '$env/dynamic/private';
import {asyncSign} from "$lib/gen/asyncify";
import {safeguard_async} from "$lib/api/result";
import pkg from 'bcryptjs';
const {genSalt, hash} = pkg;

export async function POST({ url}) {
	const data = This.POST.validateDataServer(url);
	if (data.err) { return This.POST.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.POST.errorMsg("Environments are not correct!", "env_not_correct");
	
	const token = await asyncSign({ username: data.ok.username, password: data.ok.password }, 
		secret, { expiresIn: '10h' });
	
	if (token.err) return This.POST.error(token.err);

	const prisma = new PrismaClient();
	This.POST.dbInit(prisma);

	let u = await prisma.user.findFirst({
		where: { username: data.ok.username }
	})
	
	if (u) return This.POST.errorMsg("Username already exists!", "username_exists");
	
	const result = await safeguard_async(async () => {
		const salt = await genSalt();
		const hashed = await hash(data.ok.password, salt);
		
		return prisma.user.create({
			data: {
				username: data.ok.username,
				password: hashed,
				jwt: token.ok
			}
		});
	});
	
	if (result.err) { return This.POST.error(result.err); }

	return This.POST.send({ message: `User ${result.ok.username} Created!`, token: result.ok.jwt, username: result.ok.username });
}