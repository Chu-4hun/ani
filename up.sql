CREATE TABLE releases (
  id serial NOT NULL,
  release_type int NOT NULL,
  release_name varchar(255) DEFAULT 'unknown' NOT NULL,
  release_date date NOT NULL,
  rating decimal DEFAULT '0' NOT NULL,
  min_age integer NOT NULL,
  director varchar(50) DEFAULT NULL,
  author varchar(50) DEFAULT NULL,
  studio varchar(50) DEFAULT NULL,
  description varchar(255) NOT NULL,
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
  release_FK integer NOT NULL,
  ep_name varchar(255) DEFAULT NULL,
  url varchar(255) NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT episode_release_FK_release_id_foreign FOREIGN KEY (release_FK) REFERENCES releases (id)
);

CREATE TABLE review (
  id serial NOT NULL,
  user_FK integer NOT NULL,
  review_text text NOT NULL,
  rev_data date NOT NULL,
  rating decimal NOT NULL,
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
  id serial NOT NULL,
  user_FK integer NOT NULL,
  avatar varchar(255) NOT NULL,
  status varchar(255) NOT NULL,
  register_date date NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT User_info_user_FK_user_user_id_foreign FOREIGN KEY (user_FK) REFERENCES users (id)
);

CREATE TABLE history (
  id serial NOT NULL,
  user_fk integer NOT NULL,
  episode integer NOT NULL,
  date_watched date NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT history_user_fk_user_user_id_foreign FOREIGN KEY (user_fk) REFERENCES users (id),
  CONSTRAINT history_episode_episode_episode_id_foreign FOREIGN KEY (episode) REFERENCES episode (id)
);

CREATE OR REPLACE FUNCTION update_user_info_register_date() RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO user_info (user_FK, avatar, status, register_date)
  VALUES (NEW.id, 'https://randomuser.me/api/portraits/lego/2.jpg', 'active', now());
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER insert_register_date
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION update_user_info_register_date();