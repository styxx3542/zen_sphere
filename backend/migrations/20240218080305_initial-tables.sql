-- Create the users table to store user information
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL, -- Store hashed passwords
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the posts table to store posts information
CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,  -- ID of the user who made the comment
    post_id INT NOT NULL,  -- ID of the post the comment belongs to
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts(post_id) ON DELETE CASCADE
);
