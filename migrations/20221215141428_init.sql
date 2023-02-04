create table if not exists entries
(
  id integer primary key autoincrement not null,
  end_time datetime not null,
  user_id integer not null,
  description text not null,
  remind boolean not null,
  remind_time integer not null,
  active boolean not null,
  foreign key(user_id) references users(id)
);

create table if not exists users
(
  id integer primary key autoincrement not null,
  user_id bigint not null,
  boop_score bigint not null,
  rps_wins integer not null,
  missed_entries integer not null
);
