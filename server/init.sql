CREATE DATABASE IF NOT EXISTS `veryrezsi`;
CREATE USER 'veryrezsi'@'%' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON veryrezsi.* TO 'veryrezsi'@'%';