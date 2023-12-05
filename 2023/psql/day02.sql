\set QUIET
\pset tuples_only
\pset format unaligned

create temp table input_lines(
  game_id serial,
  game_id_ignore text, -- ignore id in the input, use serial
  line text
);

create temp table games(
  game_id int,
  round_id int,
  color text,
  amount int
);

\copy input_lines (game_id_ignore, line) from 'input.txt' with (FORMAT csv, DELIMITER ':');

insert into games
select game_id, round_id, split_part(grab, ' ', 2), split_part(grab, ' ', 1)::int
from
  input_lines,
  unnest(string_to_array(substr(line, 2), '; ')) with ordinality as u(contents, round_id),
  unnest(string_to_array(contents, ', ')) as grab;

select sum(game_id) from (
  select game_id
  from games
  group by game_id
  having bool_and(not (
    color = 'red' and amount > 12 or
    color = 'green' and amount > 13 or
    color = 'blue' and amount > 14
  ))
);

select sum(red * green * blue) from (
  select
    max(case when color = 'red' then amount else null end) as red,
    max(case when color = 'green' then amount else null end) as green,
    max(case when color = 'blue' then amount else null end) as blue
  from games
  group by game_id
);
