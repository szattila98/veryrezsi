CREATE DATABASE IF NOT EXISTS `veryrezsi`;
CREATE USER IF NOT EXISTS 'veryrezsi'@'%' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON veryrezsi.* TO 'veryrezsi'@'%';

# Account activation remover scheduled event, run after migrations
SET GLOBAL event_scheduler = ON;
CREATE EVENT veryrezsi.account_activation_garbage_collection
ON SCHEDULE EVERY 1 DAY
COMMENT 'Removes expired account_activation rows, once a day'
DO DELETE FROM veryrezsi.account_activations WHERE expiration < now();