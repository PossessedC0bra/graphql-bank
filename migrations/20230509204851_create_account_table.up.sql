CREATE TABLE `account` (
    `id` int NOT NULL AUTO_INCREMENT,
    `owner` VARCHAR(255) NOT NULL,
    `balance` DOUBLE NOT NULL,
    PRIMARY KEY (`id`)
);