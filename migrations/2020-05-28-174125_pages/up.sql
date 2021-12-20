-- Your SQL goes here
CREATE TABLE syspage
(
    syspage_id SERIAL UNIQUE CONSTRAINT syspage_id_pk PRIMARY KEY,
    syspage_title TEXT NOT NULL
);

CREATE TABLE syslayout_item
(
    syslayout_item_id SERIAL UNIQUE CONSTRAINT syslayout_item_id_pk PRIMARY KEY,
    syspage_content TEXT NOT NULL,
    syspage_id INT NOT NULL,
    CONSTRAINT syspage_idfk
    FOREIGN KEY (syspage_id)
    REFERENCES syspage(syspage_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);