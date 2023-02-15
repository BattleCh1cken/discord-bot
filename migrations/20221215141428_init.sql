create table if not exists reminders
(
  id integer primary key autoincrement not null,
  end_time datetime not null,
  user_id int not null,
  guild_id int not null,
  description text not null,
  remind_time datetime,
  active boolean not null,
  foreign key(user_id) references users(id)
  foreign key(guild_id) references guilds(id)
);

create table if not exists users
(
  id integer primary key autoincrement not null,
  user_id bigint not null,
  boop_score integer,
  rps_wins integer,
  missed_entries integer
);

create table if not exists guilds
(
  id integer primary key autoincrement not null,
  guild_id bigint not null,
  reminder_master_role bigint,
  reminder_channel bigint 
);

