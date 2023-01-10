 create table if not exists entries
(
  id integer primary key autoincrement,
  end_time datetime not null,
  user_id integer not null,
  active boolean not null
);

create table if not exists boop_score
(
  id integer primary key autoincrement,
  score integer not null,
  user_id integer not null
);
