CREATE TABLE user_type (
	user_type_id SERIAL UNIQUE CONSTRAINT user_type_pk PRIMARY KEY,
	user_type_title VARCHAR(127) NOT NULL
    );


CREATE TABLE sysuser (
  user_id SERIAL UNIQUE CONSTRAINT user_pk PRIMARY KEY,
  user_name VARCHAR(255) NOT NULL,
  user_email VARCHAR(127) NOT NULL,
  user_password VARCHAR(2047) NOT NULL,
  user_birthdate DATE NOT NULL,
  user_genre VARCHAR(31) NOT NULL,
  user_alias VARCHAR(255) UNIQUE NULL,
  user_newsletter BOOLEAN NOT NULL,
  user_creation TIMESTAMP NOT NULL,
  user_lasttimeonline TIMESTAMP NULL,
  user_points INTEGER NOT NULL,
  user_balance FLOAT NOT NULL,
  user_type_id INTEGER NOT NULL,
  CONSTRAINT sysuser_user_type_fk FOREIGN KEY (user_type_id) REFERENCES
  user_type(user_type_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION);

CREATE INDEX user_user_name_SK ON sysuser(user_name);

CREATE TABLE address (
  address_id SERIAL UNIQUE CONSTRAINT address_pk PRIMARY KEY,
  address_number VARCHAR(31) NULL,
  address_street VARCHAR(255) NULL,
  address_city VARCHAR(127) NULL,
  address_state VARCHAR(127) NULL,
  address_country VARCHAR(127) NULL,
  address_postalcode VARCHAR(63) NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT address_user_fk FOREIGN KEY (user_id) REFERENCES
  sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE phone_type (
  phone_type_id SERIAL UNIQUE CONSTRAINT phone_type_pk PRIMARY KEY,
  phone_type_title VARCHAR(31) NULL);

CREATE TABLE phone (
  phone_id SERIAL UNIQUE CONSTRAINT phone_pk PRIMARY KEY,
  phone_number VARCHAR(31) NOT NULL,
  user_id INT NOT NULL,
  phone_type_id INT NOT NULL,
  CONSTRAINT phone_user_fk FOREIGN KEY (user_id) REFERENCES
  sysuser(user_id),
  CONSTRAINT phone_phone_type_fk FOREIGN KEY (phone_type_id) REFERENCES
  phone_type (phone_type_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE end_call_type (
  end_call_type_id SERIAL UNIQUE CONSTRAINT end_call_type_pk PRIMARY KEY,
  end_call_type_title VARCHAR(127) NOT NULL);


CREATE TABLE call_type (
  call_type_id SERIAL UNIQUE CONSTRAINT call_type_pk PRIMARY KEY,
  call_type_title VARCHAR(63) NOT NULL,
  call_type_value FLOAT NOT NULL);

CREATE TABLE call (
  call_id SERIAL UNIQUE CONSTRAINT call_pk PRIMARY KEY,
  call_value FLOAT NULL,
  call_begin_date TIMESTAMP NOT NULL,
  call_end_date TIMESTAMP NULL,
  end_call_type_id INTEGER NULL,
  call_type_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT call_end_call_type_fk FOREIGN KEY (end_call_type_id) REFERENCES
  end_call_type(end_call_type_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION,
  CONSTRAINT call_call_type_fk FOREIGN KEY (call_type_id) REFERENCES
  call_type(call_type_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION,
  CONSTRAINT call_user_fk FOREIGN KEY (user_id) REFERENCES
  sysuser(user_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION);

CREATE TABLE payment (
  payment_id SERIAL UNIQUE CONSTRAINT payment_pk PRIMARY KEY,
  payment_value FLOAT NOT NULL,
  payment_status BOOLEAN NOT NULL,
  payment_date TIMESTAMP NOT NULL,
  payment_obs TEXT NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT payment_user_fk FOREIGN KEY (user_id) REFERENCES
  sysuser(user_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION);


CREATE TABLE payment_type (
  payment_type_id SERIAL UNIQUE CONSTRAINT payment_type_pk PRIMARY KEY,
  payment_type_title VARCHAR(31) NOT NULL,
  payment_type_incoming BOOLEAN NOT NULL);


CREATE TABLE product_category (
  product_category_id SERIAL UNIQUE CONSTRAINT product_category_pk PRIMARY KEY,
  product_category_title VARCHAR(127) NOT NULL);


CREATE TABLE product (
  product_id SERIAL UNIQUE CONSTRAINT product_pk PRIMARY KEY,
  product_title VARCHAR(127) NOT NULL,
  product_real_value FLOAT NOT NULL,
  product_points_value INTEGER NULL,
  product_image TEXT NOT NULL,
  product_description TEXT NULL,
  product_category_id INTEGER NOT NULL,
  CONSTRAINT payment_product_category_fk FOREIGN KEY (product_category_id)
  REFERENCES product_category(product_category_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION);


CREATE TABLE sale (
  sale_id SERIAL UNIQUE CONSTRAINT sale_pk PRIMARY KEY,
  sale_date TIMESTAMP NOT NULL,
  sale_real_value FLOAT NULL,
  sale_points_value INTEGER NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT sale_user_fk FOREIGN KEY (user_id) REFERENCES
  sysuser(user_id)
  ON DELETE NO ACTION
  ON UPDATE NO ACTION);

CREATE TABLE payment_source (
  payment_source_id SERIAL UNIQUE CONSTRAINT payment_source_pk PRIMARY KEY,
  payment_source_value FLOAT NOT NULL,
  payment_source_status BOOLEAN NOT NULL,
  payment_type_id INTEGER NOT NULL,
  payment_id INTEGER NOT NULL,
  CONSTRAINT payment_source_payment_type_fk FOREIGN KEY (payment_type_id)
  REFERENCES payment_type (payment_type_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT payment_source_payment_fk FOREIGN KEY (payment_id)
    REFERENCES payment(payment_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

 CREATE TABLE product_list (
  product_list_id SERIAL UNIQUE CONSTRAINT product_list_pk PRIMARY KEY,
  product_list_amount INTEGER NOT NULL,
  product_list_use_points BOOLEAN NOT NULL,
  product_id INTEGER NOT NULL,
  sale_id INTEGER NOT NULL,
  CONSTRAINT product_list_product_fk FOREIGN KEY (product_id)
    REFERENCES product (product_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT product_list_sale_fk
    FOREIGN KEY (sale_id)
    REFERENCES sale (sale_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE clerk_info (
  clerk_info_id SERIAL UNIQUE CONSTRAINT clerk_info_pk PRIMARY KEY,
  clerk_description TEXT NULL,
  clerk_info_experience TEXT NULL,
  user_id INT NOT NULL,
  CONSTRAINT clerk_info_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE product_review (
  product_review_id SERIAL UNIQUE CONSTRAINT product_review_pk PRIMARY KEY,
  products_review_title VARCHAR(255) NOT NULL,
  product_revie_stars SMALLINT NOT NULL,
  products_review_body TEXT NULL,
  product_review_is_anonymous BOOLEAN NOT NULL,
  product_review_date TIMESTAMP NULL,
  product_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT product_review_product_fk
    FOREIGN KEY (product_id)
    REFERENCES product (product_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT product_review_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE clerk_review (
  clerk_review_id SERIAL UNIQUE CONSTRAINT clerk_review_pk PRIMARY KEY,
  clerk_review_title VARCHAR(255) NOT NULL,
  clerk_review_stars SMALLINT NOT NULL,
  clerk_review_body TEXT NULL,
  clerk_review_is_anonymous BOOLEAN NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT clerk_review_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);


CREATE TABLE skill (
  skill_id SERIAL UNIQUE CONSTRAINT skill_pk PRIMARY KEY,
  skill_title VARCHAR(255) NOT NULL,
  skill_description TEXT NULL,
  skill_image TEXT NOT NULL,
  skill_status BOOLEAN NULL);


CREATE TABLE clerk_skill_list (
  clerk_skill_list_id SERIAL UNIQUE CONSTRAINT clerk_skill_list_pk PRIMARY KEY,
  clerk_skill_list_level SMALLINT NULL,
  clerk_skill_list_status BOOLEAN NOT NULL,
  clerk_skill_list_description TEXT NULL,
  user_id INTEGER NOT NULL,
  skill_id INTEGER NOT NULL,
  CONSTRAINT clerk_skill_list_user_id
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT clerk_skill_list_skill_fk
    FOREIGN KEY (skill_id)
    REFERENCES skill (skill_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);


CREATE TABLE payment_info (
  payment_info_id SERIAL UNIQUE CONSTRAINT payment_info_pk PRIMARY KEY,
  payment_info_bank_number VARCHAR(45) NOT NULL,
  payment_info_bank_name VARCHAR(127) NOT NULL,
  payment_info_account VARCHAR(31) NOT NULL,
  payment_info_CPF VARCHAR(15) NOT NULL,
  payment_info_account_type VARCHAR(45) NOT NULL,
  payment_info_favorecido VARCHAR(255) NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT payment_info_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE business_day (
  business_day_id SERIAL UNIQUE CONSTRAINT business_day_pk PRIMARY KEY,
  business_day_title VARCHAR(63) NULL);

CREATE TABLE business_hour_list (
  business_hour_list_id SERIAL UNIQUE CONSTRAINT business_hour_list_pk PRIMARY KEY,
  business_hour_list_begin TIME NOT NULL,
  business_hour_list_end TIME NOT NULL,
  business_hour_list_status BOOLEAN NOT NULL,
  business_day_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT business_hour_business_day_fk
    FOREIGN KEY (business_day_id)
    REFERENCES business_day (business_day_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT business_hour_list_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);


CREATE TABLE call_email (
  call_email_id SERIAL UNIQUE CONSTRAINT call_email_pk PRIMARY KEY,
  call_email_title VARCHAR(255) NOT NULL,
  call_email_body TEXT NOT NULL,
  call_id INTEGER NOT NULL,
  CONSTRAINT call_email_call_fk
    FOREIGN KEY (call_id)
    REFERENCES call (call_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);

CREATE TABLE message (
  message_id SERIAL UNIQUE CONSTRAINT message_email_pk PRIMARY KEY,
  clerk_info_id INTEGER NOT NULL,
  clerk_user_email VARCHAR(127) NOT NULL,
  user_id INTEGER NOT NULL,
  user_email VARCHAR(255) NOT NULL,
  message_header VARCHAR(255),
  message_body TEXT NOT NULL,
  CONSTRAINT message_user_fk
    FOREIGN KEY (user_id)
    REFERENCES sysuser (user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT message_clerk_info_fk
    FOREIGN KEY (clerk_info_id)
    REFERENCES clerk_info (clerk_info_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION);
