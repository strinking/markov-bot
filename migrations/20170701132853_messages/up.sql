CREATE TABLE Message (
   id bigint PRIMARY KEY,
   content TEXT NOT NULL,
   guild_id bigint NOT NULL,
   channel_id bigint NOT NULL,
   author_id bigint NOT NULL
)
