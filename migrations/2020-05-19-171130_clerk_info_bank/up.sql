-- Your SQL goes here
CREATE TABLE clerk_bank
(
    clerk_bank_id SERIAL UNIQUE CONSTRAINT clerk_bank_pk PRIMARY KEY,
    clerk_id INT NOT NULL,
    clerk_bank_name TEXT NOT NULL,
    clerk_bank_account_type VARCHAR(2) NOT NULL,
    clerk_bank_agency_number VARCHAR(12) NOT NULL,
    clerk_bank_acc_number VARCHAR(20) NOT NULL,
    clerk_bank_cpf VARCHAR(16) NOT NULL,
    CONSTRAINT clerk_bank_clerk_id_fk
    FOREIGN KEY (clerk_id)
    REFERENCES sysuser(user_id)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION
);