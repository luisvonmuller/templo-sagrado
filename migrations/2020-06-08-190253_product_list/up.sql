-- Your SQL goes here
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