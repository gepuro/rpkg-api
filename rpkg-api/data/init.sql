DROP TABLE raw_rpkg;
.mode tabs
CREATE TABLE raw_rpkg(
    content json not null
);
.import pkg.json raw_rpkg
-- select count(1) from raw_rpkg;
-- select * from raw_rpkg where content LIKE "%AlexandreMillette1989/OneStopAnova%";

DROP TABLE rpkg;
CREATE TABLE rpkg(
	pkg_name TEXT,
	title TEXT,
	url TEXT NOT NULL PRIMARY KEY
);

insert into rpkg
select
json_extract(content, '$.pkg_name') pkg_name,
json_extract(content, '$.title') title,
json_extract(content, '$.url') url
from raw_rpkg
where url not in (select url from rpkg)
;
