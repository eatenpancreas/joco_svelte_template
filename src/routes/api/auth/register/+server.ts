import * as This from './endpoints';
import { PrismaClient } from '@prisma/client';
import jwt from 'jsonwebtoken';
import { env } from '$env/dynamic/private';
import type { Result } from '$lib/api/result';

async function asyncSign(payload: string | object | Buffer, secret: jwt.Secret, options: jwt.SignOptions): Promise<Result<string, any>> {
	return new Promise((resolve, reject) => {
		jwt.sign(payload, secret, options, (err, token) => {
			if (err) resolve({ err: err });
			if (!token) resolve({ err: { message: "Token failed to be created!" } });
			else resolve({ ok: token });
		});
	});
}

export async function POST({ url}) {
	const data = This.POST.validateDataServer(url);
	if (data.err) { return This.POST.error(data.err); }
	
	const secret = env.JWT_SECRET;
	if (!secret) return This.POST.error({ message: "Environments are not correct!" });
	
	const token = await asyncSign({ username: data.ok.username, password: data.ok.password }, 
		secret, { expiresIn: '10h' });
	
	if (token.err) return This.POST.error(token.err);

	const prisma = new PrismaClient();
	prisma.user.create({
		data: {
			username: data.ok.username,
			password: data.ok.password,
			jwt: token.ok
		}
	});

	return This.POST.send({ message: "User Created!", token: token.ok });
}