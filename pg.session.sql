SELECT * from user_friend_requests where friend ='2';

INSERT INTO user_friend_requests (usr, friend, request_status)
        VALUES (1, 2, 1)
        RETURNING *;