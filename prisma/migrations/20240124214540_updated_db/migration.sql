/*
  Warnings:

  - The primary key for the `UserPermission` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `id` on the `UserPermission` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[email]` on the table `User` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "Permission" ADD COLUMN     "created_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP;

-- AlterTable
ALTER TABLE "UserPermission" DROP CONSTRAINT "UserPermission_pkey",
DROP COLUMN "id",
ADD CONSTRAINT "UserPermission_pkey" PRIMARY KEY ("user_id", "permission_id");

-- CreateTable
CREATE TABLE "UnverifiedUser" (
    "user_id" UUID NOT NULL,
    "auth_jwt" VARCHAR(511) NOT NULL,

    CONSTRAINT "UnverifiedUser_pkey" PRIMARY KEY ("user_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "unique_email" ON "User"("email");

-- AddForeignKey
ALTER TABLE "UnverifiedUser" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "User"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;
