-- Схема базы данных для системы с фотографами и клиентами

-- Таблица пользователей
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица альбомов
CREATE TABLE albums (
    album_id SERIAL PRIMARY KEY,
    photographer_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (photographer_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Таблица фотографий
CREATE TABLE photos (
    photo_id SERIAL PRIMARY KEY,
    album_id INT NOT NULL,
    s3_path VARCHAR(512) NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES albums(album_id) ON DELETE CASCADE
);

-- Таблица для хранения приглашений
CREATE TABLE invitations (
    invitation_id SERIAL PRIMARY KEY,
    album_id INT NOT NULL,
    client_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (album_id) REFERENCES albums(album_id) ON DELETE CASCADE,
    FOREIGN KEY (client_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (photographer_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Таблица для подтверждения выбора клиентом
CREATE TABLE photo_selections (
    selection_id SERIAL PRIMARY KEY,
    photo_id INT NOT NULL,
    client_id INT NOT NULL,
    confirmed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (photo_id) REFERENCES photos(photo_id) ON DELETE CASCADE,
    FOREIGN KEY (client_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- -- Простой триггер для обновления времени последнего изменения альбома при добавлении фотографии
-- CREATE OR REPLACE FUNCTION update_album_timestamp()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     UPDATE albums SET created_at = CURRENT_TIMESTAMP WHERE album_id = NEW.album_id;
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER update_album_timestamp_trigger
-- AFTER INSERT ON photos
-- FOR EACH ROW
-- EXECUTE FUNCTION update_album_timestamp();

-- -- Сложный триггер для проверки уникальности email перед вставкой пользователя
-- CREATE OR REPLACE FUNCTION check_unique_email()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     IF EXISTS (SELECT 1 FROM users WHERE email = NEW.email) THEN
--         RAISE EXCEPTION 'Email % already exists.', NEW.email;
--     END IF;
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER check_unique_email_trigger
-- BEFORE INSERT ON users
-- FOR EACH ROW
-- EXECUTE FUNCTION check_unique_email();

-- -- Простая функция для получения количества фотографий в альбоме
-- CREATE OR REPLACE FUNCTION get_photo_count(album_id INT)
-- RETURNS INT AS $$
-- DECLARE
--     photo_count INT;
-- BEGIN
--     SELECT COUNT(*) INTO photo_count FROM photos WHERE album_id = album_id;
--     RETURN photo_count;
-- END;
-- $$ LANGUAGE plpgsql;

-- -- Сложная функция для получения всех альбомов фотографа с количеством фотографий
-- CREATE OR REPLACE FUNCTION get_albums_with_photo_count(photographer_id INT)
-- RETURNS TABLE(album_id INT, name VARCHAR, photo_count INT) AS $$
-- BEGIN
--     RETURN QUERY
--     SELECT a.album_id, a.name, COUNT(p.photo_id) AS photo_count
--     FROM albums a
--     LEFT JOIN photos p ON a.album_id = p.album_id
--     WHERE a.photographer_id = photographer_id
--     GROUP BY a.album_id, a.name;
-- END;
-- $$ LANGUAGE plpgsql;

-- -- Простая процедура для добавления нового пользователя
-- CREATE OR REPLACE PROCEDURE add_user(username VARCHAR, email VARCHAR, password_hash VARCHAR)
-- LANGUAGE plpgsql AS $$
-- BEGIN
--     INSERT INTO users (username, email, password_hash) VALUES (username, email, password_hash);
-- END;
-- $$;

-- -- Сложная процедура для добавления фотографии и обновления времени альбома
-- CREATE OR REPLACE PROCEDURE add_photo(album_id INT, s3_path VARCHAR)
-- LANGUAGE plpgsql AS $$
-- BEGIN
--     INSERT INTO photos (album_id, s3_path) VALUES (album_id, s3_path);
--     PERFORM update_album_timestamp();
-- END;
-- $$;

-- -- Представление для получения всех фотографий с информацией об альбоме и фотографе
-- CREATE VIEW photo_details AS
-- SELECT
--     p.photo_id,
--     p.s3_path,
--     p.uploaded_at,
--     a.album_id,
--     a.name AS album_name,
--     u.user_id AS photographer_id,
--     u.username AS photographer_username
-- FROM photos p
-- JOIN albums a ON p.album_id = a.album_id
-- JOIN users u ON a.photographer_id = u.user_id;
