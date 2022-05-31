CREATE DATABASE IF NOT EXISTS `veryrezsi`;
CREATE USER IF NOT EXISTS 'veryrezsi'@'%' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON veryrezsi.* TO 'veryrezsi'@'%';