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
    photographer_id INT NOT NULL,
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

-- Таблица для уведомлений
CREATE TABLE notifications (
    notification_id SERIAL PRIMARY KEY,
    recipient_id INT NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recipient_id) REFERENCES users(user_id) ON DELETE CASCADE
);


-- Function to mark a notification as read
CREATE FUNCTION mark_as_read(notification_id INT) 
RETURNS VOID 
AS $$
BEGIN
    UPDATE notifications SET is_read = TRUE WHERE notification_id = notification_id;
END;
$$ LANGUAGE plpgsql;


-- Trigger to update the timestamp when a notification is read
CREATE TRIGGER update_timestamp_on_read
AFTER UPDATE OF is_read ON notifications
FOR EACH ROW
WHEN (NEW.is_read = TRUE)
EXECUTE FUNCTION update_timestamp();

-- Function to update the timestamp
CREATE FUNCTION update_timestamp() 
RETURNS TRIGGER 
AS $$
BEGIN
    NEW.created_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Procedure to delete all notifications for a user
CREATE PROCEDURE delete_user_notifications(user_id INT) 
AS $$
BEGIN
    DELETE FROM notifications WHERE recipient_id = user_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get unread notifications for a user
CREATE FUNCTION get_unread_notifications(user_id INT) 
RETURNS TABLE(notification_id INT, message TEXT, created_at TIMESTAMP) 
AS $$
BEGIN
    RETURN QUERY 
        SELECT notification_id, message, created_at FROM notifications 
        WHERE recipient_id = user_id AND is_read = FALSE;
END;
$$ LANGUAGE plpgsql;


-- Function to log notification creation
CREATE FUNCTION log_notification() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO notification_logs (notification_id, action, log_time) 
    VALUES (NEW.notification_id, 'CREATED', CURRENT_TIMESTAMP);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Procedure to mark all notifications as read for a user
CREATE PROCEDURE mark_all_as_read(user_id INT) AS $$
BEGIN
    UPDATE notifications SET is_read = TRUE WHERE recipient_id = user_id;
END;
$$ LANGUAGE plpgsql;

-- Function to count unread notifications for a user
CREATE FUNCTION count_unread_notifications(user_id INT) RETURNS INT AS $$
DECLARE
    unread_count INT;
BEGIN
    SELECT COUNT(*) INTO unread_count FROM notifications 
    WHERE recipient_id = user_id AND is_read = FALSE;
    RETURN unread_count;
END;
$$ LANGUAGE plpgsql;

-- Function to add a new user
CREATE FUNCTION add_user(username TEXT, email TEXT) RETURNS VOID AS $$
BEGIN
    INSERT INTO users (username, email) VALUES (username, email);
END;
$$ LANGUAGE plpgsql;

-- Trigger to log user creation
CREATE TRIGGER log_user_creation
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION log_user();

-- Procedure to update user email
CREATE PROCEDURE update_user_email(user_id INT, new_email TEXT) AS $$
BEGIN
    UPDATE users SET email = new_email WHERE user_id = user_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get user details
CREATE FUNCTION get_user_details(user_id INT) RETURNS TABLE(username TEXT, email TEXT, created_at TIMESTAMP) AS $$
BEGIN
    RETURN QUERY SELECT username, email, created_at FROM users WHERE user_id = user_id;
END;
$$ LANGUAGE plpgsql;

-- Procedure to delete a photo
CREATE PROCEDURE delete_photo(photo_id INT) AS $$
BEGIN
    DELETE FROM photos WHERE photo_id = photo_id;
END;
$$ LANGUAGE plpgsql;

-- Function to get photos by user
CREATE FUNCTION get_photos_by_user(user_id INT) RETURNS TABLE(photo_id INT, photo_url TEXT, uploaded_at TIMESTAMP) AS $$
BEGIN
    RETURN QUERY SELECT photo_id, photo_url, uploaded_at FROM photos WHERE user_id = user_id;
END;
$$ LANGUAGE plpgsql;