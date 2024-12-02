\set QUIET
\pset tuples_only
\pset format unaligned

create temp table input_lines(line text);
create temp table columns(a int, b int);

\copy input_lines (line) from stdin with (FORMAT csv, DELIMITER ';');
STDIN_INPUT\.

insert into columns (a, b)
select
  split_part(line, '   ', 1)::int as a,
  split_part(line, '   ', 2)::int as b
from input_lines;

with
  sorted_a as (select a, row_number() over (order by a) as id from columns),
  sorted_b as (select b, row_number() over (order by b) as id from columns)
select sum(abs(a.a - b.b))
from sorted_a a
join sorted_b b on a.id = b.id;

select sum(c.a * (select count(*) from columns where b = c.a))
from columns as c;
