CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       username VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE friendships (
       lower_id BIGINT NOT NULL,
       higher_id BIGINT NOT NULL,
       CONSTRAINT keys_ordered CHECK (lower_id < higher_id),
       CONSTRAINT fk_lower_id FOREIGN KEY(lower_id) REFERENCES users(id),
       CONSTRAINT fk_higher_id FOREIGN KEY(higher_id) REFERENCES users(id),
       PRIMARY KEY (lower_id, higher_id)
);

CREATE TABLE currencies (
       id SERIAL PRIMARY KEY,
       name VARCHAR(100) UNIQUE NOT NULL
);


CREATE TABLE expenses (
       id SERIAL PRIMARY KEY,
       description VARCHAR(1000),
       amount BIGINT NOT NULL,
       currency_id BIGINT NOT NULL,
       CONSTRAINT fk_currency_id FOREIGN KEY(currency_id) REFERENCES currencies(id),
       CONSTRAINT amount_positive CHECK (amount > 0)
);

CREATE TABLE user_to_expense (
       user_id BIGINT NOT NULL,
       expense_id BIGINT NOT NULL,
       proportion_owed BIGINT NOT NULL,
       amount_paid BIGINT,
       CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id),
       CONSTRAINT fk_expense_id FOREIGN KEY(expense_id) REFERENCES expenses(id),
       PRIMARY KEY (user_id, expense_id)
);

