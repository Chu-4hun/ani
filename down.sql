-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS  history, user_info, user_friends, review, episode, bookmark, users, releases;
DROP FUNCTION IF EXISTS  update_user_info_register_date;