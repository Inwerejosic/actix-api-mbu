-- Your SQL goes here
CREATE TABLE "member" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "f_name" TEXT NOT NULL,
    "m_name" TEXT NOT NULL ,
    "l_name" TEXT NOT NULL ,
    "email" TEXT NOT NULL,
    "address" TEXT NOT NULL,
    "age" INTEGER NOT NULL
);