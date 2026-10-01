
drop table if exists web_user;
create table web_user (
	username varchar(20) primary key,
	tutor_id INT,
	user_password CHAR(100) not null
	);
