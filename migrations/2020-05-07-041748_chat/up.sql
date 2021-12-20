-- Your SQL goes here
CREATE TABLE chat
(
    chat_id SERIAL UNIQUE CONSTRAINT chat_pk PRIMARY KEY,
    client_id INT NOT NULL,
    clerk_id INT NOT NULL,
    client_socket VARCHAR(127) NOT NULL,
    clerk_socket VARCHAR(127) NOT NULL,
    init_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    CONSTRAINT chat_client_id_fk
    FOREIGN KEY (client_id)
    REFERENCES sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
    CONSTRAINT chat_clerk_id_fk
    FOREIGN KEY (clerk_id)
    REFERENCES sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);


CREATE TABLE chat_msg
(
    chat_msg_id SERIAL UNIQUE CONSTRAINT chat_msg_pk PRIMARY KEY,
    chat_msg_user_id INT NOT NULL,
    chat_msg_body TEXT NULL,
    chat_msg_time TIMESTAMP NOT NULL,
    chat_id INT NOT NULL,
    CONSTRAINT chat_msg_user_idfk
    FOREIGN KEY (chat_msg_user_id)
    REFERENCES sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
    CONSTRAINT chat_pk_id_fk
    FOREIGN KEY (chat_id)
    REFERENCES chat(chat_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);