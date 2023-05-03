CREATE TABLE dub (
  id serial NOT NULL,
  name varchar(50) NOT NULL,
  PRIMARY KEY (id)
);
CREATE TABLE releases (
  id serial NOT NULL,
  release_type integer DEFAULT 0 NOT NULL,
  release_date TIMESTAMP WITH TIME ZONE DEFAULT now(),
  rating real DEFAULT '0' NOT NULL,
  min_age integer NOT NULL,
  director varchar(200) DEFAULT 'unknown',
  author varchar(200) DEFAULT 'unknown',
  studio varchar(200) DEFAULT 'unknown',
  description text DEFAULT 'none' NOT NULL,
  release_name varchar(255) DEFAULT 'none' NOT NULL,
  img text DEFAULT 'https://kawai.shikimori.one/uploads/poster/animes/1/783ef0f9cb5' NOT NULL,
  external_id text NOT NULL,
  PRIMARY KEY (id)
);
CREATE TABLE users (
  id serial NOT NULL,
  login varchar(30) NOT NULL,
  password varchar(255) NOT NULL,
  email varchar(255) NOT NULL,
  PRIMARY KEY (id)
);
CREATE TABLE bookmark (
  id serial NOT NULL,
  user_fk integer NOT NULL,
  bookmark_name varchar(50) NOT NULL,
  release_FK integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT bookmark_user_fk_user_user_id_foreign FOREIGN KEY (user_fk) REFERENCES users (id),
  CONSTRAINT bookmark_release_FK_releases_release_id_foreign FOREIGN KEY (release_FK) REFERENCES releases (id)
);
CREATE TABLE episode (
  id serial NOT NULL,
  release_fk integer NOT NULL,
  dub_fk integer NOT NULL,
  ep_name varchar(255) DEFAULT 'unknown',
  url varchar(255) NOT NULL,
  position integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT episode_dub_fk_dub_id_foreign FOREIGN KEY (dub_fk) REFERENCES dub (id),
  CONSTRAINT episode_release_fk_releases_id_foreign FOREIGN KEY (release_fk) REFERENCES releases (id)
);
CREATE TABLE review (
  id serial NOT NULL,
  user_FK integer NOT NULL,
  review_text text NOT NULL,
  rev_data TIMESTAMP WITH TIME ZONE NOT NULL,
  rating real NOT NULL,
  release_FK integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT review_release_release_id_foreign FOREIGN KEY (release_FK) REFERENCES releases (id),
  CONSTRAINT review_user_FK_user_user_id_foreign FOREIGN KEY (user_FK) REFERENCES users (id)
);
CREATE TABLE user_friend_requests (
  id serial NOT NULL,
  usr integer NOT NULL,
  friend integer NOT NULL,
  request_status integer NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT user_friends_user_user_user_id_foreign FOREIGN KEY (usr) REFERENCES users (id),
  CONSTRAINT user_friends_friend_user_user_id_foreign FOREIGN KEY (friend) REFERENCES users (id)
);
CREATE TABLE user_info (
  id integer NOT NULL,
  avatar varchar(255) NOT NULL,
  status varchar(255) NOT NULL,
  register_date TIMESTAMP WITH TIME ZONE NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT user_info_id_users_id_foreign FOREIGN KEY (id) REFERENCES users (id)
);
CREATE TABLE history (
  id serial NOT NULL,
  user_fk integer NOT NULL,
  episode integer NOT NULL,
  date_watched TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  duration float not null default 0.0,
  PRIMARY KEY (id),
  CONSTRAINT history_user_fk_user_user_id_foreign FOREIGN KEY (user_fk) REFERENCES users (id),
  CONSTRAINT history_episode_episode_episode_id_foreign FOREIGN KEY (episode) REFERENCES episode (id)
);

CREATE OR REPLACE FUNCTION update_user_info_register_date() RETURNS TRIGGER AS $$ BEGIN
INSERT INTO user_info (id, avatar, status, register_date)
VALUES (
    NEW.id,
    'https://randomuser.me/api/portraits/lego/2.jpg',
    'active',
    now()
  );
RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER insert_register_date
AFTER
INSERT ON users FOR EACH ROW EXECUTE FUNCTION update_user_info_register_date();

-- HISTORY --
CREATE OR REPLACE FUNCTION delete_old_history_rows() RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT COUNT(*) FROM history WHERE user_fk = NEW.user_fk) >= 5 THEN
        DELETE FROM history WHERE id = (SELECT id FROM history WHERE user_fk = NEW.user_fk ORDER BY date_watched ASC LIMIT 1);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER delete_old_history_rows_trigger
BEFORE INSERT ON history
FOR EACH ROW
EXECUTE FUNCTION delete_old_history_rows();