CREATE TABLE IF NOT EXISTS entries
(
  id integer primary key asc,
  start_time datetime,
  end_time datetime
);
CREATE TABLE IF NOT EXISTS members
(
  id integer primary key asc,
  user_id int,
  name text
);

CREATE TABLE IF NOT EXISTS member_entries
(
  id integer primary key asc,
  FOREIGN KEY(entry_id) REFERENCES entries(id)
  FOREIGN KEY(member_id) REFERENCES members(id)
);
