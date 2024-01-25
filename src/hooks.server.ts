import schedule from "node-schedule"
import { prisma } from '$lib/gen/db';

schedule.scheduleJob('0 */1 * * *', async () => {
	console.log("Running cron job!");
	const finds = await prisma.user.findMany({
		where: {
			// 																						MS - SEC - MIN
			created_at: { lt: new Date(Date.now() - 1000 * 60 * 60) },
			unverified_users: { some: {} }
		}
	});
	
	const unverified_delete = finds.map(f => prisma.unverifiedUser.delete({
			where: { user_id: f.id }
	}));
	
	const user_delete = finds.map(f => prisma.user.delete({
			where: { id: f.id }
	}));
	
	const del = await prisma.$transaction([
		...unverified_delete,
		...user_delete
	]);
	
	console.log("Deleted: " + del.length)
});