-- Your SQL goes here
CREATE TABLE `notes`(
	`id` INTEGER AUTO_INCREMENT PRIMARY KEY,
	`title` VARCHAR(255) NOT NULL,
	`body` TEXT NOT NULL,
	`published` BOOL NOT NULL DEFAULT FALSE
);

