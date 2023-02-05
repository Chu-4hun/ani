CREATE TABLE rel (
  release_id bigserial NOT NULL,
  release_type varchar(255) DEFAULT 'unknown' NOT NULL,
  release_date date NOT NULL,
  rating decimal DEFAULT '0' NOT NULL,
  min_age integer NOT NULL,
  director varchar(50) DEFAULT NULL,
  author varchar(50) DEFAULT NULL,
  studio varchar(50) DEFAULT NULL,
  description varchar(255) NOT NULL,
  PRIMARY KEY (release_id)
);

CREATE TABLE usr (
  user_id serial NOT NULL,
  user_name varchar(30) NOT NULL,
  pass varchar(30) NOT NULL,
  email varchar(255) NOT NULL,
  PRIMARY KEY (user_id)
);

CREATE TABLE bookmark (
  user_fk integer NOT NULL,
  type varchar(30) NOT NULL,
  bookmark_name varchar(50) NOT NULL,
  bookmark_id integer NOT NULL,
  PRIMARY KEY (bookmark_id),
  CONSTRAINT bookmark_user_fk_user_user_id_foreign FOREIGN KEY (user_fk) REFERENCES usr (user_id)
);

CREATE TABLE episode (
  episode_id integer NOT NULL,
  release_FK integer NOT NULL,
  ep_name varchar(255) DEFAULT NULL,
  url varchar(255) NOT NULL,
  PRIMARY KEY (episode_id),
  CONSTRAINT episode_release_FK_release_id_foreign FOREIGN KEY (release_FK) REFERENCES rel (release_id)
);

CREATE TABLE review (
  review_id integer NOT NULL,
  user_FK integer NOT NULL,
  review_text text NOT NULL,
  rev_data date NOT NULL,
  rating decimal NOT NULL,
  release_FK integer NOT NULL,
  PRIMARY KEY (review_id),
  CONSTRAINT review_release_release_id_foreign FOREIGN KEY (release_FK) REFERENCES rel (release_id),
  CONSTRAINT review_user_FK_user_user_id_foreign FOREIGN KEY (user_FK) REFERENCES usr (user_id)
);

CREATE TABLE user_friends (
  usr integer NOT NULL,
  friend integer NOT NULL,
  CONSTRAINT user_friends_user_user_user_id_foreign FOREIGN KEY (usr) REFERENCES usr (user_id),
  CONSTRAINT user_friends_friend_user_user_id_foreign FOREIGN KEY (friend) REFERENCES usr (user_id)
);

CREATE TABLE user_info (
  user_FK integer NOT NULL,
  avatar varchar(255) NOT NULL,
  status varchar(255) NOT NULL,
  register_date date NOT NULL,
  PRIMARY KEY (user_FK),
  CONSTRAINT User_info_user_FK_user_user_id_foreign FOREIGN KEY (user_FK) REFERENCES usr (user_id)
);

CREATE TABLE bookmark_to_releases (
  bookmark_FK integer NOT NULL,
  release_FK integer NOT NULL,
  CONSTRAINT bookmark_to_releases_release_FK_release_id_foreign FOREIGN KEY (release_FK) REFERENCES rel (release_id),
  CONSTRAINT bookmark_to_releases_bookmark_FK_bookmark_bookmark_id_foreign FOREIGN KEY (bookmark_FK) REFERENCES bookmark (bookmark_id)
);

CREATE TABLE history (
  user_fk integer NOT NULL,
  episode integer NOT NULL,
  CONSTRAINT history_user_fk_user_user_id_foreign FOREIGN KEY (user_fk) REFERENCES usr (user_id),
  CONSTRAINT history_episode_episode_episode_id_foreign FOREIGN KEY (episode) REFERENCES episode (episode_id)
);