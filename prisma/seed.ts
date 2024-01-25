
import { PrismaClient } from '@prisma/client'
import pkg from 'bcryptjs';
const {genSalt, hash} = pkg;

const prisma = new PrismaClient();

async function main() {
	const salt = await genSalt();
	const hashed = await hash("adminohm", salt);
	const user = await prisma.user.upsert({
		where: { username: 'adminohm' },
		update: {},
		create: {
			email: 'randomemail37@gmail.com',
			username: 'adminohm',
			password: hashed,
			jwt: '',
		},
	});
	
	const permission = await prisma.permission.upsert({
		where: { permission: 'admin' },
		update: {},
		create: {
			level: 5,
			permission: 'admin',
		},
	})
	
	const user_perm_amount = await prisma.userPermission.count({
		where: { user_id: user.id }
	});

	console.log({ user, permission });
	if (user_perm_amount == 0) {
		const user_perm = await prisma.userPermission.create({
			data: {
				user_id: user.id,
				permission_id: permission.id
			}
		})
		
		console.log({ user_perm });
	}
}

main()
	.then(async () => {
		await prisma.$disconnect()
	})
	.catch(async (e) => {
		console.error(e)
		await prisma.$disconnect()
		process.exit(1)
	})
