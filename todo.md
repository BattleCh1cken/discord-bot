- Fix entry list
  - display time correctly
  - display expired entries
  - error on no entry return


## Reorg

### Support for Multiple Guilds
- settings command
  - guild db field
```sql
create table if not exists guilds
(
  id integer primary key autoincrement not null,
  guild_id bigint not null,
  reminder_master_role bigint not null,
  reminder_channel bigint not null,
);

```

### Reminder Command
- remind option (either null for no remind, or )
- description
- target user (can only be used by reminder_master, defaults to self)
```sql
create table if not exists reminders
(
  id integer primary key autoincrement not null,
  user_id bigint not null, -- Should I have a users table?
  end_time datetime not null,
  description text not null,
  remind_time datetime, -- Optional
  active boolean not null,
);

```
### Boop Command
creates way too many API requests, either rework or make an alternative

### Transmission command
- fetch messages from dm
- send messages
