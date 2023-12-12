-- Create SQL table that stores all weather data upon each post to the server

CREATE TABLE IF NOT EXISTS weather_data (
    id Integer PRIMARY KEY AUTOINCREMENT,
    data TEXT NOT NULL,
);